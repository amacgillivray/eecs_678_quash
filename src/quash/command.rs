use std::ffi::CString;
use std::path::PathBuf;

use nix::sys::signal::Signal;

use super::Quash;
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

impl Quash {

    pub fn exec_cmd(self, cmd: Command) {
        match cmd.keyword.cat {
            Dictionary::Echo => {
                cmd.echo();
            },
            Dictionary::Export => {
                cmd.export();
            },
            Dictionary::CD => {
                cmd.cd();
            },
            Dictionary::PWD => {
                cmd.pwd();
            },
            Dictionary::Kill => {
                cmd.kill();
            },
            Dictionary::Jobs => {
                self.print();
            },
            Dictionary::Quit => {
                Command::quit();
            },
            _ => { // Call exec function
                cmd.execvp();
            }
        }
    }
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

    pub fn quit() {
        use std::process;
        process::exit(0);
    }
    
    // Commands
    
    pub fn execvp(self) {
        use nix::unistd::execvp;
    
        // execvp(&CString::new(binary), &args[..]);
        // TODO
    }
    
    pub fn echo(self) {
        for txt in &self.args {
            println!("{}", txt.str);
        }
    }
    
    pub fn export(self) {
        use logos::Logos;
        use super::lexer::SetVar; 
        use std::env;

        let mut key: Option<&str> = None;
        let mut val: Option<&str>;

        let mut lex = SetVar::lexer(&self.args[0].str);
        while let Some(_) = lex.next() {
            val = key;
            key = Some(lex.slice());

            match (key, val) {
                (Some(k), Some(v)) => {
                    env::set_var(k, v);
                },
                _ => (),
            }
        }
    }
    
    pub fn cd(self) {
        let path: &str = &self.args[0].str;

        use nix::unistd::chdir;
        chdir(path);
    }
    
    pub fn pwd(self) -> PathBuf {
        use std::env;
    
        env::current_dir().unwrap()
    }
    
    pub fn kill(self) {
        let pid: Pid = Pid::from_raw(
            self.args[0].str.parse().unwrap());
        let sig: Signal = self.args[1].str.parse().unwrap();

        use nix::unistd::Pid;
        nix::sys::signal::kill(pid, sig);
    }
    
    /* Higher level abstractions */
    
    pub fn pipe() {
    
    }
    
    pub fn redirect_io() {
    
    }
    
    pub fn background() {
    }
}