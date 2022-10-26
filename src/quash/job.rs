use super::command::Command;

#[derive(Debug)]
pub struct Job {
    pub id: i32,
    pub foreground: bool,
    pub cmds: Vec<Command>, // Commands separated by pipes
}

impl Job {
    pub fn new(id: i32) -> Job {
        Job { 
            id: id,
            foreground: true, 
            cmds: Vec::<Command>::new() 
        }
    }

    pub fn run(self) {
        /* Steps for running
            1. Create process
            2. Handle input?
            3. Exec command
            4. Handle output?
            5. Background?
        
        */
        


        for cmd in self.cmds {
            cmd.exec();
        }
    }

    pub fn print(self) {
        println!("{:?}", self);
    }
}