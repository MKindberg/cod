mod cpp;
mod other;
mod rust;

use tree_sitter as TS;

pub struct Json {}
pub fn get_languages() -> Vec<Box<dyn Language>> {
    vec![
        Box::new(rust::Rust {}),
        Box::new(cpp::Cpp {}),
        Box::new(Json {}),
        other::Other::new(),
    ]
}

pub trait Language {
    fn matches_filename(&self, filename: &str) -> bool;
    fn name(&self) -> &str;
    fn language(&self) -> Option<TS::Language> {
        None
    }
    fn loop_query(&self) -> Option<&str> {
        None
    }
    fn function_query(&self) -> Option<&str> {
        None
    }
    fn variable_query(&self) -> Option<&str> {
        None
    }
    fn filename_callback(&mut self, _: &str) {}
    fn print(&self) {}
}

impl Language for Json {
    fn name(&self) -> &str {
        "JSON"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".json")
    }
}
