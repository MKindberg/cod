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
        $($qtype:expr; $query:expr),+
        ) => {
            lang_struct!(
                $name,
                endings ($file_ending),
                ts $ts,
                $($qtype; $query),+
            );
    };
    ($name:ident,
        endings ($($file_endings:expr),+),
        ts $ts:ident,
        $($qtype:expr; $query:expr),+
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
        fn queries(&self) -> Vec<Operation> {
            let mut operations: Vec<Operation> = Vec::new();
            $(
                operations.push(Operation{qtype: $qtype, query: $query.to_string()});
            )*
            operations
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

#[derive(Eq, Hash, PartialEq)]
pub enum QType {
    Loops,
    Functions,
    Variables,
}

pub struct Operation {
    pub qtype: QType,
    pub query: String,
}

pub trait Language {
    fn matches_filename(&self, filename: &str) -> bool;
    fn name(&self) -> &str;
    fn language(&self) -> Option<TS::Language> {
        None
    }
    fn queries(&self) -> Vec<Operation> {
        vec![]
    }
    fn filename_callback(&mut self, _: &str) {}
    fn print(&self) {}
}
