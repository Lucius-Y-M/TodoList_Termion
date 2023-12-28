/**
 *
 *
 *
 *
 * Items to be stored here
 *
 *
 */
#[allow(unused_macros)]
#[macro_export]
macro_rules! strNR {
    ($expr: ident) => {{
        let s = String::from_iter($expr.chars().chain("\n\r".chars()));
        s
    }};
}
#[derive(Debug)]
pub struct TodoItem {
    pub name: String,
    pub is_done: bool,
    pub is_recurr: bool,
}

impl Default for TodoItem {
    fn default() -> Self {
        Self::new("")
    }
}
#[allow(dead_code)]
impl TodoItem {
    pub fn new_full(name: &str, is_done: bool, is_recurr: bool) -> Self {
        Self {
            name: strNR!(name),
            is_done,
            is_recurr,
        }
    }
    pub fn new(name: &str) -> Self {
        Self {
            name: strNR!(name),
            is_done: false,
            is_recurr: false,
        }
    }
}
