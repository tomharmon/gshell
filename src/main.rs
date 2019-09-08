use std::io::{self, Write};

fn main() {
    loop {
        print!("gshell$");
        io::stdout().flush().expect("couldn't print command prompt");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        for s in input.split_ascii_whitespace() {
            println!("{}", s);
        }
    }
}
