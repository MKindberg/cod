use lazy_static::lazy_static;
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
        fn queries(&self) -> &'static Vec<Operation> {
            lazy_static!{
                static ref operations: Vec<Operation> = vec![
                $(
                    Operation{qtype: $qtype, query: TS::Query::new(&TS::Language::from($ts::LANGUAGE), $query).unwrap()},
                )*
                ];
            }
            &operations
        }
    }
    $language_vec.push(Box::new($name {}));
    };
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum QType {
    Loops,
    Functions,
    Variables,
    Templates,
    Defines,
}

pub struct Operation {
    pub qtype: QType,
    pub query: TS::Query,
}

pub trait Language: Send + Sync {
    fn matches_filename(&self, filename: &str) -> bool;
    fn name(&self) -> &str;
    fn language(&self) -> Option<TS::Language> {
        None
    }
    fn queries(&self) -> &'static Vec<Operation> {
        static V: Vec<Operation> = vec![];
        return &V;
    }
}
