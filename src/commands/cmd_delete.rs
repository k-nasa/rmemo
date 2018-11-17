use clap::ArgMatches;
use colored::*;
use crate::config::Config;
use dialoguer::Select;
use crate::file_or_dir::{file_names, file_or_dirs_print, file_or_dirs_remove, FileOrDir, FileOrDirs};
use std::io::*;
use std::string::*;
use termion::event::{Event, Key};
use termion::input::TermRead;

macro_rules! confirmation {
    ($question_string:expr) => {
        print!("{}", $question_string);
        std::io::stdout().flush().expect("print! is faild");

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
    let files: FileOrDirs = FileOrDir::filter_files(&memo_dir, &pattern);

    if files.is_empty() {
        println!("{}", "No matched file".yellow());
        return;
    }

    if matches.is_present("pick") {
        pick_file_delete(&files);
    } else {
        all_files_delete(&files);
    }
}

fn pick_file_delete(files: &[FileOrDir]) {
    let selections = file_names(files);
    let selections: Vec<&str> = selections.iter().map(|name| name.as_str()).collect();
    let selections = selections.as_slice();

    let selection = Select::new()
        .default(0)
        .items(selections)
        .interact()
        .unwrap();

    match files[selection].remove() {
        Ok(_) => println!("{}", "File deleted!!".green()),
        Err(e) => println!("{}", e),
    };
}

fn all_files_delete(files: &[FileOrDir]) {
    file_or_dirs_print(&files);

    println!("{}", "Will delete those entry. Are you sure?".red());
    confirmation!("Are you sure?(y/n) :");
    confirmation!("Really?(y/n) :");

    file_or_dirs_remove(&files);

    println!("{}", "All file delete".green());
}
