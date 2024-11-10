use crate::lang_struct;
use crate::language_utils::Language;

use tree_sitter as TS;

pub fn languages() -> Vec<Box<dyn Language>> {
    let mut l: Vec<Box<dyn Language>> = vec![];
    lang_struct!(l, Json, ".json");
    lang_struct!(l, Makefile, "Makefile", "makefile", ".make");
    lang_struct!(l, CMake, "CMakeLists.txt", ".cmake");
    lang_struct!(l, Ninja, ".ninja");
    lang_struct!(l, Markdown, ".md");
    lang_struct!(l, Text, ".txt");
    lang_struct!(l, Toml, ".toml");
    lang_struct!(l, Xml, ".xml");
    lang_struct!(l, Yaml, ".yaml", ".yml");

    lang_struct!(l, Python, ".py");
    lang_struct!(l, Java, ".java");
    lang_struct!(l, JavaScript, ".js");
    lang_struct!(l, TypeScript, ".ts");
    lang_struct!(l, Lua, ".lua");
    lang_struct!(l, Vim, ".vim");
    lang_struct!(l, Shell, ".sh");
    lang_struct!(l, Bash, ".bash");
    lang_struct!(l, Zsh, ".zsh");
    lang_struct!(l, Fish, ".fish");
    lang_struct!(l, Go, ".go");

    use crate::language_utils::Operation;
    use crate::language_utils::QType::*;
    lang_struct!(l,
        Rust,
        ending ".rs",
        ts tree_sitter_rust,
        Loops; "(for_expression) (while_expression) (loop_expression)",
        Functions; "(function_item)",
        Variables; "(let_declaration) (const_item) (static_item)"
    );
    lang_struct!(l,
        Cpp,
        endings (".cpp", ".hpp", ".cc", ".hh"),
        ts tree_sitter_cpp,
        Loops; "(for_range_loop) (for_statement) (while_statement) (do_statement)",
        Functions; "(function_definition)",
        Variables; "(declaration)",
        Templates; "(template_declaration)",
        Defines; "(preproc_def)"
    );
    lang_struct!(l,
        C,
        endings (".c", ".h"),
        ts tree_sitter_c,
        Loops; "(for_statement) (while_statement) (do_statement)",
        Functions; "(function_definition)",
        Variables; "(declaration)"
    );
    lang_struct!(l,
        Zig,
        ending ".zig",
        ts tree_sitter_zig,
        Loops; "(for_statement) (for_expression) (while_statement) (while_expression)",
        Functions; "(function_declaration)",
        Variables; "(variable_declaration)"
    );

    lang_struct!(l, Other, "");
    return l;
}
