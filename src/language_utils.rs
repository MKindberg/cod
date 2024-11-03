use tree_sitter as TS;

#[macro_export]
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
        ending $file_ending:expr,
        ts $ts:ident,
        loops $loops:expr,
        functions $funcs:expr,
        variables $vars:expr,
        ) => {
            lang_struct!(
                $name,
                endings ($file_ending),
                ts $ts,
                loops $loops,
                functions $funcs,
                variables $vars,
            );
    };

    ($name:ident,
        endings ($($file_endings:expr),+),
        ts $ts:ident,
        loops $loops:expr,
        functions $funcs:expr,
        variables $vars:expr,
        ) => {
    pub struct $name {}
    impl Language for $name {
        fn name(&self) -> &str {
            stringify!($name)
        }
        fn matches_filename(&self, filename: &str) -> bool {
            $(
                if filename.ends_with($file_endings) {return true;}
            )+
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
#[macro_export]
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

