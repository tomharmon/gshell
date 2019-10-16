use nix::unistd::{ForkResult, fork, close, dup, execvp };
use nix::sys::wait::{ waitpid, WaitStatus };
use nix::sys::stat::Mode;
use nix::fcntl::{open, OFlag};
use nix::NixPath;
use std::ffi::CString;
use std::fs::File;
use std::error::Error;
use std::io::{stdin, stdout};
use std::os::unix::io::AsRawFd;

use super::enums::Op;

#[derive(Debug)]
pub enum Ast {
    Node(Box<Option<Ast>>, Box<Option<Ast>>, Op),
    Leaf(CString, Vec<CString>),
}

pub fn eval_ast(tree: Box<Option<Ast>>) -> Result<WaitStatus, String> {
    match *tree {
        Some(Ast::Leaf(c, args)) => {
            match fork() {
                Ok(ForkResult::Parent {child, ..}) => {
                    match waitpid(child, None) {
                        Ok(x) => Ok(x),
                        Err(e) => Err(String::from(e.description()))
                    }
                },
                Ok(ForkResult::Child) => { 
                    execvp(&c, &args); 
                    return Err(String::from("Execution error"))
                },
                Err(e)=> Err(String::from(e.description())),
            }
        },
        Some(Ast::Node(left_child, right_child, Op::Semicolon)) => {
            let left_rv = eval_ast(left_child);
            if success(&left_rv) && right_child.is_some() {
                return eval_ast(right_child);
            }
            return left_rv;
        },
        Some(Ast::Node(left_child, right_child, Op::Background)) => {
            match fork() {
                Ok(ForkResult::Parent {child, ..}) => {
                    if right_child.is_some() {
                        return eval_ast(right_child);
                    }
                    match WaitStatus::from_raw(child, 0) {
                        Ok(ws) => Ok(ws),
                        Err(e) => Err(String::from(e.description())),
                    }
                },
                Ok(ForkResult::Child) => { 
                    eval_ast(left_child);
                    return Err(String::from("Execution error"))
                },
                Err(e)=> Err(String::from(e.description())),
            }
        },
        Some(Ast::Node(left_child, right_child, Op::And)) => {
            let left_rv = eval_ast(left_child);
            if success(&left_rv) {
                return eval_ast(right_child);
            }
            left_rv
        }
        Some(Ast::Node(left_child, right_child, Op::Or)) => {
            let left_rv = eval_ast(left_child);
            if !success(&left_rv) {
                return eval_ast(right_child);
            }
            left_rv
        }
        Some(Ast::Node(left_child, right_child, Op::RedirectLeft)) => {
            match fork() {
                Ok(ForkResult::Parent {child, ..}) => {
                    match waitpid(child, None) {
                        Ok(x) => Ok(x),
                        Err(e) => Err(String::from(e.description()))
                    }
                },
                Ok(ForkResult::Child) => {
                    match *right_child {
                        Some(Ast::Leaf(fileName, _)) => {
                            let file = open(fileName.as_c_str(), OFlag::O_RDONLY, Mode::all()).unwrap();
                            close(stdin().as_raw_fd());
                            dup(file);
                            close(file);
                            return eval_ast(left_child);
                        }
                        _ => { return Err(String::from("Expected file after <")); }
                    }
                },
                Err(e)=> Err(String::from(e.description())),
            }
        },
        Some(Ast::Node(left_child, right_child, Op::RedirectRight)) => {
            match fork() {
                Ok(ForkResult::Parent {child, ..}) => {
                    match waitpid(child, None) {
                        Ok(x) => Ok(x),
                        Err(e) => Err(String::from(e.description()))
                    }
                },
                Ok(ForkResult::Child) => {
                    match *right_child {
                        Some(Ast::Leaf(fileName, _)) => {
                            let file = open(fileName.as_c_str(), OFlag::O_CREAT | OFlag::O_WRONLY, Mode::all()).unwrap();
                            close(stdout().as_raw_fd());
                            dup(file);
                            close(file);
                            return eval_ast(left_child);
                        }
                        _ => { return Err(String::from("Expected file after <")); }
                    }
                },
                Err(e)=> Err(String::from(e.description())),
            }
        },


        _ => Err(String::from("Unknown error")),
    }
}

fn success(status: &Result<WaitStatus, String>) -> bool {
    match *status {
        Ok(s) => {
            match s {
                WaitStatus::Exited(_, code) => {
                    if code != 0 {
                        return false;
                    }
                    true
                },
                _ => false,
            }
        }
        Err(_) => false,
    }

}   
