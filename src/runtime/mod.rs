mod script;
mod util;
mod scriptparse;

use std::collections::HashMap;
use std::ops::Deref;
use crate::rt_err;
use crate::runtime::util::input_as_num;
use crate::types::{Act, ElementBody};

pub struct Runtime {
    act: Act,
    vars: HashMap<String, f32>
}
impl Runtime {
    pub fn new(act: Act) -> Self {
        Self { act, vars: HashMap::new() }
    }
    pub fn run(&mut self) -> Result<(), String> {
        println!("'{}' by {}", self.act.title, self.act.author);
        println!("Description:");
        println!("{:2}{}", "", self.act.description);
        println!();

        self.exec(String::from("start"))
    }
    fn exec(&mut self, key: String) -> Result<(), String> {
        let element = self.act.elements.get(&key).ok_or(rt_err!("Element '{key}' not found!"))?;
        println!("{}", element.content);
        if element.body.is_none() {
            return Ok(());
        }
        match element.body.clone().unwrap() {
            ElementBody::Direct(dest) => {
                self.exec_script(Some(dest))
            }
            ElementBody::List(choices) => {
                for choice in choices.clone() {
                    if choice.text.is_none() && self.eval_script(choice.cond.clone())? {
                        self.exec(choice.dest.clone())?
                    }
                }
                let mut available_choices = Vec::new();
                for c in &choices {
                    if self.eval_script(c.cond.clone())? {
                        available_choices.push(c);
                    }
                }
                if available_choices.len() == 0 {
                    return Err(rt_err!("No choices available!"));
                }
                for (i, choice) in available_choices.iter().enumerate() {
                    //switches line symbol based on if it's the last choice
                    //let symb = if i != available_choices.len() - 1 { '├' } else { '╰' };
                    println!(" > {}: {}", i + 1, choice.text.clone().unwrap());
                }
                let input: u16;
                println!();
                loop {
                    if let Some(n) = input_as_num() {
                        input = n;
                        break;
                    } else {
                        println!("Invalid input! Please enter a number.");
                    }
                }
                println!();
                let choice = available_choices.get(input as usize - 1).ok_or(rt_err!("Choice {input} of element {key} leads to no destination!"))?.clone();
                
                self.exec_script(choice.cmd.clone())?;
                self.exec(choice.dest.clone())
            }
        }
    }
}
