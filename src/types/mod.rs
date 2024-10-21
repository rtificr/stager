use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Act {
    pub title: String,
    pub author: String,
    pub description: String,
    pub elements: HashMap<String, Element>
}
#[derive(Debug, Clone)]
pub struct Element {
    pub content: String,
    pub body: Option<ElementBody>,
}
#[derive(Debug, Clone)]
pub enum ElementBody {
    Direct(String),
    List(Vec<Choice>)
}
#[derive(Clone, Debug)]
pub struct Choice {
    //choice will only be shown if all conds are met
    pub cond: Option<String>,
    //cmd will be executed only if choice is taken
    pub cmd: Option<String>,
    
    pub text: String,
    pub dest: String,
}