mod lexer;

use std::io;
use io::Write;
use lexer::Dictionary;
// use lexer::Token;
use logos::{Logos};
use lexer::Command;

fn main() {
    let mut buffer = String::new();

    loop {
        let mut cmd : Command = Command::new();

        print!("$ ");
        io::stdout().flush()
            .expect("flush stdout");

        io::stdin().read_line(&mut buffer)
            .expect("read user input");

        // Perform lexical analysis of input string
        let mut lex = Dictionary::lexer(&buffer);
        // let mut token_type : Dictionary = Dictionary::Text;
        // while let Some(_) = lex.next() {
        //     // println!("{}", lex.slice())
        //     cmd.add_token(lex.slice(), lex.next());
        // }
        // let mut itr;
        // while let end = (itr = lex.next())
        // {            
        // }
        let mut rdr = Dictionary::lexer(&buffer);
        while let Some(_) = lex.next()
        {
            cmd.add_token(lex.slice(), rdr.next());
        }
            
        cmd.execute();

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