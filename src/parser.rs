use super::ast::Ast;
use super::enums::{Op, Token};
use super::lexer;

fn find_last_occ(op: &Op, tokens: &Vec<Token>) -> Option<usize> {
    for n in (0..tokens.len()).rev() {
        match &tokens[n] {
            Token::Operator(o) => {
                if o == op {
                    return Some(n);
                }
            }
            _ => continue,
        }
    }
    return None;
}

pub fn make_ast(tokens: &Vec<Token>) -> Result<Box<Option<Ast>>, String> {
    let operators = [
        Op::Semicolon,
        Op::Background,
        Op::And,
        Op::Or,
        Op::Pipe,
        Op::RedirectLeft,
        Op::RedirectRight,
    ];

    for op in operators.iter() {
        let idx = find_last_occ(&op, tokens);
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
        match &tokens[0] {
            Token::Input(x) => {
                let s = x.to_str().unwrap();
                if s.starts_with('(') && s.ends_with(')') {
                    let mut new_str = &(s[1..s.len() - 1]).to_string();
                    let mut new_tokens: Vec<Token> = Vec::new();
                    lexer::tokenize(&mut new_str, &mut new_tokens);
                    return make_ast(&new_tokens);
                } else {
                    return Ok(Box::new(Some(Ast::Leaf(x.clone(), vec![x.clone()]))));
                }
            }
            _ => (),
        }
    }

    let mut iter = tokens.iter();
    match iter.next() {
        Some(Token::Input(command)) => {
            let mut args = vec![command.clone()];
            for tok in iter {
                match tok {
                    Token::Operator(_) => return Err(String::from("Parsing error")),
                    //recursively parse if it has parenths eg: (echo hello; cat new && ok)
                    Token::Input(x) => {
                        args.push(x.clone());
                    }
                }
            }
            return Ok(Box::new(Some(Ast::Leaf(command.clone(), args))));
        }
        _ => return Ok(Box::new(None)),
    }
}
