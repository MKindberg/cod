mod languages;

use languages::{Cpp, Language, Other, Rust};
use std::collections::hash_map::HashMap;
use std::fs;
use streaming_iterator::StreamingIterator;
use tree_sitter as TS;

struct Stats {
    files: usize,
    total_lines: usize,
    blank_lines: usize,
    functions: usize,
    variables: usize,
    loops: usize,
}

impl Stats {
    const fn new() -> Self {
        let stats = Self {
            files: 0,
            total_lines: 0,
            blank_lines: 0,
            functions: 0,
            variables: 0,
            loops: 0,
        };

        return stats;
    }

    fn update(&mut self, content: &str, language: &Box<dyn Language>) {
        if let Some(lang) = language.language() {
            let mut parser = TS::Parser::new();
            parser.set_language(&lang).unwrap();

            let tree = parser.parse(&content, None).unwrap();
            let root_node = tree.root_node();
            //Self::print_nodes(&root_node);

            let mut query_cursor = TS::QueryCursor::new();

            if let Some(query) = language.function_query() {
                let function_query = TS::Query::new(&parser.language().unwrap(), query).unwrap();
                self.functions += query_cursor
                    .matches(&function_query, root_node, content.as_bytes())
                    .count();
            }
            if let Some(query) = language.variable_query() {
                let variable_query = TS::Query::new(&parser.language().unwrap(), query).unwrap();
                self.variables += query_cursor
                    .matches(&variable_query, root_node, content.as_bytes())
                    .count();
            }
            if let Some(query) = language.loop_query() {
                let loop_query = TS::Query::new(&parser.language().unwrap(), query).unwrap();
                self.loops += query_cursor
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

    fn print_nodes(root_node: &TS::Node) {
        let mut cursor = root_node.walk();
        while let Some(node) = Self::next_node(&mut cursor) {
            println!("{}", node.kind());
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
        println!(
            "{:15}{:15}{:15}{:15}{:15}{:15}{:15}",
            "Language", "Files", "Total lines", "Blank lines", "Functions", "Variables", "Loops",
        );
        println!("{:-<width$}", "", width = 7 * 15);
    }

    fn print(&self, name: &str) {
        println!(
            "{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}",
            name,
            self.files,
            self.total_lines,
            self.blank_lines,
            self.functions,
            self.variables,
            self.loops,
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
            let Ok(content) = fs::read_to_string(filename) else {
                let name = "Binary";
                if !language_map.contains_key(name) {
                    language_map.insert(name, Stats::new());
                }
                language_map.get_mut(name).unwrap().files += 1;
                return;
            };
            if !language_map.contains_key(l.name()) {
                language_map.insert(l.name(), Stats::new());
            }
            language_map.get_mut(l.name()).unwrap().update(&content, l);
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
    let languages: Vec<Box<dyn Language>> =
        vec![Box::new(Rust {}), Box::new(Cpp {}), Box::new(Other {})];
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

#[test]
fn read_rust() {
    let mut language_map: HashMap<&str, Stats> = HashMap::new();
    let languages: Vec<Box<dyn Language>> = vec![Box::new(Rust {}), Box::new(Other {})];
    parse_dir(&languages, &mut language_map, "test_files");
    assert!(language_map.contains_key("Rust"));
    let rust = language_map.get("Rust").unwrap();
    assert_eq!(rust.files, 1);
    assert_eq!(rust.total_lines, 25);
    assert_eq!(rust.blank_lines, 2);
    assert_eq!(rust.functions, 2);
    assert_eq!(rust.variables, 4);
    assert_eq!(rust.loops, 3);
}

#[test]
fn read_cpp() {
    let mut language_map: HashMap<&str, Stats> = HashMap::new();
    let languages: Vec<Box<dyn Language>> = vec![Box::new(Cpp {}), Box::new(Other {})];
    parse_dir(&languages, &mut language_map, "test_files");
    assert!(language_map.contains_key("C++"));
    let rust = language_map.get("C++").unwrap();
    assert_eq!(rust.files, 1);
    assert_eq!(rust.total_lines, 27);
    assert_eq!(rust.blank_lines, 7);
    assert_eq!(rust.functions, 2);
    assert_eq!(rust.variables, 4);
    assert_eq!(rust.loops, 4);
}
