use logos::Logos;
use crate::lexer::Dictionary;

struct Command {
    keyword: Token, // Type of command,
    args: Vec<Token>,
    read: Option<Token>,
    write: Option<Token>,
    append: Option<Token>,
}

struct Token {
    str: String,
    cat: Dictionary,
}

pub struct Job {
    foreground: bool,
    cmds: Vec<Command>, // Commands separated by pipes
}

impl Job {
    pub fn parse(str: &String) {
        let mut lex = Dictionary::lexer(&str);
        while let Some(cat) = lex.next() {
            let token = Token{
                str: lex.slice().to_string(), 
                cat: cat
            };

            // Constructs job
            println!("{}", token.str);
        }
    }

    pub fn run() {
        // Print something
        // Run commands
    }
}