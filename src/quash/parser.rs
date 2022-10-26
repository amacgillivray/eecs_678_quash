use super::command::Command;
use super::lexer::Token;
use super::Job;

use logos::Logos;
use super::lexer::Dictionary;

impl Job {
    pub fn parse(&mut self, str: &String) {
        self.cmds.push(Command::new());
        let mut cmd: &mut Command = self.cmds.last_mut().unwrap();

        let mut lex = Dictionary::lexer(&str);
        while let Some(cat) = lex.next() {
            // Create token object from lexer
            let token = Token{
                cat: cat,
                str: lex.slice().to_string(),
            };

            // Expecting a new command
            if (cmd.keyword.cat == Dictionary::Error)
            && (token.cat < Dictionary::Quit) {
                cmd.keyword = token;
                continue;
            }

            // Expecting arguments / operators to the command
            match token.cat {
                Dictionary::Text => cmd.args.push(token),
                Dictionary::LANGLE => cmd.read = Some(Token{
                    cat: lex.next().unwrap(),
                    str: lex.slice().to_string(),
                }),
                Dictionary::RANGLE => cmd.write = Some(Token{
                    cat: lex.next().unwrap(),
                    str: lex.slice().to_string(),
                }),
                Dictionary::DRANGLE => cmd.append = Some(Token{
                    cat: lex.next().unwrap(),
                    str: lex.slice().to_string(),
                }),
                Dictionary::AMPERSAND => {
                    self.foreground = false;
                    break;
                },
                _ => { // Pipe or unknown token
                    self.cmds.push(Command::new());
                    cmd = self.cmds.last_mut().unwrap();
                }
            }
        }
    }
}