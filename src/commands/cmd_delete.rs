use clap::ArgMatches;
use colored::*;
use config::Config;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::io::*;
use std::string::*;
use termion::event::{Event, Key};
use termion::input::TermRead;

macro_rules! confirmation {
    ($question_string:expr) => {
        println!("{}", $question_string);
        match stdin().events().nth(0).unwrap().unwrap() {
            Event::Key(Key::Char('y')) => (),
            Event::Key(Key::Char('Y')) => (),
            _ => return,
        }
    };
}

#[derive(Debug, Clone)]
struct FileOrDir {
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
}

pub fn cmd_delete(matches: &ArgMatches, config: &Config) {
    let pattern = match matches.value_of("pattern") {
        Some(pattern) => pattern.to_string(),
        None => String::new(),
    };

    let memo_dir = config.memos_dir();

    let files: Vec<FileOrDir> = files(&memo_dir, &pattern);

    if files.is_empty() {
        println!("{}", "No matched file".yellow());
        return;
    }

    for file in &files {
        file.print();
    }

    println!("{}", "Will delete those entry. Are you sure?".red());
    confirmation!("Are you sure?(y/n) :");
    confirmation!("Really?(y/n) :");

    for file in files {
        file.remove().expect("Faild remove file");
    }

    println!("{}", "All file delete".green());
}

fn files(memo_dir: &str, pattern: &str) -> Vec<FileOrDir> {
    read_dir(memo_dir)
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
