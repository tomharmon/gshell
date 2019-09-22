use nix::unistd::{ForkResult, fork};
use nix::sys::wait::{ waitpid, WaitStatus };
use std::ffi::CString;
use std::error::Error;

use super::enums::{Op, Token};

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
        _ => Err(String::from("Unknown error")),
    }
}
