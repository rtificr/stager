use crate::process::token::{is_symbol, Token};
use crate::process::token::Token::{StrTok, SymbTok};

pub fn tokenize(act_str: String) -> Result<Vec<Token>, String> {
    let mut chars = act_str.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&c) = chars.peek() {
        if is_symbol(c) {
            tokens.push(Token::from_char(c)?);
            chars.next(); // Consume the symbol
        } else {
            let mut word = String::new();
            let mut escaped = false;

            while let Some(&c) = chars.peek() {
                if escaped {
                    word.push(c);
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if is_symbol(c) {
                    break;
                } else {
                    word.push(c);
                }
                chars.next(); // Consume the character
            }
            
            let str = word.trim().to_string();
            if str.is_empty() {
                continue;
            }
            tokens.push(StrTok(str));
        }
    }
    Ok(tokens)
}