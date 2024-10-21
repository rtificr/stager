use std::collections::HashMap;
use colored::Colorize;
use crate::process::token::SemTok;
use crate::types::{Choice, Element, ElementBody};

pub struct Parser<'t> {
    pub tokens: &'t Vec<SemTok>,
    pub pos: usize,
}
impl<'t> Parser<'t> {
    pub fn new(tokens: &Vec<SemTok>) -> Parser {
        Parser {
            tokens,
            pos: 0,
        }
    }
    pub fn parse(&mut self) -> Result<HashMap<String, Element>, String> {
        let mut elements: HashMap<String, Element> = HashMap::new();
        while let Some(SemTok::Title(title)) = self.peek().cloned() {
            let meta = title.starts_with('#');
            println!("Title found!");
            self.advance();
            if let Some(SemTok::Content(content)) = self.peek().cloned() {
                println!("Content found!");
                self.advance();
                if !meta {
                    let body = self.parse_body()?;
                    elements.insert(title.clone(), Element {
                        content: content.clone(),
                        body: Some(body),
                    });
                } else {
                    elements.insert(title.clone(), Element {
                        content: content.clone(),
                        body: None,
                    });
                }
            }
            self.advance()
        }
        Ok(elements)
    }

    fn parse_body(&mut self) -> Result<ElementBody, String> {
        let mut body: ElementBody = ElementBody::Direct("#end".to_string());
        loop {
            if self.peek().is_none() {
                break;
            }
            println!("Checking at head...");
            match self.peek().unwrap() {
                SemTok::Dest(dest) => {
                    body = ElementBody::Direct(dest.clone());
                    self.advance();
                    break;
                }
                SemTok::Choice(_) => {
                    println!("Choice found!");
                    let mut choices: Vec<Choice> = vec![];
                    while let Some(SemTok::Choice(choice)) = self.peek() {
                        //splits "(condition) text" into condition and text
                        let (cond, text) = parse_choice(choice.clone());
                        self.advance();

                        let dest;
                        let cmd;
                        if let Some(SemTok::Dest(dest_str)) = self.peek() {
                            //splits "destination (command)" into destination and command
                            let (parsed_dest, parsed_cmd) = parse_dest(dest_str.clone());

                            dest = parsed_dest;
                            cmd = parsed_cmd;
                        } else {
                            dest = "#end".to_string();
                            cmd = None
                        }

                        choices.push(Choice {
                            cond,
                            cmd,
                            text,
                            dest,
                        });
                        self.advance();
                    }
                    if choices.len() == 0 {
                        return Err("Expected at least one choice".to_string());
                    }
                    body = ElementBody::List(choices.clone());
                }
                SemTok::EOL => {
                    println!("EOL found!");
                    break
                },
                _ => return Err("Expected choice or destination".to_string())
            }
        }
        Ok(body)
    }

    pub fn advance(&mut self) { self.pos += 1; self.display() }
    pub fn next(&self) -> Option<&SemTok> { self.tokens.get(self.pos + 1) }
    pub fn peek(&self) -> Option<&SemTok> { self.tokens.get(self.pos) }
    pub fn last(&self) -> Option<&SemTok> { self.tokens.get(self.pos - 1) }
    pub fn undo(&mut self) { self.pos += 1; }
    fn display(&self) {
        let toks = self.tokens.clone();
        print!("{:<10}", format!("POS {} > ", self.pos));
        for (i, val) in toks.iter().enumerate() {
            if i == self.pos {
                print!("{}", format!("{val:?} ").red());
            } else {
                print!("{val:?} ");
            }
        }
        println!();
    }
}

//returns (condition, text)
fn parse_choice(choice: String) -> (Option<String>, String) {
    let begin = choice.find('(');
    let end = choice.rfind(')');
    if begin.is_some() && end.is_some() {
        let cond = choice[begin.unwrap()+1..end.unwrap()].to_string();
        let text = choice[end.unwrap()+1..].to_string();
        (Some(cond.trim().to_string()), text.trim().to_string())
    } else {
        (None, choice.trim().to_string())
    }
}
//returns (text, command)
fn parse_dest(choice: String) -> (String, Option<String>) {
    let begin = choice.find('(');
    let end = choice.rfind(')');
    if begin.is_some() && end.is_some() {
        let cond = choice[begin.unwrap()+1..end.unwrap()].to_string();
        let text = choice[..begin.unwrap()].to_string();
        (text.trim().to_string(), Some(cond.trim().to_string()))
    } else {
        (choice.trim().to_string(), None)
    }
}