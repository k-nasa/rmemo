use colored::*;
use std::cmp::Ordering;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::io::Result;

#[derive(Debug, Clone, Eq)]
pub struct FileOrDir {
    name: String,
    path: String,
    is_dir: bool,
}

impl PartialEq for FileOrDir {
    fn eq(&self, other: &FileOrDir) -> bool {
        self.path == other.path
    }
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

    pub fn files(dir: &str) -> FileOrDirs {
        FileOrDir::filter_files(dir, "")
    }

    pub fn filter_files(dir: &str, pattern: &str) -> FileOrDirs {
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

pub fn file_or_dirs_print(file_or_dirs: &[FileOrDir]) {
    for file in file_or_dirs {
        file.print();
    }
}

pub fn file_or_dirs_remove(file_or_dirs: &[FileOrDir]) {
    for file in file_or_dirs {
        file.remove().expect("Faild remove file");
    }
}

pub fn file_names(file_or_dirs: &[FileOrDir]) -> Vec<String> {
    //FIXME clone to ref
    file_or_dirs.iter().map(|f| f.name.clone()).collect()
}

pub fn file_paths(file_or_dirs: &[FileOrDir]) -> Vec<String> {
    //FIXME clone to ref
    file_or_dirs.iter().map(|f| f.path.clone()).collect()
}
