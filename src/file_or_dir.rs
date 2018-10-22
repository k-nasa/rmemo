use colored::*;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::io::Result;

#[derive(Debug, Clone)]
pub struct FileOrDir {
    name: String,
    path: String,
    is_dir: bool,
}

pub type FileOrDirs = Vec<FileOrDir>;

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
    pub fn files(dir: &str, pattern: &str) -> FileOrDirs {
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

pub fn file_or_dirs_print(file_or_dirs: &FileOrDirs) {
    for file in file_or_dirs {
        file.print();
    }
}

pub fn file_or_dirs_remove(file_or_dirs: &FileOrDirs) {
    for file in file_or_dirs {
        file.remove().expect("Faild remove file");
    }
}
