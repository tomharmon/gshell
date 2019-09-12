/// Tokens are the individual pieces that make up a shell command.
/// # Examples
/// ## Command
/// Commands consist of all the standard builtin bash programs. Examples of valid commands: `cd`, `pwd`, `echo`, `grep`, etc
#[derive(Debug, Clone)]
pub enum Token {
    Operator(Op),
    CommandOrArgument(String),
}

// pub enum CommandOrArgument {
//     Command(String),
//     Argument(String)
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Semicolon,
    Pipe,
    RedirectLeft,
    RedirectRight,
    Or,
    And,
    Background,
}