use std::io::{self, Write};

mod lexer;
mod enums;

fn main() {
    loop {
        print!("gshell$");
        io::stdout().flush().expect("couldn't print command prompt");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let tokens: Vec<enums::Token> = Vec::new();

        lexer::tokenize(&input, tokens);

        // for s in input.split_ascii_whitespace() {
        //     println!("{}", s);
        // }
    }
}

