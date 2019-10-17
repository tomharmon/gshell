use std::env;
use std::io::{self, Write, stdin};
use std::os::unix::io::AsRawFd;

use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, dup};

mod ast;
mod enums;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut should_prompt = true;
    match args.get(1) {
        Some(s) => {
            let file = open(s.as_str(), OFlag::O_RDONLY, Mode::all())
                                .expect("could not open file to read in");
            close(stdin().as_raw_fd()).unwrap();
            dup(file).unwrap();
            close(file).unwrap();
            should_prompt = false;
        }
        _ => (),
    }

    loop {
        if should_prompt {
            print!("nush$");
            io::stdout().flush().expect("couldn't print command prompt");
        }
        let mut input = String::new();
        let bytes = io::stdin().read_line(&mut input).expect("failed to read line");
        if bytes == 0 { break; }
        let mut tokens: Vec<enums::Token> = Vec::new();

        lexer::tokenize(&mut input, &mut tokens);

        let ast = parser::make_ast(&tokens);

        match ast {
            Ok(ast) => {
                ast::eval_ast(*ast);
            }
            Err(message) => {
                println!("{}", message);
            }
        }
    }
}
