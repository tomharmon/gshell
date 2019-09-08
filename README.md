# GSHELL

gshell is a bash shell written in rust.

Given a program to execute, like "head" above, your shell will do three things:

fork(), to create a new process.
in the child process, exec(), to execute the requested program, passing through the three command line arguments
in the parent process, wait() or waitpid(), to allow the program to finish before executing another command


Redirect input: sort < foo.txt
Redirect output: sort foo.txt > output.txt
Pipe: sort foo.txt | uniq
Background: sleep 10 &
And: true && echo one
Or: true || echo one
Semicolon: echo one; echo two

Planned Features:
- AST Parser
- Supports forking and executing programs
- Supports operators such as `&&`,  `&`, `||`, `|`, `>`, `<`, etc
- Tab autocomplete
- Solarized dark theme
