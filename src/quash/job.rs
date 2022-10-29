use std::fs::OpenOptions;
use std::process;
use nix::sys::signal::{Signal, SigSet};
use nix::unistd::Pid;
use stdio_override::{StdoutOverride, StdinOverride};

use super::lexer::Dictionary;
use super::{command::Command, Quash};

#[derive(Debug)]
pub struct Job {
    pub id: i32,
    pub pid: u32,
    pub foreground: bool,
    pub cmds: Vec<Command>, // Commands separated by pipes
    pub str: String,
}

impl Job {
    pub fn new(id: i32) -> Job {
        Job { 
            id: id,
            pid: process::id(),
            foreground: true, 
            cmds: Vec::<Command>::new(), 
            str: String::new(),
        }
    }

    pub fn info(&self) -> String {
        format!("[{}]\t{}\t{}", self.id, self.pid, self.str) 
    }
}

impl Quash {
    pub fn run_job(&self, job: &Job) {
        let mut read_guard: Option<stdio_override::StdinOverrideGuard> = None;
        let mut write_guard: Option<stdio_override::StdoutOverrideGuard> = None;
        let mut append_guard: Option<stdio_override::StdoutOverrideGuard> = None;

        for cmd in &job.cmds[..] {

            if let Some(file) = &cmd.read {
                match file {
                    Dictionary::Text(txt) => {
                        println!("Read from file: {txt}");

                        OpenOptions::new()
                            .read(true)
                            .open(txt)
                            .unwrap();
                        read_guard = StdinOverride::override_file(txt).ok();
                    },
                    _ => panic!("Error getting file")
                }
            }

            if let Some(file) = &cmd.write {
                match file {
                    Dictionary::Text(txt) => {
                        println!("Write to file: {txt}");

                        OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .open(txt)
                            .unwrap();
                        write_guard = StdoutOverride::override_file(txt).ok();
                    },
                    _ => panic!("Error getting file")
                }
            }

            if let Some(file) = &cmd.append {
                match file {
                    Dictionary::Text(txt) => {
                        println!("Append to file: {txt}");

                        OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(txt)
                            .unwrap();
                        append_guard = StdoutOverride::override_file(txt).ok();
                    },
                    _ => panic!("Error getting file")
                }
            }
            
            // Always override stdout

            self.exec_cmd(&cmd);

            // Stdout => stdin fof next
        }

        drop(read_guard);
        drop(write_guard);
        drop(append_guard);

        // Println stdout
    }

    pub fn run_job_bg(&self, job: &mut Job) {
        use nix::unistd::{fork, ForkResult};

        match unsafe{fork()} {
            Ok(ForkResult::Parent { .. }) => {
                let mut sig = SigSet::empty();
                sig.add(Signal::SIGUSR1);
                sig.wait().unwrap();
            },
            Ok(ForkResult::Child) => {
                let parent = job.pid;

                job.pid = process::id();
                println!("Background job started: {}", job.info());
                self.run_job(job);

                // Notify parent that job started
                nix::sys::signal::kill(Pid::from_raw(parent.try_into().unwrap()), Signal::SIGUSR1).unwrap();

                println!("Completed: {}", job.info());
                Command::quit();
            }
            Err(_) => println!("Fork failed"),
        }
    }
}