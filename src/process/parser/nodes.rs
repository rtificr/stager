#[derive(Debug)]
pub struct Element {
    pub content: String,
    pub body: ElementBody,
}

#[derive(Debug)]
pub enum ElementBody {
    End,
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