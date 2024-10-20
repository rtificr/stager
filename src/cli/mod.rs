use crate::process::parse_act;

pub fn start_cli() {
    let toks = parse_act("test.act").unwrap();
}