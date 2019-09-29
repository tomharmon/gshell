use std::fs::File;
use std::mem;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::process::{Command, Stdio, Child};
use std::thread;

use super::enums::{Op, Token};

#[derive(Debug)]
pub enum Ast {
    Node(Box<Option<Ast>>, Box<Option<Ast>>, Op),
    Leaf(String, Vec<String>),
}

pub fn eval_ast(tree: Box<Option<Ast>>) -> Option<i32> {
    let mut child = _eval_ast(tree, Stdio::inherit(), Stdio::inherit());
    return eval_child(&mut child);
}

fn eval_child(child: &mut Option<Child>) -> Option<i32> {
    match child {
        Some(c) => {
            let exit_result = c.wait();
            match exit_result {
                Ok(exit_status) => return exit_status.code(),
                Err(child_error) => panic!("not sure what to do here")
            }
        }
        None => return None
    }
}

fn _eval_ast(tree: Box<Option<Ast>>, input: Stdio, output: Stdio) -> Option<Child> {
    match *tree {
        Some(Ast::Leaf(c, args)) => {
            println!("{:?}", input);
            println!("{:?}", output);
            let mut command = Command::new(c);
            command.args(args);
            command.stdin(input);
            command.stdout(output);
            return Some(command.spawn().expect("could not start a command"));
        }
        // check for semi colon
        Some(Ast::Node(left_tree, right_tree, Op::Semicolon)) => {
            let mut left_child = _eval_ast(left_tree, input, output);
            println!("{:?}", left_child);
            let left_rv = eval_child(&mut left_child);
            let left_child_unwrap = left_child.unwrap();
            println!("{:?}", left_child_unwrap);
            let right_child_input = left_child_unwrap.stdin.unwrap();
            let right_child_output = left_child_unwrap.stdout.unwrap();
            match *right_tree {
                Some(ast) => {
                    let mut right_child = _eval_ast(Box::new(Some(ast)), Stdio::from(right_child_input), Stdio::from(right_child_output));
                    let right_rv = eval_child(&mut right_child);
                    match (left_rv, right_rv) {
                        (None, None) => return None,
                        (None, x) => return right_child,
                        // should return Some(left_child_unwrap)
                        (x, None) => return None,
                        // note: if left_rv.unwrap() != 0 should return Some(left_child_unwrap)
                        (Some(_x), Some(_y)) => return if left_rv.unwrap() != 0 { right_child } else { right_child },
                    }
                }
                // should return Some(left_child_unwrap)
                None => return None,
            }
        }
        //check for background operator
        // Some(Ast::Node(left_child, right_child, Op::Background)) => {
        //     thread::spawn(move || {
        //         eval_ast(left_child, input, output);
        //     });
        //     match *right_child {
        //         Some(ast) => return eval_ast(Box::new(Some(ast)), input, output),
        //         None => return 0,
        //     }
        // }
        // // check for redirect left
        // Some(Ast::Node(left_child, right_child, Op::RedirectLeft)) => {
        //     match *right_child {
        //         Some(Ast::Leaf(file_name, _trash)) => {
        //             let thread_handle = thread::spawn(move || {
        //                 let file = File::open(file_name).unwrap();
        //                 return eval_ast(left_child, Stdio::from(file), output);
        //             });
        //             return thread_handle.join().unwrap();
        //             // return eval_ast(left_child);
        //         }
        //         _ => panic!("no file :( "),
        //     }
        // }
        // // check for redirect right
        // Some(Ast::Node(left_child, right_child, Op::RedirectRight)) => match *right_child {
        //     Some(Ast::Leaf(file_name, _trash)) => {
        //         let thread_handle = thread::spawn(move || {
        //             let file = File::create(file_name).unwrap();
        //             return eval_ast(left_child, input, Stdio::from(file));
        //         });
        //         return thread_handle.join().unwrap();
        //     }
        //     _ => panic!("no file :( "),
        // },
        // // check for && or ||
        // Some(Ast::Node(left_child, right_child, Op::And)) => {
        //     let left_rv = eval_ast(left_child, input, output);
        //     match left_rv {
        //         Some(rv) => {
        //             if rv == 0 {
        //                 return eval_ast(right_child, input, output);
        //             } else {
        //                 return left_rv;
        //             }
        //         }
        //         None => {
        //             return eval_ast(right_child, input, output);
        //         }
        //     }
        // }
        // Some(Ast::Node(left_child, right_child, Op::Or)) => {
        //     let left_rv = eval_ast(left_child, input, output);
        //     match left_rv {
        //         Some(rv) => {
        //             if rv != 0 {
        //                 return eval_ast(right_child, input, output);
        //             } else {
        //                 return left_rv;
        //             }
        //         }
        //         None => {
        //             return None;
        //         }
        //     }
        // }
        // check for |
        // Some(Ast::Node(left_child, right_child, Op::Pipe)) => {
        //     thread::spawn(move || {
        //         eval_ast(left_child, input, output);
        //         // eval_ast(right_child, output, output);
        //     });
        //     return None
        // }
        // Ast::Node(_, _, _) => {
        //     return None
        // }
        // Ast::File(_) => {
        //     return None
        // }
        _ => None,
    }
}
