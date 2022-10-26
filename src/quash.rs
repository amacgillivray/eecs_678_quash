mod lexer;
mod parser;
mod command;
mod job;

use self::job::Job;

use std::io;
use io::Write;

pub struct Quash {
    jobs: Vec<Job>
}

impl Quash {
    pub fn new() -> Quash {
        return Quash {
            jobs: Vec::<Job>::new()
        }
    }

    pub fn run(self) {
        let mut i = 0;
        let mut buffer = String::new();

        loop {
            print!("$ ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buffer).unwrap();

            let mut job = Job::new(i);
            job.parse(&buffer);

            // Clear buffer string
            // More efficient than deleting and reallocating every loop
            buffer.clear();

            // Execute the job
            job.run();
            i += 1;
        }
    }


}