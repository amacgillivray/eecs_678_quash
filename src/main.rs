mod lexer;

use std::io;
use io::Write;
use lexer::Token;
use logos::Logos;

fn main() {
    let mut buffer = String::new();

    loop {
        print!("$ ");
        io::stdout().flush()
            .expect("flush stdout");

        io::stdin().read_line(&mut buffer)
            .expect("read user input");

        // Perform lexical analysis of input string
        let mut lex = Token::lexer(&buffer);
        while let Some(_) = lex.next() {
            println!("{}", lex.slice())
        }

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