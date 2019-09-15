use std::process::Command;
use std::io::{self, Write};

use super::enums::{Op, Token};
use super::lexer;

#[derive(Debug)]
pub enum Ast {
    Node(Box<Option<Ast>>, Box<Option<Ast>>, Op),
    Leaf(Command),
}

// pub struct Node
// {
//     left  : Ast,
//     right : Ast,
//     operator : Operator,
// }

fn eval_ast(tree: Ast) -> () {
    match tree {
        Ast::Leaf(mut command) => {
            let output = command.output().expect("could eval an ast leaf");
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        },
        // check for semi colon
        Ast::Node(left_child, right_child, Op::Semicolon) => {
            eval_ast((*left_child).unwrap());
            match *right_child {
                Some(ast) => eval_ast(ast),
                None => (),
            }
        },
        // check for background operator
        // TODO: need to add a flag to eval_ast function signature or something
        // or find out how to fork in rust and call eval_ast after forking
        Ast::Node(left_child, right_child, Op::Background) => {
            eval_ast((*left_child).unwrap());
            match *right_child {
                Some(ast) => eval_ast(ast),
                None => (),
            }
        },
        // check for redirect left or right
        Ast::Node(left_child, right_child, Op::RedirectLeft) | Ast::Node(left_child, right_child, Op::RedirectRight) => {
            // TODO: either Inherit stdin/stdout/stderr for spawn or status, or create pipes for command.output()
        },
        // check for && or ||
        Ast::Node(left_child, right_child, Op::And) | Ast::Node(left_child, right_child, Op::Or) => {
            // TODO need make eval_ast return value to do && and ||
            eval_ast((*left_child).unwrap());
        },
        // check for |
        Ast::Node(left_child, right_child, Op::Pipe) => {
            // TODO
        },
    }
}