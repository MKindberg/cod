mod language_utils;
mod languages;

use clap::{Arg, Command};
use language_utils::Language;
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

    fn update(&mut self, content: &str, language: &mut Box<dyn Language>) {
        if let Some(lang) = language.language() {
            let mut parser = TS::Parser::new();
            parser.set_language(&lang).unwrap();

            let tree = parser.parse(&content, None).unwrap();
            let root_node = tree.root_node();

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

fn parse_file(
    languages: &mut Vec<Box<dyn Language>>,
    language_map: &mut HashMap<String, Stats>,
    filename: &str,
) {
    for l in languages {
        if l.matches_filename(filename) {
            let Ok(content) = fs::read_to_string(filename) else {
                let name = "Binary";
                if !language_map.contains_key(name) {
                    language_map.insert(name.to_string(), Stats::new());
                }
                language_map.get_mut(name).unwrap().files += 1;
                return;
            };
            if !language_map.contains_key(l.name()) {
                language_map.insert(l.name().to_string(), Stats::new());
            }
            language_map.get_mut(l.name()).unwrap().update(&content, l);
            l.filename_callback(filename);
            break;
        }
    }
}

fn parse_dir(file_list: &mut Vec<String>, ignore_list: &mut Vec<glob::Pattern>, dirname: &str) {
    for entry in fs::read_dir(dirname).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.file_name().unwrap() == ".gitignore" {
            let content = fs::read_to_string(&path).unwrap();
            for line in content.lines() {
                let l = line.trim();
                if l.starts_with('#') || line.len() == 0 {
                    continue;
                }
                let s: String = path.parent().unwrap().to_str().unwrap().to_string() + "/" + l + "*";
                let pat = glob::Pattern::new(s.trim_start_matches("./")).unwrap();
                ignore_list.push(pat);
            }
        }
        if path.file_name().unwrap().to_str().unwrap().starts_with(".") {
            continue;
        }
        if path.is_dir() {
            parse_dir(file_list, ignore_list, path.to_str().unwrap());
        } else {
            file_list.push(path.to_str().unwrap().trim_start_matches("./").to_string())
        }
    }
}

fn main() {
    let mut language_map: HashMap<String, Stats> = HashMap::new();
    let mut languages = languages::get_languages();
    let mut file_list = vec![];

    let matches = Command::new("cod")
        .author("mkindberg")
        .about("Count code related metrics in files")
        .arg(
            Arg::new("ignore")
                .short('i')
                .long("ignore")
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("files")
                .action(clap::ArgAction::Append)
                .default_value("."),
        )
        .get_matches();

    let mut ignore: Vec<glob::Pattern> = matches
        .get_many::<String>("ignore")
        .unwrap_or_default()
        .map(|s| glob::Pattern::new(&s).unwrap())
        .collect();
    let wanted_langs: Vec<String> = matches
        .get_many::<String>("language")
        .unwrap_or_default()
        .cloned()
        .collect();
    let file_args: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap_or_default()
        .cloned()
        .collect::<Vec<String>>();

    for f in file_args {
        if std::path::Path::new(&f).is_file() {
            file_list.push(f);
        } else if std::path::Path::new(&f).is_dir() {
            parse_dir(&mut file_list, &mut ignore, &f);
        }
    }

    file_list = file_list
        .iter()
        .filter(|f| ignore.iter().filter(|i| i.matches(f)).count() == 0)
        .cloned()
        .collect();

    for file in file_list {
        parse_file(&mut languages, &mut language_map, &file);
    }

    Stats::print_header();
    for (k, v) in language_map.iter() {
        v.print(k);
    }
    for l in languages.iter() {
        if wanted_langs.contains(&l.name().to_lowercase()) {
            println!();
            l.print();
        }
    }
}

fn print_nodes(filename: &str, lang: TS::Language) {
    let content = fs::read_to_string(filename).unwrap();
    let mut parser = TS::Parser::new();
    parser.set_language(&lang).unwrap();

    let tree = parser.parse(&content, None).unwrap();
    let root_node = tree.root_node();
    let mut cursor = root_node.walk();
    while let Some(node) = next_node(&mut cursor) {
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

#[test]
fn read_rust() {
    let mut language_map: HashMap<String, Stats> = HashMap::new();
    let mut languages = languages::get_languages();
    parse_file(&mut languages, &mut language_map, "test_files/test.rs");
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
    let mut language_map: HashMap<String, Stats> = HashMap::new();
    let mut languages = languages::get_languages();
    parse_file(&mut languages, &mut language_map, "test_files/test.cpp");
    assert!(language_map.contains_key("Cpp"));
    let cpp = language_map.get("Cpp").unwrap();
    assert_eq!(cpp.files, 1);
    assert_eq!(cpp.total_lines, 27);
    assert_eq!(cpp.blank_lines, 7);
    assert_eq!(cpp.functions, 2);
    assert_eq!(cpp.variables, 4);
    assert_eq!(cpp.loops, 4);
}

#[test]
fn read_c() {
    let mut language_map: HashMap<String, Stats> = HashMap::new();
    let mut languages = languages::get_languages();
    parse_file(&mut languages, &mut language_map, "test_files/test.c");
    assert!(language_map.contains_key("C"));
    let c = language_map.get("C").unwrap();
    assert_eq!(c.files, 1);
    assert_eq!(c.total_lines, 22);
    assert_eq!(c.blank_lines, 6);
    assert_eq!(c.functions, 2);
    assert_eq!(c.variables, 4);
    assert_eq!(c.loops, 3);
}

#[test]
fn read_zig() {
    print_nodes("test_files/test.zig", tree_sitter_zig::LANGUAGE.into());
    let mut language_map: HashMap<String, Stats> = HashMap::new();
    let mut languages = languages::get_languages();
    parse_file(&mut languages, &mut language_map, "test_files/test.zig");
    assert!(language_map.contains_key("Zig"));
    let zig = language_map.get("Zig").unwrap();
    assert_eq!(zig.files, 1);
    assert_eq!(zig.total_lines, 27);
    assert_eq!(zig.blank_lines, 5);
    assert_eq!(zig.functions, 2);
    assert_eq!(zig.variables, 9);
    assert_eq!(zig.loops, 4);
}
