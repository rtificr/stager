use crate::process::token::{SemTok, Token};
use crate::process::token::SemTok::*;
use crate::process::token::Token::{StrTok, SymbTok};
use crate::process::token::Symbol::*;

pub fn semanticize(tokens: &Vec<Token>) -> Result<Vec<SemTok>, String> {
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

fn ctx(last: Option<&Token>, cur: Option<&Token>, next: Option<&Token>) -> Result<Option<SemTok>, String> {
    match cur {
        Some(StrTok(t)) => {
            if matches!(last.clone(), None | Some(SymbTok(SemiColon) | SymbTok(CloseBrace) | SymbTok(SemiColon))) &&
                matches!(next.clone(), Some(SymbTok(Colon))) {
                Ok(Some(Title(t.clone())))
            } else if matches!(last.clone(), Some(SymbTok(Colon))) &&
                matches!(next.clone(), Some(SymbTok(SemiColon)) | Some(SymbTok(Tilde)) | Some(SymbTok(OpenBrace))) {
                Ok(Some(Content(t.clone())))
            } else if matches!(last.clone(), Some(SymbTok(OpenBrace) | SymbTok(CloseParen) | SymbTok(SemiColon))) &&
                matches! (next.clone(), Some(SymbTok(Tilde))) {
                Ok(Some(Choice(t.clone())))
            } else if matches!(last.clone(), Some(SymbTok(Tilde))) &&
                matches! (next.clone(), Some(SymbTok(SemiColon))) {
                Ok(Some(Dest(t.clone())))
            } else if matches!(last.clone(), Some(SymbTok(OpenParen))) &&
                matches! (next.clone(), Some(SymbTok(CloseParen))) {
                Ok(Some(Condition(t.clone())))
            } else {
                Ok(None)
            }
        }
        Some(SymbTok(SemiColon)) => Ok(Some(EOL)),
        _ => Ok(None),
    }
}