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

struct Stats {
    total_lines: usize,
    blank_lines: usize,
    language: Language,
}

impl Stats {
    fn new(filename: &str) -> Self {
        let mut stats = Self {
            total_lines: 0,
            blank_lines: 0,
            language: Language::new(filename),
        };
        stats.update(filename);
        return stats;
    }

    fn update(&mut self, filename: &str) {
        let content = fs::read_to_string(filename).unwrap();

        for line in content.lines() {
            self.total_lines += 1;
            if line.trim().is_empty() {
                self.blank_lines += 1;
            }
        }
    }

    fn print(&self) {
        println!("Language: {:?}", self.language);
        println!("Total lines: {}", self.total_lines);
        println!("Blank lines: {}", self.blank_lines);
    }
}

fn parse_file(language_map: &mut HashMap<Language, Stats>, filename: &str) {
    let mut parser = TS::Parser::new();
    let language = Language::new(filename);
    language.set_language(&mut parser);
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
