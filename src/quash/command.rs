use std::ffi::CString;
use std::io::Write;
use std::process::Command as cmd;
use std::path::PathBuf;
use std::fs::OpenOptions;
use stdio_override::{StdoutOverride, StdinOverride};
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

    pub fn exec_cmd(&self, cmd: &Command) {

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
    
    pub fn execvp(&self) {
        use nix::unistd::execvp;
        use nix::{sys::wait::waitpid,unistd::{fork, ForkResult}};
        
        let mut filename: Option<CString> = None;
        // let mut filename: String = String::new();
        if let Some(s) = &self.keyword {
            match s {
                Dictionary::Text(binary) => {
                    filename = CString::new(binary.clone()).ok()
                    // filename = binary.to_string();
                },
                _ => (),
            }
        }

        let args: Vec<CString> = self.args.iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();

        match unsafe{fork()} {
            Ok(ForkResult::Parent { child, .. }) => {
                waitpid(child, None).unwrap();
            }
            Ok(ForkResult::Child) => {
                if let Some(binary) = filename {
                    execvp(&binary, &args[..]).unwrap();
                }
            }
            Err(_) => println!("Fork failed"),
        }

        // let res = cmd::new(filename)
        //     .args(&self.args[..])
        //     .output()
        //     .expect("failed to execute process");

        // std::io::stdout().write_all(&res.stdout).unwrap();

    }
    
    pub fn echo(&self) {
        for txt in &self.args {
            println!("{}", txt);
        }
    }
    
    pub fn export(&self) {
        use logos::Logos;
        use super::lexer::SetVar; 
        use std::env;

        let mut key: Option<&str>;
        let mut val: Option<&str> = None;

        let mut lex = SetVar::lexer(&self.args[0]);
        while let Some(_) = lex.next() {
            key = val;
            val = Some(lex.slice());

            match (key, val) {
                (Some(k), Some(v)) => {
                    env::set_var(k, v);
                    println!("set value {k}: {v}");
                },
                _ => (),
            }
        }
    }
    
    pub fn cd(&self) {
        let path: &str = &self.args[0];

        use nix::unistd::chdir;
        chdir(path).unwrap();
    }
    
    pub fn pwd(&self) {
        use std::env;
    
        let path: PathBuf = env::current_dir().unwrap();
        println!("{}", path.display());        
    }
    
    pub fn kill(&self) {
        let pid: Pid = Pid::from_raw(
            self.args[0].parse().unwrap());
        let sig: Signal = self.args[1].parse().unwrap();

        use nix::unistd::Pid;
        nix::sys::signal::kill(pid, sig).unwrap();
    }
}