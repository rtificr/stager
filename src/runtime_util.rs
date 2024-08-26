use std::fs;
use fancy_regex::Regex;
use crate::act::Act;
use crate::error::err_code;

//returns act in string form, absolute path or not
pub fn get_act(mut path: String) -> Result<Act, String> {
    let mut p = path;
    let pattern_file = Regex::new(r".*\.act").map_err(|e| err_code("200", e))?;

    if pattern_file.find(p.clone().as_str()).map_err(|e| err_code("210", e))?.is_none() {
        p.push_str(".act");
    }

    println!("Loading '{p}'");

    let content = fs::read_to_string(&p)
        .map_err(|e| format!("Error reading '{p}':\n{}",e))?;

    let cleaned_content = content.lines().collect::<Vec<&str>>().join("");

    Act::new(cleaned_content)
}
// Makes sure the correct amount of arguments is present
pub fn range(args: Vec<String>, min: usize, max: usize) -> Result<(), String> {
    if args.len() > max {
        return Err(format!("Too many arguments: Minimum is {min}"));
    }
    if args.len() < min {
        return Err(format!("Too few arguments: Maximum is {max}"));
    }

    Ok(())
}
//binds boolean to the presence of an argument
pub fn arg_bind(args: Vec<String>, arg: String, flag: &mut bool) {
    *flag = args.contains(&arg);
}