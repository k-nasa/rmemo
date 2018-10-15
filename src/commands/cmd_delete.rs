use clap::ArgMatches;
use colored::*;
use config::Config;
use std::fs::read_dir;
use std::fs::remove_file;
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

pub fn cmd_delete(matches: &ArgMatches, config: &Config) {
    let pattern = match matches.value_of("pattern") {
        Some(pattern) => pattern.to_string(),
        None => String::new(),
    };

    let memo_dir = config.memos_dir();

    let full_path_files: Vec<String> = full_path_files(&memo_dir, &pattern);
    let display_file_paths: Vec<String> = display_file_paths(&memo_dir, &pattern);

    if display_file_paths.is_empty() {
        println!("{}", "No matched file".yellow());
        return;
    }

    for file in display_file_paths {
        println!("{}", file);
    }

    println!("{}", "Will delete those entry. Are you sure?".red());
    confirmation!("Are you sure?(y/n) :");
    confirmation!("Really?(y/n) :");

    for file in full_path_files {
        remove_file(file).expect("failed remove files");
    }

    println!("{}", "All file delete".green());
}

fn full_path_files(memo_dir: &str, pattern: &str) -> Vec<String> {
    read_dir(memo_dir)
        .unwrap()
        .map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_string())
        .filter(|c| c.contains(pattern))
        .collect()
}

fn display_file_paths(memo_dir: &str, pattern: &str) -> Vec<String> {
    read_dir(memo_dir)
        .unwrap()
        .map(|dir_entry| dir_entry.unwrap().file_name().into_string().unwrap())
        .filter(|c| c.contains(pattern))
        .collect()
}
