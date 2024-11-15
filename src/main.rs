#[macro_use]
extern crate lazy_static;

mod language_utils;
mod languages;

use clap::{Arg, Command};
use language_utils::Language;
use std::collections::hash_map::HashMap;
use std::fs;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use streaming_iterator::StreamingIterator;
use tree_sitter as TS;

use language_utils::QType;

#[derive(Clone)]
struct Stats {
    name: String,
    files: usize,
    total_lines: usize,
    blank_lines: usize,
    operations: HashMap<QType, usize>,
}

impl Stats {
    fn new(name: &str) -> Self {
        let stats = Self {
            name: name.to_string(),
            files: 0,
            total_lines: 0,
            blank_lines: 0,
            operations: HashMap::new(),
        };

        return stats;
    }

    fn add(&mut self, other: &Stats) {
        assert!(self.name == other.name);
        self.files += other.files;
        self.total_lines += other.total_lines;
        self.blank_lines += other.blank_lines;
        for (k, v) in &other.operations {
            if self.operations.contains_key(&k) {
                *self.operations.get_mut(&k).unwrap() += v;
            } else {
                self.operations.insert(k.clone(), *v);
            }
        }
    }

    fn update(&mut self, content: &str, language: &Box<dyn Language>) {
        if let Some(lang) = language.language() {
            let mut parser = TS::Parser::new();
            parser.set_language(&lang).unwrap();

            let tree = parser.parse(&content, None).unwrap();
            let root_node = tree.root_node();

            let mut query_cursor = TS::QueryCursor::new();

            for query in language.queries() {
                let matches = query_cursor
                    .matches(&query.query, root_node, content.as_bytes())
                    .count();
                let count = self.operations.get(&query.qtype).unwrap_or(&0);
                self.operations.insert(query.qtype.clone(), count + matches);
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
        println!("{:=<width$}", "", width = 7 * 15);
    }

    fn print(&self) {
        let functions = self
            .operations
            .get(&QType::Functions)
            .map_or("-".to_string(), |n| n.to_string());
        let variables = self
            .operations
            .get(&QType::Variables)
            .map_or("-".to_string(), |n| n.to_string());
        let loops = self
            .operations
            .get(&QType::Loops)
            .map_or("-".to_string(), |n| n.to_string());
        println!(
            "{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}",
            self.name, self.files, self.total_lines, self.blank_lines, functions, variables, loops,
        );
    }
    fn print_detailed(&self) {
        println!("*** {} ***", self.name);
        println!("Number of files: {}", self.files);
        println!("Total lines: {}", self.total_lines);
        println!("Blank lines: {}", self.blank_lines);
        for (k, v) in &self.operations {
            println!("{:?}: {}", k, v);
        }
    }
}

fn parse_file(
    languages: &Vec<Box<dyn Language>>,
    language_map: &mut HashMap<String, Stats>,
    filename: &str,
) {
    for l in languages {
        if l.matches_filename(filename) {
            let Ok(content) = fs::read_to_string(filename) else {
                let name = "Binary";
                if !language_map.contains_key(name) {
                    language_map.insert(name.to_string(), Stats::new(name));
                }
                language_map.get_mut(name).unwrap().files += 1;
                return;
            };
            if !language_map.contains_key(l.name()) {
                language_map.insert(l.name().to_string(), Stats::new(l.name()));
            }
            language_map.get_mut(l.name()).unwrap().update(&content, l);
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
                let mut s: String = path.parent().unwrap().to_str().unwrap().to_string() + "/" + l;
                if !s.ends_with("*") {
                    s += "*";
                }
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
    let languages = Arc::new(languages::languages());
    let mut file_list = vec![];

    let matches = Command::new("cod")
        .name("COD")
        .author("mkindberg")
        .about("Count lines other code related metrics in files")
        .arg(
            Arg::new("ignore")
                .short('i')
                .long("ignore")
                .help("Glob expression for files to ignore. Can be used multiple times.")
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .help("Language to show detailed information for. Can be used multiple times.")
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("no-summary")
                .long("no-summary")
                .help("Don't show summary, can be useful with -l")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .default_value("1")
                .value_parser(clap::value_parser!(usize)),
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
    let show_summary = !*matches.get_one::<bool>("no-summary").unwrap();
    let jobs: usize = *matches.get_one::<usize>("jobs").unwrap();

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

    file_list.sort_by_key(|s| {
        std::path::Path::new(s)
            .extension()
            .unwrap_or(std::ffi::OsStr::new(""))
            .to_str()
            .unwrap()
            .to_string();
    });

    let list_sizes = file_list.len() / jobs;
    let (tx, rx) = mpsc::channel();
    for i in 0..jobs - 1 {
        let langs = languages.clone();
        let fl = file_list[i * list_sizes..(i + 1) * list_sizes].to_vec();
        let thread_tx = tx.clone();
        thread::spawn(move || {
            let mut res = HashMap::<String, Stats>::new();
            for file in fl {
                parse_file(&langs, &mut res, &file);
            }
            thread_tx.send(res).unwrap();
        });
    }

    for file in &file_list[(jobs - 1) * list_sizes..] {
        parse_file(&languages, &mut language_map, &file);
    }
    for _ in 0..jobs - 1 {
        for (k, v) in rx.recv().unwrap().iter() {
            if language_map.contains_key(k) {
                language_map.get_mut(k).unwrap().add(v);
            } else {
                language_map.insert(k.to_string(), v.clone());
            }
        }
    }

    let mut stats = vec![];
    for v in language_map.values() {
        stats.push(v);
    }
    if show_summary {
        Stats::print_header();

        stats.sort_by_key(|s| s.name.clone());
        let mut total = Stats::new("Total");
        total.operations.insert(QType::Functions, 0);
        total.operations.insert(QType::Variables, 0);
        total.operations.insert(QType::Loops, 0);
        for s in stats.iter() {
            s.print();
            total.files += s.files;
            total.total_lines += s.total_lines;
            total.blank_lines += s.blank_lines;
            *total.operations.get_mut(&QType::Functions).unwrap() +=
                s.operations.get(&QType::Functions).unwrap_or(&0);
            *total.operations.get_mut(&QType::Variables).unwrap() +=
                s.operations.get(&QType::Variables).unwrap_or(&0);
            *total.operations.get_mut(&QType::Loops).unwrap() +=
                s.operations.get(&QType::Loops).unwrap_or(&0);
        }
        println!("{:-<width$}", "", width = 7 * 15);
        total.print();
        println!();
    }
    let mut other_endings = vec![];
    for file in file_list {
        for l in languages.iter() {
            if l.name() == "Other" {
                if let Some(ext) = std::path::Path::new(&file).extension() {
                    let e = ext.to_str().unwrap().to_string();
                    if !other_endings.contains(&e) {
                        other_endings.push(e);
                    }
                }
            }
            if l.matches_filename(&file) {
                break;
            }
        }
    }
    for s in stats.iter() {
        if wanted_langs.contains(&s.name.to_lowercase()) {
            s.print_detailed();
            if s.name.to_lowercase() == "other" {
                println!("Other file endings: ");
                for e in &other_endings {
                    println!("  {}", e);
                }
            }
            println!();
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

#[cfg(test)]
mod test {
    use super::*;

    fn lang_arc() -> Arc<Vec<Box<dyn Language>>> {
        Arc::new(languages::languages())
    }
    #[test]
    fn read_rust() {
        let mut language_map: HashMap<String, Stats> = HashMap::new();
        let mut languages = lang_arc();
        parse_file(&mut languages, &mut language_map, "test_files/test.rs");
        assert!(language_map.contains_key("Rust"));
        let rust = language_map.get("Rust").unwrap();
        assert_eq!(rust.files, 1);
        assert_eq!(rust.total_lines, 25);
        assert_eq!(rust.blank_lines, 2);
        assert_eq!(rust.operations.get(&QType::Functions).unwrap(), &2);
        assert_eq!(rust.operations.get(&QType::Variables).unwrap(), &4);
        assert_eq!(rust.operations.get(&QType::Loops).unwrap(), &3);
    }

    #[test]
    fn read_cpp() {
        let mut language_map: HashMap<String, Stats> = HashMap::new();
        let mut languages = lang_arc();
        parse_file(&mut languages, &mut language_map, "test_files/test.cpp");
        assert!(language_map.contains_key("Cpp"));
        let cpp = language_map.get("Cpp").unwrap();
        assert_eq!(cpp.files, 1);
        assert_eq!(cpp.total_lines, 29);
        assert_eq!(cpp.blank_lines, 8);
        assert_eq!(cpp.operations.get(&QType::Functions).unwrap(), &2);
        assert_eq!(cpp.operations.get(&QType::Variables).unwrap(), &4);
        assert_eq!(cpp.operations.get(&QType::Loops).unwrap(), &4);
        assert_eq!(cpp.operations.get(&QType::Templates).unwrap(), &1);
        assert_eq!(cpp.operations.get(&QType::Defines).unwrap(), &1);
    }

    #[test]
    fn read_c() {
        let mut language_map: HashMap<String, Stats> = HashMap::new();
        let mut languages = lang_arc();
        parse_file(&mut languages, &mut language_map, "test_files/test.c");
        assert!(language_map.contains_key("C"));
        let c = language_map.get("C").unwrap();
        assert_eq!(c.files, 1);
        assert_eq!(c.total_lines, 22);
        assert_eq!(c.blank_lines, 6);
        assert_eq!(c.operations.get(&QType::Functions).unwrap(), &2);
        assert_eq!(c.operations.get(&QType::Variables).unwrap(), &4);
        assert_eq!(c.operations.get(&QType::Loops).unwrap(), &3);
    }

    #[test]
    fn read_zig() {
        print_nodes("test_files/test.zig", tree_sitter_zig::LANGUAGE.into());
        let mut language_map: HashMap<String, Stats> = HashMap::new();
        let mut languages = lang_arc();
        parse_file(&mut languages, &mut language_map, "test_files/test.zig");
        assert!(language_map.contains_key("Zig"));
        let zig = language_map.get("Zig").unwrap();
        assert_eq!(zig.files, 1);
        assert_eq!(zig.total_lines, 27);
        assert_eq!(zig.blank_lines, 5);
        assert_eq!(zig.operations.get(&QType::Functions).unwrap(), &2);
        assert_eq!(zig.operations.get(&QType::Variables).unwrap(), &9);
        assert_eq!(zig.operations.get(&QType::Loops).unwrap(), &4);
    }
}
