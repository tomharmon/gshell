/// Tokens are the individual pieces that make up a shell command.
/// # Examples
///
/// ## Operators
/// One of: `;`, `|`, `>`, `<`, `||`, `&&`, `&`.
///
/// ## Input
/// An input is either a string starting with a program name (e.g. `grep`) followed by arguments, or
/// a filename.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(Op),
    Input(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Semicolon,
    Pipe,
    RedirectIn,
    RedirectOut,
    Or,
    And,
    Background,
}
