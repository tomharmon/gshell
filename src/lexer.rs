use super::enums::{Op, Token};
use std::iter::Peekable;

/// Converts a shell input string into a vector of tokens.
pub fn tokenize(input: &String, tokens: &mut Vec<Token>) {
    let mut input_iter = input.chars().peekable();
    while let Some(c) = input_iter.next() {
        if c == ';' {
            tokens.push(Token::Operator(Op::Semicolon));
        } else if c == '<' {
            tokens.push(Token::Operator(Op::RedirectIn));
        } else if c == '>' {
            tokens.push(Token::Operator(Op::RedirectOut));
        } else if c == '&' {
            if input_iter.peek().unwrap_or(&' ') == &'&' {
                tokens.push(Token::Operator(Op::And));
                input_iter.next();
            } else {
                tokens.push(Token::Operator(Op::Background));
            }
        } else if c == '|' {
            if input_iter.peek().unwrap_or(&' ') == &'|' {
                tokens.push(Token::Operator(Op::Or));
                input_iter.next();
            } else {
                tokens.push(Token::Operator(Op::Pipe));
            }
        } else if c == '\'' {
            let mut s = String::new();
            keep_until(&mut s, |x| x == '\'', &mut input_iter);
            tokens.push(Token::Input(s));
        } else if c == '\"' {
            let mut s = String::new();
            keep_until(&mut s, |x| x == '\"', &mut input_iter);
            tokens.push(Token::Input(s));
        } else if c == '(' {
            let mut s = String::from("(");
            read_until_close_paren(&mut s, &mut input_iter);
            tokens.push(Token::Input(s));
        } else if !c.is_whitespace() {
            let mut s = String::from(c.to_string());
            let closure = |x: char| -> bool {
                x == '<' || x == '>' || x == '|' || x == '&' || x == ';' || x.is_whitespace() || x == '\'' || x == '\"'
            };
            keep_while_ex(&mut s, closure, &mut input_iter);
            tokens.push(Token::Input(s));
        }
    }
}

fn read_until_close_paren<T>(s: &mut String, iter: &mut Peekable<T>)
where
    T: Iterator<Item = char>,
{
    let mut starts = 1;
    let mut ends = 0;
    let mut sqmode = false;
    let mut dqmode = false;
    for c in iter {
        if sqmode && c == '\'' {
            sqmode = false;
        } else if dqmode && c == '\"' {
            dqmode = false;
        } else if !sqmode && !dqmode && c == ')' {
            ends += 1;
        } else if !sqmode && !dqmode && c == '(' {
            starts += 1;
        } else if c == '\'' {
            sqmode = true;
        } else if c == '\"' {
            dqmode = true;
        }
        s.push(c);
        if starts == ends {
            break;
        }
    }
}

/// Return `s` from start until `predicate` is true, exclusive.
fn keep_until<F, T>(s: &mut String, predicate: F, iter: &mut Peekable<T>)
where
    F: Fn(char) -> bool,
    T: Iterator<Item = char>,
{
    for c in iter {
        if predicate(c) {
            break;
        }
        s.push(c);
    }
}

/// Return `s` from start until `predicate` is true.
fn keep_while_ex<F, T>(s: &mut String, predicate: F, iter: &mut Peekable<T>)
where
    F: Fn(char) -> bool,
    T: Iterator<Item = char>,
{
    if predicate(*(iter.peek().unwrap_or(&'x'))) {
        return;
    }
    while let Some(c) = iter.next() {
        s.push(c);
        if predicate(*(iter.peek().unwrap_or(&'x'))) {
            break;
        }
    }
}
