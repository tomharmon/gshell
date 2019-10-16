use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

mod ast;
mod enums;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(s) => {
            let open_file_result = File::open(s);
            match open_file_result {
                Ok(mut file) => {
                    let mut input = String::new();
                    let _ = file.read_to_string(&mut input).unwrap();
                    let mut tokens: Vec<enums::Token> = Vec::new();
                    lexer::tokenize(&mut input, &mut tokens);
                    let ast = parser::make_ast(&tokens);
                    match ast {
                        Ok(ast) => {
                            ast::eval_ast(ast);
                        }
                        Err(message) => {
                            println!("{}", message);
                        }
                    }
                }
                Err(_) => println!("Could not open file: {}", s),
            }
            return;
        }
        _ => (),
    }

    loop {
        print!("gshell$");
        io::stdout().flush().expect("couldn't print command prompt");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let mut tokens: Vec<enums::Token> = Vec::new();

        lexer::tokenize(&mut input, &mut tokens);

        let ast = parser::make_ast(&tokens);

        match ast {
            Ok(ast) => {
                ast::eval_ast(ast);
            }
            Err(message) => {
                println!("{}", message);
            }
        }
    }
}
