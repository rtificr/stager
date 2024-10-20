use std::fs::File;
use std::io::Read;
use crate::process::parser::parse::Parser;
use crate::process::token::Token;
use crate::process::tokenizer::tokenize;

mod tokenizer;
mod assembler;
mod token;
mod semanticize;
mod sem_ctx;
mod parser;

pub fn parse_act(path: &str) -> Result<Vec<Token>, String> {
    let mut act_file = File::open(path).map_err(|e| e.to_string())?;
    let mut act_str = "".to_string();
    act_file.read_to_string(&mut act_str).map_err(|e| e.to_string())?;

    let tokens = tokenize(act_str)?;
    let semtoks = semanticize::semanticize(&tokens)?;
    println!("{:?}", semtoks);
    let mut parser = Parser::new(&semtoks);
    let act = parser.parse()?;
    
    for (k, v) in act.iter() {
        println!("{}: {}", k, v.content);
        match &v.body {
            parser::nodes::ElementBody::Direct(dest) => {
                println!("  -> {}", dest);
            }
            parser::nodes::ElementBody::List(choices) => {
                for choice in choices {
                    println!("  {} -> {}", choice.text, choice.dest);
                }
            }
            parser::nodes::ElementBody::End => (),
        }
    }

    Ok(tokens)
}