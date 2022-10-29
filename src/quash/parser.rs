use super::command::Command;
use super::Job;

use logos::Logos;
use super::lexer::Dictionary;

impl Job {
    pub fn parse(&mut self, str: &String) {
        self.str = str.to_string();

        self.cmds.push(Command::new());
        let mut cmd: &mut Command = self.cmds.last_mut().unwrap();

        let mut lex = Dictionary::lexer(&str);
        while let Some(token) = lex.next() {
            // Expecting arguments / operators to the command
            if let Some(_) = cmd.keyword {
                match token {
                    Dictionary::Text(txt) => cmd.args.push(txt),
                    Dictionary::LANGLE(_) => cmd.read = Some(lex.next().unwrap()),
                    Dictionary::RANGLE(_) => cmd.write = Some(lex.next().unwrap()),
                    Dictionary::DRANGLE(_) => cmd.append = Some(lex.next().unwrap()),
                    Dictionary::AMPERSAND(_) => {
                        self.foreground = false;
                        break;
                    },
                    _ => { // Pipe or unknown token
                        self.cmds.push(Command::new());
                        cmd = self.cmds.last_mut().unwrap();
                    }
                }
            } else {
                cmd.keyword = Some(token);
            }
            
        }
    }
}