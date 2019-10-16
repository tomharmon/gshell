use nix::unistd::{ForkResult, fork, close, dup, execvp };
use nix::sys::wait::waitpid;
use nix::sys::stat::Mode;
use nix::fcntl::{open, OFlag};
use std::ffi::CString;
use std::io::stdout;
use std::os::unix::io::AsRawFd;

fn main() {
    match fork() {
        Ok(ForkResult::Parent {child, ..}) => { 
            waitpid(child, None); 
            println!("hello");
        } ,
        Ok(ForkResult::Child) => { 
            let file = open(CString::new("filename").unwrap().as_c_str(), OFlag::O_CREAT | OFlag::O_WRONLY, Mode::all()).unwrap();
            close(stdout().as_raw_fd());
            dup(file);
            close(file);
            match fork() {
                Ok(ForkResult::Parent {child, ..}) => { waitpid(child, None); },  
                Ok(ForkResult::Child) => { execvp(&CString::new("echo").unwrap(), &[CString::new("echo").unwrap(), CString::new("hi").unwrap()]); },
                Err(e) => panic!(e),
            }
        },
        Err(e)=> panic!(e)
    }
}
