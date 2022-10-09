use std::io;
use io::Write;

fn main() {
    let mut buffer = String::new();

    loop {
        print!("$ ");
        io::stdout().flush()
            .expect("Unable to flush stdout");

        io::stdin().read_line(&mut buffer)
            .expect("Failed to read user input");

        if buffer == "quit\n" || buffer == "exit\n" {
            break;
        }

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