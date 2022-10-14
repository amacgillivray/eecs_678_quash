use logos::Logos;
use crate::lexer::Dictionary;

struct Command {
    keyword: Token, // Type of command,
    args: Vec<Token>,
    read: Option<Token>,
    write: Option<Token>,
    append: Option<Token>,
}

impl Command {
    pub fn new() -> Command {
        Command{
            keyword: Token { 
                str: String::new(), 
                cat: Dictionary::Error 
            },
            args: Vec::<Token>::new(),
            read: None,
            write: None,
            append: None,
        }
    }
}

struct Token {
    cat: Dictionary,
    str: String,
}

pub struct Job {
    foreground: bool,
    cmds: Vec<Command>, // Commands separated by pipes
}

impl Job {
    pub fn parse(&mut self, str: &String) {
        let mut cmd = Command::new();

        let mut lex = Dictionary::lexer(&str);
        while let Some(cat) = lex.next() {
            let token = Token{
                cat: cat,
                str: lex.slice().to_string(),
            };

            if (cmd.keyword.cat == Dictionary::Error)
            && (token.cat < Dictionary::Quit) {
                cmd.keyword = token;
                continue;
            }

            match token.cat {
                Dictionary::Text => cmd.args.push(token),
                Dictionary::LANGLE => cmd.read = Some(Token{
                    cat: lex.next(),
                    str: lex.slice().to_string(),
                }),
                Dictionary::RANGLE => cmd.write = Some(Token{
                    cat: lex.next(),
                    str: lex.slice().to_string(),
                }),
                Dictionary::DRANGLE => cmd.append = Some(Token{
                    cat: lex.next(),
                    str: lex.slice().to_string(),
                }),
                Dictionary::AMPERSAND => self.foreground = false,
                _ => {
                    self.cmds.push(cmd);
                    cmd = Command::new();
                    cmd.keyword = token;
                }
            }

            self.cmds.push(cmd);
        }
    }

    pub fn run() {
        // Print something
        // Run commands
    }
}