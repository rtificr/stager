pub struct Act {
    pub descs: Descs
}

impl Act {
    pub(crate) fn new() -> Act {
        todo!()
    }
}

pub struct Descs {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
}