use std::ffi::CString;
use std::path::PathBuf;

use nix::sys::signal::Signal;

use super::Quash;
use super::lexer::Dictionary;

#[derive(Debug)]
pub struct Command {
    pub keyword: Option<Dictionary>, // Type of command,
    pub args: Vec<String>,
    pub read: Option<Dictionary>,
    pub write: Option<Dictionary>,
    pub append: Option<Dictionary>,
}

impl Quash {

    pub fn exec_cmd(self, cmd: Command) {
        if let Some(ref key) = cmd.keyword {
            match key {
                Dictionary::Echo(_) => {
                    cmd.echo();
                },
                Dictionary::Export(_) => {
                    cmd.export();
                },
                Dictionary::CD(_) => {
                    cmd.cd();
                },
                Dictionary::PWD(_) => {
                    cmd.pwd();
                },
                Dictionary::Kill(_) => {
                    cmd.kill();
                },
                Dictionary::Jobs(_) => {
                    self.print();
                },
                Dictionary::Quit(_) => {
                    Command::quit();
                },
                _ => { // Call exec function
                    cmd.execvp();
                }
            }
        }
    }
}

impl Command {
    pub fn new() -> Command {
        Command{
            keyword: None,
            args: Vec::<String>::new(),
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

        let filename: Option<CString>;
        if let Some(s) = self.keyword {
            match s {
                Dictionary::Text(binary) => filename = CString::new(binary).ok(),
                _ => filename = None,
            }
        }

        let args: Vec<CString> = self.args.iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();
    
        if let Some(binary) = filename {
            execvp(&binary, &args[..]);
        }
    }
    
    pub fn echo(self) {
        for txt in &self.args {
            println!("{}", txt);
        }
    }
    
    pub fn export(self) {
        use logos::Logos;
        use super::lexer::SetVar; 
        use std::env;

        let mut key: Option<&str> = None;
        let mut val: Option<&str>;

        let mut lex = SetVar::lexer(&self.args[0]);
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
        let path: &str = &self.args[0];

        use nix::unistd::chdir;
        chdir(path);
    }
    
    pub fn pwd(self) -> PathBuf {
        use std::env;
    
        env::current_dir().unwrap()
    }
    
    pub fn kill(self) {
        let pid: Pid = Pid::from_raw(
            self.args[0].parse().unwrap());
        let sig: Signal = self.args[1].parse().unwrap();

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