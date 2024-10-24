use std::collections::hash_map::HashMap;
use std::fs;
use streaming_iterator::StreamingIterator;
use tree_sitter as TS;

trait Language {
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
}

struct Rust {}
impl Language for Rust {
    fn name(&self) -> &str {
        "Rust"
    }
    fn matches_filename(&self, filename: &str) -> bool {
        filename.ends_with(".rs")
    }
    fn language(&self) -> Option<TS::Language> {
        Some(tree_sitter_rust::LANGUAGE.into())
    }
    fn loop_query(&self) -> Option<&str> {
        Some(
            "
(for_expression)
(while_expression)
(loop_expression)
",
        )
    }
    fn function_query(&self) -> Option<&str> {
        Some("(function_item)")
    }
    fn variable_query(&self) -> Option<&str> {
        Some("(let_declaration)")
    }
}

struct Other {}
impl Language for Other {
    fn name(&self) -> &str {
        "Other"
    }
    fn matches_filename(&self, _: &str) -> bool {
        true
    }
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
}

#[derive(Default)]
struct Stats {
    files: usize,
    total_lines: usize,
    blank_lines: usize,
    functions: usize,
    variables: usize,
    loops: usize,
}

impl Stats {
    fn new() -> Self {
        let stats = Self::default();

        return stats;
    }

    fn update(&mut self, filename: &str, language: &Box<dyn Language>) {
        let Ok(content) = fs::read_to_string(filename) else {
            return;
        };

        if let Some(lang) = language.language() {
            let mut parser = TS::Parser::new();
            parser.set_language(&lang).unwrap();

            let tree = parser.parse(&content, None).unwrap();
            let root_node = tree.root_node();

            let mut query_cursor = TS::QueryCursor::new();

            if let Some(query) = language.function_query() {
                let function_query = TS::Query::new(&parser.language().unwrap(), query).unwrap();
                self.functions = query_cursor
                    .matches(&function_query, root_node, content.as_bytes())
                    .count();
            }
            if let Some(query) = language.variable_query() {
                let variable_query = TS::Query::new(&parser.language().unwrap(), query).unwrap();
                self.variables = query_cursor
                    .matches(&variable_query, root_node, content.as_bytes())
                    .count();
            }
            if let Some(query) = language.loop_query() {
                let loop_query = TS::Query::new(&parser.language().unwrap(), query).unwrap();
                self.loops = query_cursor
                    .matches(&loop_query, root_node, content.as_bytes())
                    .count();
            }
        }

        self.files += 1;
        for line in content.lines() {
            self.total_lines += 1;
            if line.trim().is_empty() {
                self.blank_lines += 1;
            }
        }
    }

    fn next_node<'a>(cursor: &mut TS::TreeCursor<'a>) -> Option<TS::Node<'a>> {
        if cursor.goto_first_child() {
            return Some(cursor.node());
        }

        if cursor.goto_next_sibling() {
            return Some(cursor.node());
        }

        loop {
            if !cursor.goto_parent() {
                return None;
            }
            if cursor.goto_next_sibling() {
                return Some(cursor.node());
            }
        }
    }

    fn print_header() {
        println!("Language\tFiles\tTotal lines\tBlank lines\tFunctions\tVariables\tLoops");
        println!("---------------------------------------------------------------------------------------");
    }

    fn print(&self, name: &str) {
        println!(
            "{}\t\t{}\t{}\t\t{}\t\t{}\t\t{}\t\t{}",
            name,
            self.files,
            self.total_lines,
            self.blank_lines,
            self.functions,
            self.variables,
            self.loops
        );
    }
}

fn parse_file<'a>(
    languages: &'a Vec<Box<dyn Language>>,
    language_map: &mut HashMap<&'a str, Stats>,
    filename: &str,
) {
    for l in languages {
        if l.matches_filename(filename) {
            if !language_map.contains_key(l.name()) {
                language_map.insert(l.name(), Stats::new());
            }
            language_map.get_mut(l.name()).unwrap().update(filename, l);
            break;
        }
    }
}

fn parse_dir<'a>(
    languages: &'a Vec<Box<dyn Language>>,
    language_map: &mut HashMap<&'a str, Stats>,
    dirname: &str,
) {
    for entry in fs::read_dir(dirname).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            parse_dir(languages, language_map, path.to_str().unwrap());
        } else {
            parse_file(languages, language_map, path.to_str().unwrap());
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut language_map: HashMap<&str, Stats> = HashMap::new();
    let languages: Vec<Box<dyn Language>> = vec![Box::new(Rust {}), Box::new(Other {})];
    for arg in args {
        if std::path::Path::new(&arg).is_file() {
            parse_file(&languages, &mut language_map, &arg);
        } else if std::path::Path::new(&arg).is_dir() {
            parse_dir(&languages, &mut language_map, &arg);
        }
    }

    Stats::print_header();
    for (k, v) in language_map.iter() {
        v.print(k);
    }
}
