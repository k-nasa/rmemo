use colored::*;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::io::Result;

#[derive(Debug, Clone)]
pub struct FileOrDir {
    name: String,
    path: String,
    is_dir: bool,
}

impl FileOrDir {
    pub fn print(&self) {
        if self.is_dir {
            println!("{}{}", self.name.cyan(), "/".cyan());
        } else {
            println!("{}", self.name);
        }
    }

    pub fn remove(&self) -> Result<()> {
        if self.is_dir {
            remove_dir_all(&self.path)
        } else {
            remove_file(&self.path)
        }
    }
    pub fn files(dir: &str, pattern: &str) -> Vec<FileOrDir> {
        read_dir(dir)
            .unwrap()
            .map(|dir_entry| {
                let dir_entry = dir_entry.unwrap();
                let name = dir_entry.file_name().into_string().unwrap();
                let path = dir_entry.path().to_str().unwrap().to_string();
                let is_dir = dir_entry.file_type().unwrap().is_dir();

                FileOrDir { name, path, is_dir }
            })
            .filter(|f| f.name.contains(pattern))
            .collect()
    }
}
