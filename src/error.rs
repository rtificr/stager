use std::fmt;

/* CODES:
0xx: Input errors
1xx: Load-time errors
2xx: Runtime errors
x0x: Regex compilation errors
x1x: Regex matching errors
*/
pub fn err_code_msg<T: fmt::Display>(code: &str, e: T, msg: &str) -> String {
    format!("{}:\n{}\n    {e}", err_code_blank(code), msg)
}
pub fn err_code<T: fmt::Display>(code: &str, e: T) -> String {
    format!("{}:\n    {e}", err_code_blank(code))
}
pub fn err_code_blank(code: &str) -> String {
    format!("ERROR CODE {code}; Please report bug (command 'help')")
}