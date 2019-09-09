use std::io::{self, Write};

fn main() {
    loop {
        print!("gshell$");
        io::stdout().flush().expect("couldn't print command prompt");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        input.trim();

        for s in input.split_ascii_whitespace() {
            println!("{}", s);
        }
    }
}

/// Tokens are the individual pieces that make up a shell command.
/// # Examples
/// ## Command
/// Commands consist of all the standard builtin bash programs. Examples of valid commands: `cd`, `pwd`, `echo`, `grep`, etc
/// ## Arg
/// Args are arguments to a given program. Eg for the command `echo hi people`, `hi` `people` would be args.
/// ## Semicolon
/// Semicolons (`;`) separate shell commands.
/// ## Pipe
/// A pipeline is a sequence of one or more commands separated by one of the control operators `|` or `|&`.
/// ## Redirect
/// `>` or `<`
/// ## Or
/// `||`
/// ## And
/// `&&`
/// ## Background
/// `&`
pub enum Token {
    COMMAND(String),
    ARG(String),
    SEMICOLON,
    PIPE,
    REDIRECT,
    OR,
    AND,
    BACKGROUND,
    EOF,
}

mod lexer {
    use super::Token;

    /// Converts a shell input into a vector of tokens.
    pub fn tokenize(input: &str) -> Vec<Token> {
        return vec![];
    }

    fn get_next_token(input: &str) -> Token {
        let trimmed_input = input.trim_start();

        

        return Token::EOF;

    }

    /// Return `s` from start until `predicate` is no longer true.
    /// Is there a better error type to use?
    fn keep_while<F>(s: &str, predicate: F) -> Result<&str, &'static str>
    where
        F: Fn(char) -> bool,
    {
        let mut i = 0;

        for c in s.chars() {
            if !predicate(c) {
                break;
            }
            i += c.len_utf8();
        }

        if i == 0 {
            return Err("No characters matched predicate");
        } else {
            return Ok(&s[..i]);
        }
    }

    fn skip_whitespace(s: &str) -> &str {
        match s.chars().position(|c| !c.is_whitespace()) {
            None => return s,
            Some(index) => return &s[index..],
        }
    }
}

mod parser {
    //use super::Token;
}

#[cfg(test)]
mod test_lexer {
    use super::*;

    #[test]
    fn test_tokenize() {}
}
