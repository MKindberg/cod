use std::collections::hash_map::HashMap;
use std::fs;
use tree_sitter as TS;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Language {
    Rust,
    Other,
}

impl Language {
    fn new(filename: &str) -> Self {
        if filename.ends_with(".rs") {
            return Self::Rust;
        }
        return Self::Other;
    }

    fn set_language(&self, parser: &mut TS::Parser) {
        match self {
            Self::Rust => parser
                .set_language(&tree_sitter_rust::language())
                .expect("Error loading Rust grammar"),
            Self::Other => (),
        }
    }
}
impl Default for Language {
    fn default() -> Self {
        Self::Other
    }
}

#[derive(Default)]
struct Stats {
    total_lines: usize,
    blank_lines: usize,
    functions: usize,
    variables: usize,
    loops: usize,
    language: Language,
}

impl Stats {
    fn new(filename: &str) -> Self {
        let mut stats = Self::default();
        stats.language = Language::new(filename);

        stats.update(filename);
        return stats;
    }

    fn update(&mut self, filename: &str) {
        let content = fs::read_to_string(filename).unwrap();

        let mut parser = TS::Parser::new();
        self.language.set_language(&mut parser);

        let tree = parser.parse(&content, None).unwrap();
        let root_node = tree.root_node();

        let mut cursor = root_node.walk();

        while let Some(node) = Self::next_node(&mut cursor) {
            if node.kind() == "function_item" {
                self.functions += 1;
            } else if node.kind() == "let" {
                self.variables += 1;
            } else if node.kind() == "for_expression" || node.kind() == "loop_expression" || node.kind() == "while_expression" {
                self.loops += 1;
            }
        }
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

    fn print(&self) {
        println!("Language: {:?}", self.language);
        println!("Total lines: {}", self.total_lines);
        println!("Blank lines: {}", self.blank_lines);
        println!("Functions: {}", self.functions);
        println!("Variables: {}", self.variables);
        println!("Loops: {}", self.loops);
    }
}

fn parse_file(language_map: &mut HashMap<Language, Stats>, filename: &str) {
    let language = Language::new(filename);
    if let Some(l) = language_map.get_mut(&language) {
        l.update(filename);
    } else {
        language_map.insert(language, Stats::new(filename));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut language_map: HashMap<Language, Stats> = HashMap::new();
    for arg in args {
        if std::path::Path::new(&arg).is_file() {
            parse_file(&mut language_map, &arg);
        }
    }

    for l in language_map.values() {
        l.print();
    }
}
