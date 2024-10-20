use crate::err;
use crate::process::sem_ctx::ctx;
use crate::process::token::{SemTok, Token};

pub fn semanticize(tokens: &Vec<Token>) -> Result<Vec<SemTok>, String> {
    let semtoks = sem_pass(tokens)?;
    
    Ok(semtoks)
}

fn sem_pass(tokens: &Vec<Token>) -> Result<Vec<SemTok>, String> {
    let mut semtoks = Vec::new();
    let mut last = None;
    let mut cur = None;
    let mut next = None;

    for i in 0..tokens.len() {
        if i > 0 {
            last = Some(&tokens[i - 1]);
        }
        cur = Some(&tokens[i]);
        if i < tokens.len() - 1 {
            next = Some(&tokens[i + 1]);
        } else {
            next = None;
        }

        match ctx(last, cur, next)? {
            Some(st) => semtoks.push(st),
            None => (),
        }
    }

    Ok(semtoks)
}