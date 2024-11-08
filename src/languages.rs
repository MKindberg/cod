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
lang_struct!(Makefile, "Makefile", "makefile", ".make");
lang_struct!(CMake, "CMakeLists.txt", ".cmake");
lang_struct!(Ninja, ".ninja");
lang_struct!(Markdown, ".md");
lang_struct!(Text, ".txt");
lang_struct!(Toml, ".toml");
lang_struct!(Xml, ".xml");
lang_struct!(Yaml, ".yaml", ".yml");

use crate::language_utils::Operation;
use crate::language_utils::QType::*;
lang_struct!(
    Rust,
    ending ".rs",
    ts tree_sitter_rust,
    Loops; "(for_expression) (while_expression) (loop_expression)",
    Functions; "(function_item)",
    Variables; "(let_declaration) (const_item) (static_item)"
);
lang_struct!(
    Cpp,
    endings (".cpp", ".hpp", ".cc", ".hh"),
    ts tree_sitter_cpp,
    Loops; "(for_range_loop) (for_statement) (while_statement) (do_statement)",
    Functions; "(function_definition)",
    Variables; "(declaration)",
    Templates; "(template_declaration)",
    Defines; "(preproc_def)"
);
lang_struct!(
    C,
    endings (".c", ".h"),
    ts tree_sitter_c,
    Loops; "(for_statement) (while_statement) (do_statement)",
    Functions; "(function_definition)",
    Variables; "(declaration)"
);
lang_struct!(
    Zig,
    ending ".zig",
    ts tree_sitter_zig,
    Loops; "(for_statement) (for_expression) (while_statement) (while_expression)",
    Functions; "(function_declaration)",
    Variables; "(variable_declaration)"
);
