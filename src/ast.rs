use super::enums::{Op, Token};

fn find_last_occ(op: &Op, tokens: &Vec<Token>) -> Option<usize> {
    println!("{:?}", tokens);
    for n in (tokens.len()-1)..0 {
        match &tokens[n] {
            Token::Operator(o) => { 
                println!("{:?}", o);
                if o == op { 
                    return Some(n) 
                }
            },
            Token::CommandOrArgument(_) => continue,
        } 
    }
    return None;
}

#[derive(Debug)]
pub enum Ast {
    Node(Box<Option<Ast>>, Box<Option<Ast>>, Op),
    Leaf(String)
}

pub fn make_ast(tokens: &Vec<Token>) -> Box<Option<Ast>> {
    //println!("{:?}", tokens);
    if tokens.len() == 1 {
        let to_match = &tokens[0];   
        match to_match {
            Token::Operator(_)  => println!("Parsing error"),
            //recursively parse if it has parenths eg: (echo hello; cat new && ok)
            Token::CommandOrArgument(x) => return Box::new(Some(Ast::Leaf(x.to_string()))),
        }
    }

    let operators = [
        Op::Semicolon, 
        Op::Background, 
        Op::And, 
        Op::Or, 
        Op::Pipe, 
        Op::RedirectLeft, 
        Op::RedirectRight ];
           
    for op in operators.iter() {
        let idx = find_last_occ(&op, tokens);
        match idx {
            Some(n) => {
                println!("{}", n);
                if n == tokens.len() - 1 {
                    return Box::new(Some(Ast::Node(make_ast(&(tokens[..n]).to_vec()), Box::new(None), *op)));
                } else if n == 0 {
                    println!("Unexpected Token found {:?} at the start of an expression", *op);
                } else {
                    return Box::new(Some(Ast::Node(make_ast(&(tokens[..n]).to_vec()), make_ast(&(tokens[n+1..]).to_vec()), *op)));
                }
            },
            None => continue,
        }
    }
    return Box::new(None)
}

// pub struct Node 
// {
//     left  : Ast,
//     right : Ast,
//     operator : Operator,
// }
