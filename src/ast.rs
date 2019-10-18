use std::error::Error;
use std::io::{stdin, stdout};
use std::os::unix::io::AsRawFd;
use std::process::{exit, Command};

use nix::fcntl::{open, OFlag};
use nix::sys::signal::Signal;
use nix::sys::stat::Mode;
use nix::sys::wait::{waitpid, WaitStatus};
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
pub fn eval_ast(tree: Option<Ast>) -> Result<i32, String> {
    match tree {
        Some(Ast::Leaf(c, args)) => {
            if c == "cd" {
                let root = Path::new(&args[0]);
                match env::set_current_dir(&root) {
                    Ok(_) => Ok(0),
                    Err(message) => Err(String::from(message.description())),
                }
            } else if c == "exit" {
                exit(0);
            } else {
                let mut command = Command::new(c.clone());
                command.args(args);
                match command.status() {
                    Ok(exit_status) => Ok(exit_status.code().unwrap_or(-1)),
                    Err(e) => Err(format!(
                        "Could not run command {}, with error message: {}",
                        c,
                        e.description()
                    )),
                }
            }
        }
        // check for semi colon
        Some(Ast::Node(left_child, right_child, Op::Semicolon)) => {
            let left_rv = eval_ast(*left_child);
            match *right_child {
                Some(ast) => {
                    let right_rv = eval_ast(Some(ast));
                    match (left_rv, right_rv) {
                        (Err(e), _) => Err(e),
                        (_, Err(e)) => Err(e),
                        (Ok(x), Ok(y)) => {
                            if x != 0 {
                                Ok(x)
                            } else {
                                Ok(y)
                            }
                        }
                    }
                }
                None => left_rv,
            }
        }
        //check for background operator
        Some(Ast::Node(left_child, right_child, Op::Background)) => match fork() {
            Ok(ForkResult::Parent { .. }) => match *right_child {
                Some(_) => eval_ast(*right_child),
                _ => Ok(0),
            },
            Ok(ForkResult::Child) => match eval_ast(*left_child) {
                Ok(status) => exit(status),
                Err(e) => {
                    println!("{}", e);
                    exit(-1)
                }
            },
            Err(e) => Err(format!("Forking failed for '&' operator : {}", e.description())),
        },
        // check for &&
        Some(Ast::Node(left_child, right_child, Op::And)) => {
            let left_rv = eval_ast(*left_child);
            match left_rv {
                Ok(rv) => {
                    if rv == 0 {
                        eval_ast(*right_child)
                    } else {
                        left_rv
                    }
                }
                Err(e) => Err(e),
            }
        }
        // check for ||
        Some(Ast::Node(left_child, right_child, Op::Or)) => {
            let left_rv = eval_ast(*left_child);
            match left_rv {
                Ok(rv) => {
                    if rv != 0 {
                        eval_ast(*right_child)
                    } else {
                        left_rv
                    }
                }
                Err(e) => Err(e),
            }
        }
        // check for redirect in <
        Some(Ast::Node(left_child, right_child, Op::RedirectIn)) => match fork() {
            Ok(ForkResult::Parent { child, .. }) => match waitpid(child, None) {
                Ok(wait_status) => Ok(eval_wait_status(wait_status)),
                Err(e) => Err(format!("Waiting failed with error message : {}", e.description())),
            },
            Ok(ForkResult::Child) => match *right_child {
                Some(Ast::Leaf(file_name, _)) => {
                    match open(file_name.as_str(), OFlag::O_RDONLY, Mode::from_bits(420).unwrap()) {
                        Ok(file) => {
                            close(stdin().as_raw_fd()).unwrap();
                            dup(file).unwrap();
                            close(file).unwrap();
                            match eval_ast(*left_child) {
                                Ok(status) => exit(status),
                                Err(_) => exit(-1),
                            }
                        }
                        Err(e) => Err(format!("Could not open file {}: {}", file_name, e.description())),
                    }
                }
                _ => panic!("Parser should have thrown error"),
            },
            Err(e) => Err(format!("Forking failed for '<' operator : {}", e.description())),
        },
        // check for redirect out >
        Some(Ast::Node(left_child, right_child, Op::RedirectOut)) => match fork() {
            Ok(ForkResult::Parent { child, .. }) => match waitpid(child, None) {
                Ok(wait_status) => Ok(eval_wait_status(wait_status)),
                Err(e) => Err(format!("Waiting failed with error message : {}", e.description())),
            },
            Ok(ForkResult::Child) => match *right_child {
                Some(Ast::Leaf(file_name, _)) => {
                    match open(
                        file_name.as_str(),
                        OFlag::O_CREAT | OFlag::O_TRUNC | OFlag::O_WRONLY,
                        Mode::from_bits(420).unwrap(),
                    ) {
                        Ok(file) => {
                            close(stdout().as_raw_fd()).unwrap();
                            dup(file).unwrap();
                            close(file).unwrap();
                            match eval_ast(*left_child) {
                                Ok(status) => exit(status),
                                Err(_) => exit(-1),
                            }
                        }
                        Err(e) => Err(format!("Could not open file {}: {}", file_name, e.description())),
                    }
                }
                _ => panic!("Parser should have thrown error"),
            },
            Err(e) => Err(format!("Forking failed for '>' operator : {}", e.description())),
        },
        // check for |
        Some(Ast::Node(left_child, right_child, Op::Pipe)) => match fork() {
            Ok(ForkResult::Parent { child, .. }) => match waitpid(child, None) {
                Ok(wait_status) => Ok(eval_wait_status(wait_status)),
                Err(e) => Err(format!("Waiting failed with error message : {}", e.description())),
            },
            Ok(ForkResult::Child) => {
                let p = pipe().unwrap();
                match fork() {
                    Ok(ForkResult::Parent { child, .. }) => {
                        close(stdin().as_raw_fd()).unwrap();
                        dup(p.0).unwrap();
                        close(p.1).unwrap();
                        let _ = eval_ast(*right_child);
                        match waitpid(child, None) {
                            Ok(wait_status) => exit(eval_wait_status(wait_status)),
                            Err(_) => exit(-1),
                        }
                    }
                    Ok(ForkResult::Child) => {
                        close(stdout().as_raw_fd()).unwrap();
                        dup(p.1).unwrap();
                        close(p.0).unwrap();
                        match eval_ast(*left_child) {
                            Ok(status) => exit(status),
                            Err(_) => exit(-1),
                        }
                    }
                    Err(e) => Err(format!("Forking failed for '|' operator : {}", e.description())),
                }
            }
            Err(e) => Err(format!("Forking failed for '|'' operator : {}", e.description())),
        },
        None => Ok(0),
    }
}

fn eval_wait_status(wait_status: WaitStatus) -> i32 {
    match wait_status {
        WaitStatus::Exited(_, code) => code,
        WaitStatus::Signaled(_, signal, _) => signal_to_int(signal),
        WaitStatus::Stopped(_, signal) => signal_to_int(signal),
        WaitStatus::Continued(_) => 0,
        WaitStatus::StillAlive => 0,
    }
}

fn signal_to_int(signal: Signal) -> i32 {
    match signal {
        Signal::SIGINT => 2,
        Signal::SIGQUIT => 3,
        Signal::SIGKILL => 9,
        _ => 0,
    }
}
