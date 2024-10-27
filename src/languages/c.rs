use crate::languages::Language;
use tree_sitter as TS;

pub struct C {}
impl Language for C {
    fn name(&self) -> &str {
        "C"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".c") || filename.ends_with(".c") || filename.ends_with(".h")
    }
    fn language(&self) -> Option<TS::Language> {
        Some(tree_sitter_c::LANGUAGE.into())
    }
    fn loop_query(&self) -> Option<&str> {
        Some(
            "
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
