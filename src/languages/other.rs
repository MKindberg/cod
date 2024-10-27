use crate::languages::Language;

use std::path::Path;

pub struct Other {
    pub file_endings: Vec<String>,
}

impl Other {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            file_endings: vec![],
        })
    }
}
impl Language for Other {
    fn name(&self) -> &str {
        "Other"
    }
    fn matches_filename(&self, _: &str) -> bool {
        true
    }
    fn filename_callback(&mut self, filename: &str) {
        let p = Path::new(filename);
        if let Some(ext) = p.extension() {
            let s = ext.to_str().unwrap();
            if !self.file_endings.contains(&s.to_string()) {
                self.file_endings.push(s.to_string());
            }
        }
    }
    fn print(&self) {
        println!("Other file endings:");
        for ending in &self.file_endings {
            println!("  {}", ending);
        }
    }
}
