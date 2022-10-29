use std::fs::OpenOptions;
use std::process;
use nix::sys::signal::{Signal, SigSet};
use nix::unistd::Pid;
use stdio_override::{StdinOverride, StdoutOverride};
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

        let mut file_in = ".tmp_in";

        for (i, cmd) in job.cmds.iter().enumerate() {
            let mut file_out = ".tmp_out";

            let mut read_guard: Option<stdio_override::StdinOverrideGuard> = None;
            let mut write_guard: Option<stdio_override::StdoutOverrideGuard> = None;
            let mut append_guard: Option<stdio_override::StdoutOverrideGuard> = None;

            let mut read = false;
            let mut write = false;

            if let Some(file) = &cmd.read {
                match file {
                    Dictionary::Text(txt) => {
                        // println!("Read from file: {txt}");
                        file_in = txt;
                        read = true;
                    },
                    _ => panic!("Error getting file")
                }
            }

            if let Some(file) = &cmd.write {
                match file {
                    Dictionary::Text(txt) => {
                        // println!("Write to file: {txt}");

                        file_out = txt;
                        write = true;
                    },
                    _ => panic!("Error getting file")
                }
            }

            if let Some(file) = &cmd.append {
                match file {
                    Dictionary::Text(txt) => {
                        // println!("Append to file: {txt}");

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

            if (i > 0) || read {
                OpenOptions::new()
                    .read(true)
                    .open(file_in)
                    .unwrap();
                read_guard = StdinOverride::override_file(file_in).ok();
            }

            if (i < job.cmds.iter().len() - 1) || write {
                OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(file_out)
                    .unwrap();
                write_guard = StdoutOverride::override_file(file_out).ok();
            }

            self.exec_cmd(cmd);

            file_in = file_out;

            drop(read_guard);
            drop(write_guard);
            drop(append_guard);
        }
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