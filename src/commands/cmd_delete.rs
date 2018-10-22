use clap::ArgMatches;
use colored::*;
use config::Config;
use file_or_dir::FileOrDir;
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

    let files: Vec<FileOrDir> = FileOrDir::files(&memo_dir, &pattern);

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
