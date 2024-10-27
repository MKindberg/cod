mod c;
mod cpp;
mod other;
mod rust;
mod zig;

use tree_sitter as TS;

pub struct Json {}
pub struct Toml {}
pub struct Text {}
pub struct Markdown {}
pub struct Makefile {}
pub struct Xml {}
pub struct Yaml {}
pub fn get_languages() -> Vec<Box<dyn Language>> {
    vec![
        Box::new(rust::Rust {}),
        Box::new(cpp::Cpp {}),
        Box::new(c::C {}),
        Box::new(zig::Zig {}),
        Box::new(Json {}),
        Box::new(Toml {}),
        Box::new(Markdown {}),
        Box::new(Makefile {}),
        Box::new(Xml {}),
        Box::new(Yaml {}),
        Box::new(Text {}),
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

impl Language for Toml {
    fn name(&self) -> &str {
        "TOML"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".toml")
    }
}

impl Language for Text {
    fn name(&self) -> &str {
        "Text"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".txt")
    }
}

impl Language for Markdown {
    fn name(&self) -> &str {
        "Markdown"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".md")
    }
}

impl Language for Makefile {
    fn name(&self) -> &str {
        "Makefile"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename == "Makefile" || filename == "makefile"
    }
}

impl Language for Xml {
    fn name(&self) -> &str {
        "XML"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".xml")
    }
}

impl Language for Yaml {
    fn name(&self) -> &str {
        "YAML"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".yaml") | filename.ends_with(".yml")
    }
}
