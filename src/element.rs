use crate::choice::Choice;

pub struct Element {
    pub content: String,
    pub choices: Vec<Choice>
}

impl Element {
    pub fn default() -> Element{
        let element = Element {
            content: String::new(),
            choices: Vec::new()
        };

        element
    }
    pub fn new(content: String, choices: Vec<Choice>) -> Element{
        let element = Element {
            content, choices
        };

        element
    }
}