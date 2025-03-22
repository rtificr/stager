use std::fs::File;
use std::io::Read;
use crate::process::parser::parse::Parser;
use crate::process::token::Token;
use crate::process::tokenizer::tokenize;
use crate::types::{Act, ElementBody};

mod tokenizer;
mod assembler;
mod token;
mod semanticize;
pub mod parser;

pub fn parse_act(path: &str) -> Result<Act, String> {
    let mut act_file = File::open(path).map_err(|e| e.to_string())?;
    let mut act_str = String::new();
    act_file.read_to_string(&mut act_str).map_err(|e| e.to_string())?;

    let tokens = tokenize(act_str)?;
    //println!("{:?}", tokens);
    let semtoks = semanticize::semanticize(&tokens)?;
    // println!("{:?}", semtoks);
    let mut parser = Parser::new(&semtoks);
    let act = parser.parse()?;
    println!("{:#?}", act);

    /*for (k, v) in act.iter() {
        println!("{}: {}", k, v.content);
        match &v.body.clone() {
            None => {}
            Some(body) => match body {
                ElementBody::Direct(dest) => {
                    println!("<direct>");
                    println!(" ╰─> {}", dest);
                }
                ElementBody::List(choices) => {
                    println!("<list>");
                    for (i, choice) in choices.iter().enumerate() {
                        if i != choices.len() - 1 { println!(" ├─> {} ─> {}", choice.text, choice.dest); } else { println!(" ╰─> {} ─> {}", choice.text, choice.dest); }
                    }
                }
            }
        }
    }*/

    Ok(Act {
        title: act.get("#t").unwrap().content.clone(),
        author: act.get("#a").unwrap().content.clone(),
        description: act.get("#d").unwrap().content.clone(),
        elements: act
    })
}