use std::path::Path;
use tree_sitter as TS;

pub struct Rust {}
pub struct Cpp {}
pub struct Json {}
pub struct Other {
    pub file_endings: Vec<String>,
}

pub fn get_languages() -> Vec<Box<dyn Language>> {
    vec![
        Box::new(Rust {}),
        Box::new(Cpp {}),
        Box::new(Json {}),
        Other::new(),
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

impl Language for Rust {
    fn name(&self) -> &str {
        "Rust"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".rs")
    }
    fn language(&self) -> Option<TS::Language> {
        Some(tree_sitter_rust::LANGUAGE.into())
    }
    fn loop_query(&self) -> Option<&str> {
        Some(
            "
(for_expression)
(while_expression)
(loop_expression)
",
        )
    }
    fn function_query(&self) -> Option<&str> {
        Some("(function_item)")
    }
    fn variable_query(&self) -> Option<&str> {
        Some(
            "
(let_declaration)
(const_item)
(static_item)
",
        )
    }
}

impl Language for Cpp {
    fn name(&self) -> &str {
        "C++"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".cpp")
            || filename.ends_with(".cc")
            || filename.ends_with(".hpp")
            || filename.ends_with(".hh")
    }
    fn language(&self) -> Option<TS::Language> {
        Some(tree_sitter_cpp::LANGUAGE.into())
    }
    fn loop_query(&self) -> Option<&str> {
        Some(
            "
(for_range_loop)
(for_statement)
(while_statement)
(do_statement)
        ",
        )
    }
    fn function_query(&self) -> Option<&str> {
        Some("(function_definition)")
    }
    fn variable_query(&self) -> Option<&str> {
        Some(
            "
(declaration)
",
        )
    }
}

impl Language for Json {
    fn name(&self) -> &str {
        "JSON"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".json")
    }
}

impl Other {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            file_endings: vec![],
        })
    }
}
impl Language for Other {
    fn name(&self) -> &str {
        "Other"
    }
    fn matches_filename(&self, _: &str) -> bool {
        true
    }
    fn filename_callback(&mut self, filename: &str) {
        let p = Path::new(filename);
        if let Some(ext) = p.extension() {
            let s = ext.to_str().unwrap();
            if !self.file_endings.contains(&s.to_string()) {
                self.file_endings.push(s.to_string());
            }
        }
    }
    fn print(&self) {
        println!("Other file endings:");
        for ending in &self.file_endings {
            println!("  {}", ending);
        }
    }
}
