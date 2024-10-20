pub fn err_code(id: u16) -> String {
    format!("Error code: {}", id)
}
#[macro_export] macro_rules! err {
    ($id:expr, $msg:expr) => {
        format!("Error code {}: {}", $id, $msg).as_str()
    };
    ($id:expr) => {
        format!("Error code {}", $id).as_str()
    };
    () => {};
}