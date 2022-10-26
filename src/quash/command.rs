use super::lexer::Token;
use super::lexer::Dictionary;

#[derive(Debug)]
pub struct Command {
    pub keyword: Token, // Type of command,
    pub args: Vec<Token>,
    pub read: Option<Token>,
    pub write: Option<Token>,
    pub append: Option<Token>,
}

impl Command {
    pub fn new() -> Command {
        Command{
            keyword: Token { 
                str: String::new(), 
                cat: Dictionary::Error 
            },
            args: Vec::<Token>::new(),
            read: None,
            write: None,
            append: None,
        }
    }

    pub fn exec(self) {
        match self.keyword.cat {
            Dictionary::Echo => {

            },
            Dictionary::Export => {

            },
            Dictionary::CD => {

            },
            Dictionary::PWD => {

            },
            Dictionary::Kill => {

            },
            Dictionary::Jobs => {
                // print();
            },
            Dictionary::Quit => {
                Command::quit();
            },
            _ => { // Call exec function
                
            }
        }
    }
}

impl Command {
    pub fn quit() {
        use std::process;
        process::exit(0);
    }
    
    // Commands
    
    pub fn execvp(binary: &String, args: Vec<&String>) {
        use nix::unistd::execvp;
    
        // execvp(binary, &args);
    }
    
    pub fn echo() {
    
    
    }
    
    pub fn export() {
    
    }
    
    pub fn cd(path: String) {
        use nix::unistd::chdir;
        // chdir(&path);
    }
    
    pub fn pwd() {
        use nix::unistd;
    
        // return unistd::getcwd().unwrap();
    }
    
    // pub fn jobs() {
    
    // }
    
    // nix::sys::signal::kill
    pub fn kill(pid: i32) {
    
    }
    
    /* Higher level abstractions */
    
    pub fn pipe() {
    
    }
    
    pub fn redirect_io() {
    
    }
    
    pub fn background() {
    }
}