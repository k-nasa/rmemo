use clap::ArgMatches;
use colored::*;
use config::Config;
use file_or_dir::{file_or_dirs_print, file_or_dirs_remove, FileOrDir, FileOrDirs};
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

    let files: FileOrDirs = FileOrDir::files(&memo_dir, &pattern);

    if files.is_empty() {
        println!("{}", "No matched file".yellow());
        return;
    }

    file_or_dirs_print(&files);

    println!("{}", "Will delete those entry. Are you sure?".red());
    confirmation!("Are you sure?(y/n) :");
    confirmation!("Really?(y/n) :");

    file_or_dirs_remove(&files);

    println!("{}", "All file delete".green());
}
