use std::io::{self, Write};

mod lexer;
mod enums;
mod ast;

fn main() {
    loop {
        print!("gshell$");
        io::stdout().flush().expect("couldn't print command prompt");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let mut tokens: Vec<enums::Token> = Vec::new();

        lexer::tokenize(&mut input, &mut tokens);

        let ast = ast::make_ast(&tokens);
        println!("{:?}", ast);
    }
}

