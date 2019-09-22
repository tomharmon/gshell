use nix::unistd::{ForkResult, fork};
use nix::sys::wait::{ waitpid, WaitStatus };
use std::ffi::CString;
use std::error::Error;

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
                    nix::unistd::execvp(&c, &args); 
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
