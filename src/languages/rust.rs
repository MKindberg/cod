use crate::languages::Language;
use tree_sitter as TS;

pub struct Rust {}
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
