#[macro_export]
macro_rules! err {
    ($id:expr, $msg:expr) => {
        format!("Error code {}: {}", $id, $msg)
    };
    ($id:expr) => {
        format!("Error code {}", $id)
    };
    () => {};
}
#[macro_export]
macro_rules! p_err {
    ($($arg:tt)*) => (format!("Parsing error: {}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! rt_err {
    ($($arg:tt)*) => (format!("Runtime error: {}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! sp_err {
    ($($arg:tt)*) => (format!("Script parsing error: {}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! sc_err {
    ($($arg:tt)*) => (format!("Script runtime error: {}", format_args!($($arg)*)));
}