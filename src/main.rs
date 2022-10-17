mod lexer;
mod parser;

use std::io;
use io::Write;
use parser::Job;

fn main() {
    let mut buffer = String::new();

    loop {
        print!("$ ");
        io::stdout().flush()
            .expect("flush stdout");

        io::stdin().read_line(&mut buffer)
            .expect("read user input");

        let mut parser = Job::new();
        parser.parse(&buffer);

        // Clear buffer string
        // More efficient than deleting and reallocating every loop
        buffer.clear();
    }
}
/* Commands to implement
    * Executables
        Absolute, relative, or PATH
        Background execution
    * <, >, >>, 
    * pipes
    * # comments
    * echo command
    * export command
    * cd command
    * pwd command
    * quit and exit
    * jobs
    * kill

Extra credit
    * Pipes and redirects can be mixed
    * Pipes and redirects work with built-in commands
    * Append redirection

*/