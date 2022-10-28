use std::os::unix::prelude::AsRawFd;
use std::process;
use std::io;
use std::fs::File;
use nix::sys::signal::{Signal, SigSet};
use nix::unistd::Pid;

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
        use std::io::Write;
        // Init empty stdin
        // Init empty stdout
        let stdin = io::stdin();
        let stdout = io::stdout();

        for cmd in &job.cmds[..] {

            // if read:
                // Override stdin input
                // nix::unistd::dup2
            if cmd.read != None {
                nix::unistd::dup2(stdin.as_raw_fd(), stdout.as_raw_fd());
            }

            // if write / append
                // Override stdout with file
            if cmd.write != None {
                let mut file = File::create(&cmd.args[0]).expect("Error: Unable to open file.");
                let mut outputs = "";

                // for line :
                // file.write_all(stdout);
            } else if cmd.append != None {
                let mut file = File::create(&cmd.args[0]).expect("Error: Unable to open file.");
                // for line :
                // file.write_all(stdout);
            }


            // Always override stdout

            self.exec_cmd(&cmd);

            // Stdout => stdin fof next
            nix::unistd::dup2(stdout.as_raw_fd(), stdin.as_raw_fd());
        }

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