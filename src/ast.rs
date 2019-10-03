use std::fs::File;
use std::process::{Command, Stdio, Child, ChildStdin};
use std::thread;

use super::enums::Op;

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
                Err(_) => panic!("not sure what to do here")
            }
        }
        None => return None
    }
}

// fn eval_child_not_option(child: &mut Child) -> Option<i32> {
//     let exit_result = child.wait();
//     match exit_result {
//         Ok(exit_status) => return exit_status.code(),
//         Err(_) => panic!("not sure what to do here")
//     }
// }

fn _eval_ast(tree: Box<Option<Ast>>, input: Stdio, output: Stdio) -> Option<Child> {
    match *tree {
        Some(Ast::Leaf(c, args)) => {
            // println!("{:?}", input);
            // println!("{:?}", output);
            let mut command = Command::new(c);
            command.args(args);
            command.stdin(input);
            command.stdout(output);
            return Some(command.spawn().expect("could not start a command"));
        }
        // check for semi colon
        Some(Ast::Node(left_tree, right_tree, Op::Semicolon)) => {
            let mut left_child_proc = _eval_ast(left_tree, input, output);
            let _left_rv = eval_child(&mut left_child_proc);
            let right_child_input;
            let right_child_output;
            println!("{:?}", left_child_proc);
            match left_child_proc {
                Some(lc) => {
                    match lc.stdin {
                        Some(x) => right_child_input = Stdio::from(x),
                        None => right_child_input = Stdio::inherit(),
                    } 
                    match lc.stdout {
                        Some(x) => { println!("SOME"); right_child_output = Stdio::from(x) },
                        None => { println!("NONE"); right_child_output = Stdio::inherit() },
                    }
                }
                None => {
                    right_child_input = Stdio::inherit();
                    right_child_output = Stdio::inherit();
                }

            }
            //Should sometimes return the left_child ... cannot rust good
            return _eval_ast(right_tree, right_child_input, right_child_output);
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
        // check for redirect right
        Some(Ast::Node(left_child, right_child, Op::RedirectRight)) => match *right_child {
            Some(Ast::Leaf(file_name, _trash)) => {
                //let thread_handle = thread::spawn(move || {
                    let file = File::create(file_name).unwrap();
                    return _eval_ast(left_child, input, Stdio::from(file));
                //});
                //return thread_handle.join().unwrap();
            }
            _ => panic!("no file :( "),
        },
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
