use crate::process::parse_act;
use crate::runtime::Runtime;

pub fn start_cli() {
    let act = parse_act("test.act").unwrap();
    let mut rt = Runtime::new(act);
    rt.run().unwrap();
}