mod other;

use tree_sitter as TS;

macro_rules! lang_struct {
    ($name:ident, $($file_ending:expr),*) => {
        pub struct $name {}
        impl Language for $name {
            fn name(&self) -> &str {
                stringify!($name)
            }
            fn matches_filename(&self, filename: &str) -> bool {
            $(
                if filename.ends_with($file_ending) {return true;}
            )*
            return false;
            }
        }
    };

    ($name:ident,
        ts $ts:ident,
        loops $loops:expr,
        functions $funcs:expr,
        variables $vars:expr,
        endings $($file_ending:expr),*
        ) => {
    pub struct $name {}
    impl Language for $name {
        fn name(&self) -> &str {
            stringify!($name)
        }
        fn matches_filename(&self, filename: &str) -> bool {
        $(
            if filename.ends_with($file_ending) {return true;}
        )*
        return false;
        }
    fn language(&self) -> Option<TS::Language> {
        Some($ts::LANGUAGE.into())
    }
    fn loop_query(&self) -> Option<&str> {
        Some($loops)
    }
    fn function_query(&self) -> Option<&str> {
        Some($funcs)
    }
    fn variable_query(&self) -> Option<&str> {
        Some($vars)
    }
        }
    };
}
macro_rules! lang_vec {
    ( $($x:expr),* ) => {
        {
            let mut temp_vec: Vec<Box<dyn Language>> = Vec::new();
            $(
                temp_vec.push(Box::new($x));
            )*
            temp_vec
        }
    };
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
    ts tree_sitter_rust,
    loops "(for_expression) (while_expression) (loop_expression)",
    functions "(function_item)",
    variables "(let_declaration) (const_item) (static_item)",
    endings ".rs"
);
lang_struct!(
    Cpp,
    ts tree_sitter_cpp,
    loops "(for_range_loop) (for_statement) (while_statement) (do_statement)",
    functions "(function_definition)",
    variables "(declaration)",
    endings ".cpp", ".hpp", ".cc", ".hh"
);
lang_struct!(
    C,
    ts tree_sitter_c,
    loops "(for_statement) (while_statement) (do_statement)",
    functions "(function_definition)",
    variables "(declaration)",
    endings ".c", ".h"
);
lang_struct!(
    Zig,
    ts tree_sitter_zig,
    loops "(for_statement) (for_expression) (while_statement) (while_expression)",
    functions "(function_declaration)",
    variables "(variable_declaration)",
    endings ".zig"
);

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
