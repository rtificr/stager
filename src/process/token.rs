#[derive(Debug, Clone)]
pub enum Token {
    StrTok(String),
    IntTok(i32),
    SymbTok(Symbol),
}
#[derive(Debug, Clone)]
pub enum Symbol {
    Colon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Tilde,
    SemiColon,
    OpenAngle,
    CloseAngle,
}
#[derive(Debug, Clone)]
pub enum SemTok {
    Title(String),
    Content(String),
    Condition(String),
    Command(String),
    Choice(String),
    Dest(String),
    Script(String),
    EOL
}
impl Token {
    pub fn from_char(c: char) -> Result<Token, String> {
        match c {
            ':' => Ok(Token::SymbTok(Symbol::Colon)),
            '{' => Ok(Token::SymbTok(Symbol::OpenBrace)),
            '}' => Ok(Token::SymbTok(Symbol::CloseBrace)),
            '~' => Ok(Token::SymbTok(Symbol::Tilde)),
            ';' => Ok(Token::SymbTok(Symbol::SemiColon)),
            '<' => Ok(Token::SymbTok(Symbol::OpenAngle)),
            '>' => Ok(Token::SymbTok(Symbol::CloseAngle)),
            _ => Err("Invalid character".to_string()),
        }
    }
}

pub fn is_symbol(c: char) -> bool {
    match c {
        // angle brackets are not included, since they are inline
        ':' | '{' | '}' | '~' | ';' | '\\' => true,
        _ => false,
    }
}