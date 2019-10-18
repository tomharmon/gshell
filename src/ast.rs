use std::io::{stdin, stdout};
use std::os::unix::io::AsRawFd;
use std::process::{exit, Command};
use std::thread;

use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::sys::wait::waitpid;
use nix::unistd::{close, dup, fork, pipe, ForkResult};

use std::env;
use std::path::Path;

use super::enums::Op;

#[derive(Debug)]
pub enum Ast {
    Node(Box<Option<Ast>>, Box<Option<Ast>>, Op),
    Leaf(String, Vec<String>),
}

// return Result (so we can use `?` everywhere instead of unwrap), or Option?
pub fn eval_ast(tree: Option<Ast>) -> Option<i32> {
    match tree {
        Some(Ast::Leaf(c, args)) => {
            if c == "cd" {
                let root = Path::new(&args[0]);
                env::set_current_dir(&root);
                Some(0)
            } else if c == "exit" {
                exit(1); //uhhhhh
            } else {
                let mut command = Command::new(c);
                command.args(args);
                command.status().expect("could not eval an ast leaf").code()
            }
        }
        // check for semi colon
        Some(Ast::Node(left_child, right_child, Op::Semicolon)) => {
            let left_rv = eval_ast(*left_child);
            match *right_child {
                Some(ast) => {
                    let right_rv = eval_ast(Some(ast));
                    match (left_rv, right_rv) {
                        (None, None) => None,
                        (None, x) => x,
                        (x, None) => x,
                        (Some(x), Some(y)) => {
                            if x != 0 {
                                Some(x)
                            } else {
                                Some(y)
                            }
                        }
                    }
                }
                None => left_rv,
            }
        }
        //check for background operator
        Some(Ast::Node(left_child, right_child, Op::Background)) => {
            match fork() {
                Ok(ForkResult::Parent { .. }) => {
                    match *right_child {
                        Some(_) => eval_ast(*right_child),
                        _ => None
                    }
                },
                Ok(ForkResult::Child) => {
                    eval_ast(*left_child);
                    exit(1); //uhhhh
                }
                Err(_) => None, // uhh
            }
        }
        // check for redirect in <
        Some(Ast::Node(left_child, right_child, Op::RedirectIn)) => {
            match fork() {
                Ok(ForkResult::Parent { child, .. }) => {
                    match waitpid(child, None) {
                        Ok(_) => Some(5), // uhhh
                        Err(_) => None,   // uhhhh
                    }
                }
                Ok(ForkResult::Child) => {
                    match *right_child {
                        Some(Ast::Leaf(file_name, _)) => {
                            // TODO: actual error handling for opening file, modes?
                            let file = open(file_name.as_str(), OFlag::O_RDONLY, Mode::all())
                                .expect("could not open file to read in");
                            // i think these unwraps are fine, can panic in this case
                            close(stdin().as_raw_fd()).unwrap();
                            dup(file).unwrap();
                            close(file).unwrap();
                            eval_ast(*left_child);
                            exit(1) //uhhhh
                        }
                        _ => {
                            panic!("need a file after <");
                        } // TODO: this already never happens bc of parser iirc, right
                    }
                }
                Err(_) => None, // uhh
            }
        }
        // check for redirect out >
        Some(Ast::Node(left_child, right_child, Op::RedirectOut)) => {
            match fork() {
                Ok(ForkResult::Parent { child, .. }) => {
                    match waitpid(child, None) {
                        Ok(_) => Some(5), // uhh
                        Err(_) => None,   // uhh
                    }
                }
                Ok(ForkResult::Child) => {
                    match *right_child {
                        Some(Ast::Leaf(file_name, _)) => {
                            // TODO: actual error handling for creating file, def fix modes
                            let file = open(file_name.as_str(), OFlag::O_CREAT | OFlag::O_WRONLY, Mode::all()).unwrap();
                            close(stdout().as_raw_fd()).unwrap();
                            dup(file).unwrap();
                            close(file).unwrap();
                            eval_ast(*left_child).unwrap();
                            exit(1);
                        }
                        _ => panic!("need a file after >"), // TODO: this already never happens bc of parser iirc, right
                    }
                }
                Err(_) => None, // uhh
            }
        }
        // check for &&
        Some(Ast::Node(left_child, right_child, Op::And)) => {
            let left_rv = eval_ast(*left_child);
            match left_rv {
                Some(rv) => {
                    if rv == 0 {
                        eval_ast(*right_child)
                    } else {
                        left_rv
                    }
                }
                None => eval_ast(*right_child),
            }
        }
        // check for ||
        Some(Ast::Node(left_child, right_child, Op::Or)) => {
            let left_rv = eval_ast(*left_child);
            match left_rv {
                Some(rv) => {
                    if rv != 0 {
                        eval_ast(*right_child)
                    } else {
                        left_rv
                    }
                }
                None => None,
            }
        }
        // check for |
        Some(Ast::Node(left_child, right_child, Op::Pipe)) => {
            match fork() {
                Ok(ForkResult::Parent { child, .. }) => {
                    match waitpid(child, None) {
                        Ok(_) => Some(5), // uhh
                        Err(_) => None,   // uhh
                    }
                }
                Ok(ForkResult::Child) => {
                    let p = pipe().unwrap();
                    match fork() {
                        Ok(ForkResult::Parent { child, .. }) => {
                            close(stdin().as_raw_fd()).unwrap();
                            dup(p.0).unwrap();
                            close(p.1).unwrap();
                            eval_ast(*right_child); // TODO: what to do w returned val?
                            let _ = waitpid(child, None); // TODO: what to do w return val
                            exit(1);
                        }
                        Ok(ForkResult::Child) => {
                            close(stdout().as_raw_fd()).unwrap();
                            dup(p.1).unwrap();
                            close(p.0).unwrap();
                            eval_ast(*left_child);
                            exit(1);
                        }
                        Err(_) => None, // uhh
                    }
                }
                Err(_) => None, // uhh
            }
        }
        _ => None,
    }
}
