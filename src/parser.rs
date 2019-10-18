use super::ast::Ast;
use super::enums::{Op, Token};
use super::lexer;

fn find_last_occ(op: Op, tokens: &[Token]) -> Option<usize> {
    for n in (0..tokens.len()).rev() {
        match &tokens[n] {
            Token::Operator(o) => {
                if o == &op {
                    return Some(n);
                }
            }
            _ => continue,
        }
    }
    None
}

pub fn make_ast(tokens: &[Token]) -> Result<Box<Option<Ast>>, String> {
    let operators = [
        Op::Semicolon,
        Op::Background,
        Op::And,
        Op::Or,
        Op::Pipe,
        Op::RedirectIn,
        Op::RedirectOut,
    ];

    for op in operators.iter() {
        let idx = find_last_occ(*op, tokens);
        match idx {
            Some(n) => {
                if n == tokens.len() - 1 {
                    let left_tree = make_ast(&(tokens[..n]).to_vec());
                    match left_tree {
                        Ok(tree) => return Ok(Box::new(Some(Ast::Node(tree, Box::new(None), *op)))),
                        x => return x,
                    }
                } else if n == 0 {
                    return Err(format!(
                        "Unexpected Token found {:?} at the start of an expression",
                        *op
                    ));
                } else {
                    let left_tree = make_ast(&(tokens[..n]).to_vec());
                    let right_tree = make_ast(&(tokens[n + 1..]).to_vec());
                    match (left_tree, right_tree) {
                        (Ok(l_tree), Ok(r_tree)) => return Ok(Box::new(Some(Ast::Node(l_tree, r_tree, *op)))),
                        (Err(x), _) => return Err(x),
                        (_, Err(x)) => return Err(x),
                    }
                }
            }
            None => continue,
        }
    }

    if tokens.len() == 1 {
        if let Token::Input(x) = &tokens[0] {
            if x.starts_with('(') && x.ends_with(')') {
                let new_str = &(x.as_str()[1..x.len() - 1]).to_string();
                let mut new_tokens: Vec<Token> = Vec::new();
                match lexer::tokenize(&new_str, &mut new_tokens) {
                    Ok(_) => {}
                    Err(message) => return Err(message),
                }
                return make_ast(&new_tokens);
            }
        }
    }

    let mut iter = tokens.iter();
    match iter.next() {
        Some(Token::Input(command)) => {
            let comm = String::from(command);
            let mut args = Vec::new();
            for tok in iter {
                match tok {
                    Token::Operator(_) => return Err(String::from("Parsing error, should never get this error")),
                    Token::Input(x) => {
                        args.push(String::from(x));
                    }
                }
            }
            Ok(Box::new(Some(Ast::Leaf(comm, args))))
        }
        _ => Ok(Box::new(None)),
    }
}
