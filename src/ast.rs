use std::fs::File;
use std::error::Error;
use nix::unistd::{fork, ForkResult};
use nix::sys::wait::{ waitpid};
use std::ffi::CString;


use super::enums::{Op, Token};

#[derive(Debug)]
pub enum Ast {
    Node(Box<Option<Ast>>, Box<Option<Ast>>, Op),
    Leaf(CString, Vec<CString>),
}

pub fn eval_ast(tree: Box<Option<Ast>>) -> Option<i32> {
    match *tree {
        Some(Ast::Leaf(c, args)) => {
            match fork() {
                Ok(ForkResult::Parent {child, ..}) => { waitpid(child, None); },
                Ok(ForkResult::Child) => { nix::unistd::execvp(&c, &args); },
                Err(e)=> { println!("Fork Failed"); }
            }
            return Some(1);
        }
        _ => None

    }
}
   /* match *tree {x
        Some(Ast::Leaf(c, args)) => {
            match fork() {
                Ok(ForkResult::Parent { child, .. }) => {
                    waitpid(child, None);
                },
                Ok(ForkResult::Child) => {
                    nix::unistd::execvp(&CString::new(c), &args);
                },
                Err(e) => { println!("Fork failed") },
            }
            return Some(1);
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
        // check for redirect right
        Some(Ast::Node(left_child, right_child, Op::RedirectRight)) => {
            match *right_child {
                Some(Ast::Leaf(filename, _trash)) => {
                    let new_thread = thread::spawn(move || {
                    let mut file = match File::create(&filename) {
                        Err(why) => panic!("Couldn't open {}: {}", filename, why.description()),
                        Ok(file) => file,
                    }; 

                });

                    return eval_ast(left_child);
                }
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
}*/
