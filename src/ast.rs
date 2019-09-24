use libc;
use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::process::{Command, Stdio};
use std::thread;

use super::enums::Op;

#[derive(Debug)]
pub enum Ast {
    Node(Box<Option<Ast>>, Box<Option<Ast>>, Op),
    Leaf(String, Vec<String>),
}

pub fn eval_ast(tree: Box<Option<Ast>>, input: RawFd, output: RawFd) -> Option<i32> {
    match *tree {
        Some(Ast::Leaf(c, args)) => {
            let mut command = Command::new(c);
            command.args(args);
            unsafe {
                let stdin: Stdio = FromRawFd::from_raw_fd(libc::dup(input));
                let stdout: Stdio = FromRawFd::from_raw_fd(libc::dup(output));
                command.stdin(stdin);
                command.stdout(stdout);
            }
            return command.status().expect("could not eval an ast leaf").code();
        }
        // check for semi colon
        Some(Ast::Node(left_child, right_child, Op::Semicolon)) => {
            let left_rv = eval_ast(left_child, input, output);
            match *right_child {
                Some(ast) => {
                    let right_rv = eval_ast(Box::new(Some(ast)), input, output);
                    match (left_rv, right_rv) {
                        (None, None) => return None,
                        (None, x) => return x,
                        (x, None) => return x,
                        (Some(x), Some(y)) => return if x != 0 { Some(x) } else { Some(y) },
                    }
                }
                None => return left_rv,
            }
        }
        //check for background operator
        Some(Ast::Node(left_child, right_child, Op::Background)) => {
            thread::spawn(move || {
                eval_ast(left_child, input, output);
            });
            match *right_child {
                Some(ast) => return eval_ast(Box::new(Some(ast)), input, output),
                None => return None,
            }
        }
        // check for redirect left
        Some(Ast::Node(left_child, right_child, Op::RedirectLeft)) => {
            match *right_child {
                Some(Ast::Leaf(file_name, _trash)) => {
                    let file = File::open(file_name).unwrap();
                    return eval_ast(left_child, AsRawFd::as_raw_fd(&file), output);
                }
                _ => panic!("no file :( "),
            }
        }
        // check for redirect right
        Some(Ast::Node(left_child, right_child, Op::RedirectRight)) => match *right_child {
            Some(Ast::Leaf(file_name, _trash)) => {
                let file = File::create(file_name).unwrap();
                return eval_ast(left_child, input, AsRawFd::as_raw_fd(&file));
            }
            _ => panic!("no file :( "),
        },
        // check for && or ||
        Some(Ast::Node(left_child, right_child, Op::And)) => {
            let left_rv = eval_ast(left_child, input, output);
            match left_rv {
                Some(rv) => {
                    if rv == 0 {
                        return eval_ast(right_child, input, output);
                    } else {
                        return left_rv;
                    }
                }
                None => {
                    return eval_ast(right_child, input, output);
                }
            }
        }
        Some(Ast::Node(left_child, right_child, Op::Or)) => {
            let left_rv = eval_ast(left_child, input, output);
            match left_rv {
                Some(rv) => {
                    if rv != 0 {
                        return eval_ast(right_child, input, output);
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
        Some(Ast::Node(left_child, right_child, Op::Pipe)) => {
            let mut pipes: [i32; 2] = [input, output];
            unsafe {
                libc::pipe(pipes.as_mut_ptr());
            }
            let left_rv = eval_ast(left_child, pipes[1], output);
            match *right_child {
                Some(ast) => {
                    let right_rv = eval_ast(Box::new(Some(ast)), input, pipes[0]);
                    match (left_rv, right_rv) {
                        (None, None) => return None,
                        (None, x) => return x,
                        (x, None) => return x,
                        (Some(x), Some(y)) => return if x != 0 { Some(x) } else { Some(y) },
                    }
                }
                None => return left_rv
            }
        }
        _ => None,
    }
}
