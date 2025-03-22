use crate::runtime::Runtime;
use crate::{sc_err, sp_err};

//not too proud of this one

pub fn parse_cond_script(string: String) -> Result<Vec<ScriptTok>, String>{
    let conditions = string.split("&").flat_map(|s| s.split("|")).collect::<Vec<&str>>();
    let mut stmts: Vec<ScriptTok> = Vec::new();
    for cond in conditions {
        let toks = cond.split_whitespace().filter(|t| !t.is_empty()).collect::<Vec<&str>>();
        if toks.len() != 3 { return Err(sp_err!("Expected three tokens!")); }
        if !matches!(toks[1], "=" | "<" | "<=" | ">" | ">=" | "&" | "|") { return Err(sp_err!("Expected an operator!")); }
        if !toks[0].chars().all(|c| c.is_alphanumeric() || c == '_') { return Err(sp_err!("Expected an alphanumeric variable name!")); }
        if !toks[2].parse::<f32>().is_ok() { return Err(sp_err!("Expected a number!")); }

        let var = toks[0].to_string();
        let val = toks[2].parse().unwrap();

        stmts.push(match toks[1] {
            "=" => ScriptTok::Equals(var, val),
            "<" => ScriptTok::LessThan(var, val),
            "<=" => ScriptTok::LessThanEq(var, val),
            ">" => ScriptTok::GreaterThan(var, val),
            ">=" => ScriptTok::GreaterThanEq(var, val),
            "&" => ScriptTok::And,
            "|" => ScriptTok::Or,
            _ => return Err(sc_err!("Invalid operator!"))
        });
    }
    Ok(stmts)
}
pub fn parse_cmd_script(string: String) -> Result<Vec<ScriptTok>, String>{
    let cmds = string.split(',');
    let mut stmts: Vec<ScriptTok> = Vec::new();
    for cmd in cmds {
        let toks = cmd.split_whitespace().filter(|t| !t.is_empty()).collect::<Vec<&str>>();
        if toks.len() != 3 { return Err(sp_err!("Expected three tokens!")); }
        if !matches!(toks[1], "=" | "+=" | "-=" | "*=" | "/=" ) { return Err(sp_err!("Expected an operator!")); }
        if !toks[0].chars().all(|c| c.is_alphanumeric() || c == '_') { return Err(sp_err!("Expected an alphanumeric variable name!")); }
        if !toks[2].parse::<f32>().is_ok() { return Err(sp_err!("Expected a number!")); }
        
        let var = toks[0].to_string();
        let val = toks[2].parse().unwrap();
        
        stmts.push(match toks[1] {
            "=" => ScriptTok::Equals(var, val),
            "+=" => ScriptTok::PlusEq(var, val),
            "-=" => ScriptTok::MinusEq(var, val),
            "*=" => ScriptTok::TimesEq(var, val),
            "/=" => ScriptTok::DivEq(var, val),
            _ => return Err(sp_err!("Invalid operator!"))
        });
    }
    Ok(stmts)
}
#[derive(Debug, Clone, PartialEq)]
pub enum ScriptTok {
    Equals(String, f32),
    LessThan(String, f32),
    LessThanEq(String, f32),
    GreaterThan(String, f32),
    GreaterThanEq(String, f32),
    PlusEq(String, f32),
    MinusEq(String, f32),
    TimesEq(String, f32),
    DivEq(String, f32),
    And,
    Or  
}