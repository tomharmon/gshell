use std::env;
use std::io::{self, stdin, Read, Write};
use std::os::unix::io::AsRawFd;

use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, dup};

mod ast;
mod enums;
mod lexer;
mod parser;

fn read_line_smart(buff: &mut String) -> u8 {
    let mut prev: u8 = 0;
    let mut length = 0;
    for c in io::stdin().bytes() {
        match c {
            Ok(ch) => {
                buff.push(ch as char);
                length += 1;
                if ch == '\n' as u8 {
                    if prev != '\\' as u8 {
                        break;
                    } else {
                        buff.pop();
                        buff.pop();
                    }
                }
                prev = ch;
            }
            Err(e) => panic!("Cannot read, Am blind {}", e),
        }
    }
    length
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut should_prompt = true;
    match args.get(1) {
        Some(s) => {
            let file = open(s.as_str(), OFlag::O_RDONLY, Mode::all()).expect("could not open file to read in");
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
        let bytes = read_line_smart(&mut input);
        if bytes == 0 {
            break;
        }
        let mut tokens: Vec<enums::Token> = Vec::new();

        match lexer::tokenize(&mut input, &mut tokens) {
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
}
