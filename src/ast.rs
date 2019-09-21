
use std::process::{Command, Stdio};
use std::fs::File;
use std::thread;

use super::enums::{Op, Token};

#[derive(Debug)]
pub enum Ast {
    Node(Box<Option<Ast>>, Box<Option<Ast>>, Op),
    Leaf(String, Vec<String>)
}

// pub struct Node
// {
//     left  : Ast,
//     right : Ast,
//     operator : Operator,
// }

pub fn eval_ast(tree: Box<Option<Ast>>) -> Option<i32> {
    match *tree {
        Some(Ast::Leaf(c, args)) => {
            let mut command = Command::new(c);
            command.args(args);
            command.stdin(Stdio::inherit());
            command.stdout(Stdio::inherit());
            return command.status().expect("could not eval an ast leaf").code();
        }
        // check for semi colon
        Some(Ast::Node(left_child, right_child, Op::Semicolon)) => {
            let left_rv = eval_ast(left_child);
            match *right_child {
                Some(ast) => {
                    let right_rv = eval_ast(Box::new(Some(ast)));
                    match (left_rv, right_rv) {
                        (None, None) => return None,
                        (None, x) => return x,
                        (x, None) => return x,
                        (Some(x), Some(y)) => return if x != 0 { Some(x) } else { Some(y) },
                    }
                },
                None => return left_rv,
            }
        }
        //check for background operator
        Some(Ast::Node(left_child, right_child, Op::Background)) => {
            thread::spawn(move || {
                eval_ast(left_child);
            });
            match *right_child {
                Some(ast) => return eval_ast(Box::new(Some(ast))),
                None => return None,
            }
        }
        // check for redirect left
        Some(Ast::Node(left_child, right_child, Op::RedirectLeft)) => {
            // thread::spawn(move || {
            //     std
            // });
            match *right_child {
                Some(Ast::Leaf(file, _trash)) => {

                    // thread::spawn(move || {
                    //     eval_ast(left_child);
                    // });
                    return eval_ast(left_child);
                },
                _ => panic!("no file :( "),
            }
        }
        // check for redirect right
        // Some(Ast::Node(left_child, right_child, Op::RedirectRight)) => {
        //     match *right_child {
        //         Some(Ast::File(f)) => return eval_ast(&left_child.unwrap(), input, &mut Stdio::from(f), bg),
        //         _ => panic!("no file :( "),
        //     }
        // }
        // check for && or ||
        Some(Ast::Node(left_child, right_child, Op::And)) => {
            let left_rv = eval_ast(left_child);
            match left_rv {
                Some(rv) => {
                    if rv == 0 {
                        return eval_ast(right_child);
                    } else {
                        return left_rv;
                    }
                }
                None => { 
                    return eval_ast(right_child);
                }
            }
        }
        Some(Ast::Node(left_child, right_child, Op::Or)) => {
            let left_rv = eval_ast(left_child);
            match left_rv {
                Some(rv) => {
                    if rv != 0 {
                        return eval_ast(right_child);
                    } else {
                        return left_rv;
                    }
                }
                None => {
                    return None;
                }
            }
        }
        // check for |
        // Ast::Node(_, _, _) => {
        //     return None
        // }
        // Ast::File(_) => {
        //     return None
        // }
        _ => None,
    }
}
