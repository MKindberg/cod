mod other;

use crate::lang_struct;
use crate::lang_vec;
use crate::language_utils::Language;

use tree_sitter as TS;

pub fn get_languages() -> Vec<Box<dyn Language>> {
    lang_vec!(
        Rust {},
        Cpp {},
        C {},
        Zig {},
        Json {},
        Toml {},
        Markdown {},
        Makefile {},
        Xml {},
        Yaml {},
        Text {},
        other::Other::new()
    )
}

lang_struct!(Json, ".json");
lang_struct!(Makefile, "Makefile", "makefile");
lang_struct!(Markdown, ".md");
lang_struct!(Text, ".txt");
lang_struct!(Toml, ".toml");
lang_struct!(Xml, ".xml");
lang_struct!(Yaml, ".yaml", ".yml");

lang_struct!(
    Rust,
    ending ".rs",
    ts tree_sitter_rust,
    loops "(for_expression) (while_expression) (loop_expression)",
    functions "(function_item)",
    variables "(let_declaration) (const_item) (static_item)",
);
lang_struct!(
    Cpp,
    endings (".cpp", ".hpp", ".cc", ".hh"),
    ts tree_sitter_cpp,
    loops "(for_range_loop) (for_statement) (while_statement) (do_statement)",
    functions "(function_definition)",
    variables "(declaration)",
);
lang_struct!(
    C,
    endings (".c", ".h"),
    ts tree_sitter_c,
    loops "(for_statement) (while_statement) (do_statement)",
    functions "(function_definition)",
    variables "(declaration)",
);
lang_struct!(
    Zig,
    ending ".zig",
    ts tree_sitter_zig,
    loops "(for_statement) (for_expression) (while_statement) (while_expression)",
    functions "(function_declaration)",
    variables "(variable_declaration)",
);
