use std::io::{self, Write};
use std::os::unix::io::AsRawFd;

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

        match ast {
            Ok(ast) => {
                ast::eval_ast(ast, AsRawFd::as_raw_fd(&io::stdin()), AsRawFd::as_raw_fd(&io::stdout()));
            }
            Err(message) => {
                println!("{}", message);
            }
        }
        //break;

        // let socket = match UnixStream::connect("/dev/tty") {
        //     Ok(sock) => sock,
        //     Err(e) => {
        //         println!("Couldn't connect: {:?}", e);
        //         return
        //     }
        // };

        // print!("gshell$");
        // handle.write_all(b"hello world").expect("fail");
        // io::stdout().write_all(b"hello world").expect("fail");
        // io::stdout().flush().expect("couldn't print command prompt");
    }
}
