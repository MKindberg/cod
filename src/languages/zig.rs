use crate::languages::Language;
use tree_sitter as TS;

pub struct Zig {}
impl Language for Zig {
    fn name(&self) -> &str {
        "Zig"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".zig")
    }
    fn language(&self) -> Option<TS::Language> {
        Some(tree_sitter_zig::LANGUAGE.into())
    }
    fn loop_query(&self) -> Option<&str> {
        Some(
            "
(for_statement)
(for_expression)
(while_statement)
(while_expression)
        ",
        )
    }
    fn function_query(&self) -> Option<&str> {
        Some("(function_declaration)")
    }
    fn variable_query(&self) -> Option<&str> {
        Some(
            "
(variable_declaration)
",
        )
    }
}
