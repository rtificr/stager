use crate::runtime::Runtime;
use crate::runtime::scriptparse::{parse_cmd_script, parse_cond_script, ScriptTok};
use crate::{sc_err, sp_err};

impl Runtime {
    pub fn eval_script(&self, string: Option<String>) -> Result<bool, String> {
        println!("Evaluating script: {:?}", string);
        if string.is_none() { return Ok(true); }
        let script = parse_cond_script(string.unwrap())?;
        println!("Evaluating script: {:?}", script);
        let mut result = true;
        let mut current_result = true;
        let mut last_op = ScriptTok::And;
        
        for stmt in script {
            match stmt {
                ScriptTok::Equals(var, val) => {
                    current_result = self.vars.get(&var).unwrap_or(&0f32) == &val;
                }
                ScriptTok::LessThan(var, val) => {
                    current_result = self.vars.get(&var).unwrap_or(&0f32) < &val;
                }
                ScriptTok::LessThanEq(var, val) => {
                    current_result = self.vars.get(&var).unwrap_or(&0f32) <= &val;
                }
                ScriptTok::GreaterThan(var, val) => {
                    current_result = self.vars.get(&var).unwrap_or(&0f32) > &val;
                }
                ScriptTok::GreaterThanEq(var, val) => {
                    current_result = self.vars.get(&var).unwrap_or(&0f32) >= &val;
                }
                ScriptTok::And => {
                    result = result && current_result;
                    current_result = true;
                    last_op = ScriptTok::And;
                }
                ScriptTok::Or => {
                    result = result || current_result;
                    current_result = true;
                    last_op = ScriptTok::Or;
                }
                _ => return Err(sc_err!("Invalid condition!")),
            }
        }
        
        if last_op == ScriptTok::And {
            result = result && current_result;
        } else if last_op == ScriptTok::Or {
            result = result || current_result;
        }
        Ok(result)
    }
    pub fn exec_script(&mut self, string: Option<String>) -> Result<(), String> {
        if string.is_none() { return Ok(()) }
        let script = parse_cmd_script(string.unwrap())?;
        for stmt in script {
            match stmt {
                ScriptTok::Equals(var, val) => {
                    self.vars.insert(var, val);
                }
                ScriptTok::PlusEq(var, val) => {
                    let mut v = self.vars.get(&var).unwrap_or(&0f32).clone();
                    v += val;
                    self.vars.insert(var, v);
                }
                ScriptTok::MinusEq(var, val) => {
                    let mut v = self.vars.get(&var).unwrap_or(&0f32).clone();
                    v -= val;
                    self.vars.insert(var, v);
                }
                ScriptTok::TimesEq(var, val) => {
                    let mut v = self.vars.get(&var).unwrap_or(&0f32).clone();
                    v *= val;
                    self.vars.insert(var, v);
                }
                ScriptTok::DivEq(var, val) => {
                    let mut v = self.vars.get(&var).unwrap_or(&0f32).clone();
                    v /= val;
                    self.vars.insert(var, v);
                }
                _ => return Err(sc_err!("Cannot compare variables in a command!"))
            }
        }
        Ok(())
    }
}