use std::io::stdin;

pub fn input_as_num() -> Option<u16> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().ok()
}