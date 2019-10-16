use std::fs::File;
use std::os::unix::io::{AsRawFd};
use std::io::{stdin, stdout};
use std::process::{Command, exit};
use std::thread;


use nix::unistd::{ForkResult, fork, close, dup, pipe};
use nix::sys::wait::{ waitpid, WaitStatus };
use nix::sys::stat::Mode;
use nix::fcntl::{open, OFlag};

use super::enums::Op;

#[derive(Debug)]
pub enum Ast {
    Node(Box<Option<Ast>>, Box<Option<Ast>>, Op),
    Leaf(String, Vec<String>),
}

pub fn eval_ast(tree: Box<Option<Ast>>) -> Option<i32> {
    match *tree {
        Some(Ast::Leaf(c, args)) => {
            let mut command = Command::new(c);
            command.args(args);
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
                }
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
        // check for redirect in <
        Some(Ast::Node(left_child, right_child, Op::RedirectIn)) => {
            match fork() {
                Ok(ForkResult::Parent {child, ..}) => {
                    match waitpid(child, None) {
                        Ok(x) => Some(5), // uhhh
                        Err(e) => None // uhhhh
                    }
                },
                Ok(ForkResult::Child) => {
                    match *right_child {
                        Some(Ast::Leaf(fileName, _)) => {
                            let file = open(fileName.as_str(), OFlag::O_RDONLY, Mode::all()).unwrap();
                            close(stdin().as_raw_fd());
                            dup(file);
                            close(file);
                            eval_ast(left_child);
                            exit(1)
                        }
                        _ => { panic!("need a file after <"); }
                    }
                },
                Err(e)=> None // uhh
            }
        }
        // check for redirect out >
        Some(Ast::Node(left_child, right_child, Op::RedirectOut)) => {
            match fork() {
                Ok(ForkResult::Parent {child, ..}) => {
                    match waitpid(child, None) {
                        Ok(x) => {
                            return Some(5); // uhh
                        },
                        Err(e) => None // uhh
                    }
                },
                Ok(ForkResult::Child) => {
                    match *right_child {
                        Some(Ast::Leaf(fileName, _)) => {
                            let file = open(fileName.as_str(), OFlag::O_CREAT | OFlag::O_WRONLY, Mode::all()).unwrap();
                            close(stdout().as_raw_fd());
                            dup(file);
                            close(file);
                            eval_ast(left_child);
                            exit(1);
                        }
                        _ => { panic!("need a file after >") }
                    }
                },
                Err(e)=> None // uhh
            }
        },
        // check for &&
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
        // check for ||
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
        Some(Ast::Node(left_child, right_child, Op::Pipe)) => {
            match fork() {
                Ok(ForkResult::Parent {child, ..}) => {
                    match waitpid(child, None) {
                        Ok(x) => Some(5), // uhh
                        Err(e) => None // uhh
                    }
                },
                Ok(ForkResult::Child) => {
                   let p = pipe().unwrap();
                   match fork() {
                        Ok(ForkResult::Parent {child, ..}) => { 
                            close(stdin().as_raw_fd());
                            dup(p.0);
                            close(p.1);
                            eval_ast(right_child);
                            waitpid(child, None);
                            exit(1);
                        },
                        Ok(ForkResult::Child) => { 
                            close(stdout().as_raw_fd());
                            dup(p.1);
                            close(p.0);
                            eval_ast(left_child);
                            exit(1);
                        },
                        Err(e)=> None // uhh 
                   }

                },
                Err(e)=> None // uhh
            }
        }
        _ => None,
    }
}
