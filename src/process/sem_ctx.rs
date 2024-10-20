use crate::process::token::{SemTok, Symbol, Token};
use crate::process::token::SemTok::*;
use crate::process::token::Symbol::*;
use crate::process::token::Token::*;

pub fn ctx(last: Option<&Token>, cur: Option<&Token>, next: Option<&Token>) -> Result<Option<SemTok>, String> {
    match cur {
        Some(StrTok(t)) => {
            if matches!(last.clone(), None | Some(SymbTok(SemiColon))) &&
                matches!(next.clone(), Some(SymbTok(Colon))) {
                Ok(Some(Title(t.clone())))
            } else if matches!(last.clone(), Some(SymbTok(Colon))) &&
                matches!(next.clone(), Some(SymbTok(SemiColon)) | Some(SymbTok(Tilde)) | Some(SymbTok(OpenBrace))) {
                Ok(Some(Content(t.clone())))
            } else if matches!(last.clone(), Some(SymbTok(OpenBrace) | SymbTok(CloseParen))) && 
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