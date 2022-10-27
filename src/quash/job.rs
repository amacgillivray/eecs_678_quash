use super::{command::Command, Quash};

#[derive(Debug)]
pub struct Job {
    pub id: i32,
    pub pid: i32,
    pub foreground: bool,
    pub cmds: Vec<Command>, // Commands separated by pipes
    pub str: String,
}

impl Job {
    pub fn new(id: i32) -> Job {
        Job { 
            id: id,
            pid: -1,
            foreground: true, 
            cmds: Vec::<Command>::new(), 
            str: String::new(),
        }
    }

    pub fn info(self) -> String {
        format!("[{}]\t{}\t{}", self.id, self.pid, self.str) 
    }
}

impl Quash {
    pub fn run_job(&self, job: Job) {
        
        // Init empty stdin
        // Init empty stdout

        for cmd in job.cmds {

            // if read:
                // Override stdin input
                // nix::unistd::dup2

            // if write / append
                // Override stdout with file
            
            // Always override stdout
            
            self.exec_cmd(cmd);

            // Stdout => stdin fof next
        }

        // Println stdout
    }

    pub fn run_job_bg(&self, job: &Job) {
        use nix::unistd::{fork, ForkResult};

        // TODO: set job pid

        match unsafe{fork()} {
            Ok(ForkResult::Parent { child, .. }) => {},
            Ok(ForkResult::Child) => {
                println!("Background job started: {}", job.info());
                self.run_job(job);
                Command::quit();
                println!("Completed: {}", job.info());
            }
            Err(_) => println!("Fork failed"),
         }
    }
}