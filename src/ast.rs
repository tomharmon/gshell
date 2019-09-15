
use std::process::{Command, Stdio};
use std::fs::File;

use super::enums::{Op};

#[derive(Debug)]
pub enum Ast {
    Node(Box<Option<Ast>>, Box<Option<Ast>>, Op),
    File(File),
    Leaf(Command),
}

// pub struct Node
// {
//     left  : Ast,
//     right : Ast,
//     operator : Operator,
// }

fn eval_ast(tree: &Ast, input: &mut Stdio, output: &mut Stdio, bg: bool) -> Option<i32> {
    match tree {
        &Ast::Leaf(mut command) => {
            command.stdin(*input);
            command.stdout(*output);
            if bg {
                command.spawn().expect("could not eval an ast leaf");
                return None;
            } else {
                return command.status().expect("could not eval an ast leaf").code();
            }
        }
        // check for semi colon
        &Ast::Node(left_child, right_child, Op::Semicolon) => {
            let left_rv = eval_ast(&left_child.unwrap(), input, output, bg);
            match *right_child {
                Some(ast) => {
                    let right_rv = eval_ast(&ast, input, output, bg);
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
        // check for background operator
        &Ast::Node(left_child, right_child, Op::Background) => {
            eval_ast(&left_child.unwrap(), input, output, true);
            match *right_child {
                Some(ast) => return eval_ast(&ast, input, output, bg),
                None => return None,
            }
        }
        // check for redirect left or right
        &Ast::Node(left_child, right_child, Op::RedirectLeft) => {
            match *right_child {
                Some(Ast::File(f)) => return eval_ast(&left_child.unwrap(), &mut Stdio::from(f), output, bg),
                _ => panic!("no file :( "),
            }
        }
        &Ast::Node(left_child, right_child, Op::RedirectRight) => {
            match *right_child {
                Some(Ast::File(f)) => return eval_ast(&left_child.unwrap(), input, &mut Stdio::from(f), bg),
                _ => panic!("no file :( "),
            }
        }
        // check for && or ||
        // &Ast::Node(left_child, right_child, Op::And) | &Ast::Node(left_child, right_child, Op::Or) => {
        //     let left_rv = eval_ast(&left_child.unwrap(), input, output, bg);
        //     match (left_rv, tree) {
        //         (Some(rv), Ast::Node(_, _, Op::And)) => {
        //             if rv == 0 {
        //                 return eval_ast(&right_child.unwrap(), input, output, bg);
        //             } else {
        //                 return left_rv;
        //             }
        //         }
        //         (Some(rv), Ast::Node(_, _, Op::Or)) => {
        //             if rv != 0 {
        //                 return eval_ast(&right_child.unwrap(), input, output, bg);
        //             } else {
        //                 return left_rv;
        //             }
        //         }
        //         (_, _) => {
        //             panic!("Should've thrown an error when parsing");
        //         }
        //     }
        // }
        // check for |
        &Ast::Node(_, _, _) => {
            return None
        }
        &Ast::File(_) => {
            return None
        }
    }
}
