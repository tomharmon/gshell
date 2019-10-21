use std::env;
use std::io::stdin;
use std::os::unix::io::AsRawFd;

use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, dup};

use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::completion::FilenameCompleter;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::hint::HistoryHinter;

mod ast;
mod enums;
mod lexer;
mod parser;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(s) => {
            let file = open(s.as_str(), OFlag::O_RDONLY, Mode::all()).expect("could not open file to read in");
            close(stdin().as_raw_fd()).unwrap();
            dup(file).unwrap();
            close(file).unwrap();
        }
        _ => (),
    }


    let h = utils::MyHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        colored_prompt: "gshell$".to_owned(),
    };

    let mut rl = Editor::<utils::MyHelper>::new();
    rl.set_helper(Some(h));

    loop {
        let readline = rl.readline("gshell$");
        match readline {
            Ok(mut line) => {
                rl.add_history_entry(line.as_str());

                let mut tokens: Vec<enums::Token> = Vec::new();
                match lexer::tokenize(&mut line, &mut tokens) {
                    Ok(_) => {}
                    Err(message) => {
                        println!("{}", message);
                        continue;
                    }
                }

                let ast = parser::make_ast(&tokens);

                match ast {
                    Ok(ast) => match ast::eval_ast(*ast) {
                        Ok(_) => {}
                        Err(e) => println!("{}", e),
                    },
                    Err(message) => {
                        println!("{}", message);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => { break; }
            Err(ReadlineError::Eof) => { break; }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
