use tree_sitter as TS;

pub struct Rust {}
pub struct Other {}

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
        Some("
(let_declaration)
(const_item)
(static_item)
")
    }
}

impl Language for Other {
    fn name(&self) -> &str {
        "Other"
    }
    fn matches_filename(&self, _: &str) -> bool {
        true
    }
}