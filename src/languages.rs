mod c;
mod cpp;
mod other;
mod rust;
mod zig;

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

pub fn get_languages() -> Vec<Box<dyn Language>> {
    lang_vec!(
        rust::Rust {},
        cpp::Cpp {},
        c::C {},
        zig::Zig {},
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
