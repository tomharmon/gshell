use std::io::{self, Write};
use std::os::unix::io::{AsRawFd};


mod ast;
mod enums;
mod lexer;
mod parser;

fn main() {
    loop {
        print!("gshell$");
        io::stdout().flush().expect("couldn't print command prompt");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        let mut tokens: Vec<enums::Token> = Vec::new();

        lexer::tokenize(&mut input, &mut tokens);

        let ast = parser::make_ast(&tokens);

        println!("{:?}", ast);
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        match ast {
            Ok(ast) => {
                ast::eval_ast(ast, AsRawFd::as_raw_fd(&handle), AsRawFd::as_raw_fd(&handle));
            }
            Err(message) => {
                println!("{}", message);
            }
        }
        // print!("gshell$");
        handle.write_all(b"hello world").expect("fail");
        // io::stdout().write_all(b"hello world").expect("fail");
        // io::stdout().flush().expect("couldn't print command prompt");
    }
}
