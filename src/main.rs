use std::io::{self, Write, Read};
use std::os::unix::io::AsRawFd;
use std::fs::File;
use std::env;

mod ast;
mod enums;
mod lexer;
mod parser;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // // match args.get(1) {
    // //     Some(s) => {
    // //         let mut file = File::open(s).unwrap();
    // //         let mut input = String::new();
    // //         file.read_to_string(&mut input);
    // //         let mut tokens: Vec<enums::Token> = Vec::new();
    // //         lexer::tokenize(&mut input, &mut tokens);

    // //         let ast = parser::make_ast(&tokens);
    // //         match ast {
    // //             Ok(ast) => {
    // //                 ast::eval_ast(ast, AsRawFd::as_raw_fd(&io::stdin()), AsRawFd::as_raw_fd(&io::stdout()));
    // //             }
    // //             Err(message) => {
    // //                 println!("{}", message);
    // //             }
    // //         }
    // //         return;
    // //     }
    // //     _ => {}
    // // }

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
