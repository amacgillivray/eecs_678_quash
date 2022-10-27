use logos::{Logos, Lexer};

#[derive(Logos, PartialEq, PartialOrd, Debug)]
pub enum Environment {
    #[regex("[$][^ \n\r\t$]+")] // arguments, or executable
    Variable,

    #[regex("[^ \n\r\t$]+")] // arguments, or executable
    Text,

    #[error] // Oops! Bad syntax
    Error

}

fn get_env(lex: &mut Lexer<Dictionary>) -> Option<String> {
    let slice = lex.slice();

    let mut lex = Environment::lexer(slice);
    let mut output = String::new();

    while let Some(cat) = lex.next() {
        match cat {
            Environment::Variable => {
                use std::env;

                let slice = lex.slice();
                let key = &slice[1..]; // Remove '$' char

                match env::var(key) {
                    Ok(val) => output += &val,
                    Err(e) => println!("couldn't interpret {key}: {e}"),
                }
            },
            _ => output += lex.slice()
        }
    }

    Some(output)
}

#[derive(Logos, PartialEq, PartialOrd, Debug)]
pub enum Dictionary {

    // Text

    // #[regex("[^ \n\r\t$]+")] // arguments, or executable
    #[regex("[^ \n\r\t]+", get_env)] // Text with '$' needs to be filtered for environment variables.
    Text(String),

    // Commands

    #[regex("echo", 
        |lex| lex.slice().to_string())]  // Echos the argument to terminal
    Echo(String),

    #[regex("export", 
        |lex| lex.slice().to_string())] // Set val of environment variable
    Export(String),

    #[regex("cd", 
        |lex| lex.slice().to_string())] // Change directory
    CD(String),

    #[regex("pwd", 
        |lex| lex.slice().to_string())] // Echo current directory name
    PWD(String),

    #[regex("kill", 
        |lex| lex.slice().to_string())] // Kill a process
    Kill(String),

    #[regex("jobs", 
        |lex| lex.slice().to_string())] // Show all active jobs in quash
    Jobs(String),

    #[regex("quit|exit", 
        |lex| lex.slice().to_string())] // Exit
    Quit(String),

    // Operators

    #[regex("<", 
        |lex| lex.slice().to_string())] // Take input from file
    LANGLE(String),

    #[regex(">", 
        |lex| lex.slice().to_string())] // Write to file
    RANGLE(String),

    #[regex(">>", 
        |lex| lex.slice().to_string())] // Append to file
    DRANGLE(String),

    #[token("|", 
        |lex| lex.slice().to_string())] // Pipes output of one to input of next
    PIPE(String),

    #[regex("&", 
        |lex| lex.slice().to_string())] // Run in background
    AMPERSAND(String),

    // Skipped / Error

    #[regex("#.+\n", logos::skip)] // Skip line
    Comment,

    #[regex("[ \n\t\r]+", logos::skip)] // Skip ws
    Whitespace,

    #[error] // Oops! Bad syntax
    Error
}

#[derive(Logos, PartialEq, PartialOrd, Debug)]
pub enum SetVar {
    #[regex("[^ =\n\r\t]+")] // arguments, or executable
    Text,

    #[regex("[=]", logos::skip)] // assignment op
    EQUAL,

    #[error] // Oops! Bad syntax
    Error

}