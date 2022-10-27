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

    #[regex("echo")]  // Echos the argument to terminal
    Echo,

    #[regex("export")] // Set val of environment variable
    Export,

    #[regex("cd")] // Change directory
    CD,

    #[regex("pwd")] // Echo current directory name
    PWD,

    #[regex("kill")] // Kill a process
    Kill,

    #[regex("jobs")] // Show all active jobs in quash
    Jobs,

    #[regex("quit|exit")] // Exit
    Quit,

    // Operators

    #[regex("<")] // Take input from file
    LANGLE,

    #[regex(">")] // Write to file
    RANGLE,

    #[regex(">>")] // Append to file
    DRANGLE,

    #[token("|")] // Pipes output of one to input of next
    PIPE,

    #[regex("&")] // Run in background
    AMPERSAND,

    // Skipped / Error

    #[regex("#.+\n", logos::skip)] // Skip line
    Comment,

    #[regex("[ \n\t\r]+", logos::skip)] // Skip ws
    Whitespace,

    #[error] // Oops! Bad syntax
    Error
}

#[derive(Debug)]
pub struct Token {
    pub cat: Dictionary,
    pub str: String,
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