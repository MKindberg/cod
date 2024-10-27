use crate::languages::Language;
use tree_sitter as TS;

pub struct Cpp {}
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
