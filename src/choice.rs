pub struct Choice {
    pub text: String,
    pub dest: String
}
impl Choice {
    pub fn new(text: String, dest: String) -> Choice{
        let choice = Choice {
            text, dest
        };

        choice
    }
}