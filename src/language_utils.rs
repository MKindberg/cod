use tree_sitter as TS;

#[macro_export]
macro_rules! lang_struct {
    ($language_vec: expr, $name:ident, $($file_ending:expr),*) => {
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
        $language_vec.push(Box::new($name {}));
    };

    ($language_vec:expr, $name:ident,
        ending $file_ending:expr,
        ts $ts:ident,
        $($qtype:expr; $query:expr),+
        ) => {
            lang_struct!(
                $language_vec,
                $name,
                endings ($file_ending),
                ts $ts,
                $($qtype; $query),+
            );
    };
    ($language_vec:expr, $name:ident,
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
    $language_vec.push(Box::new($name {}));
    };
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum QType {
    Loops,
    Functions,
    Variables,
    Templates,
    Defines,
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
}
