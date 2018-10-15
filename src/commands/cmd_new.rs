extern crate chrono;

use super::run_editor;
use clap::ArgMatches;
use colored::*;
use config::Config;
use std::fs::{copy, create_dir_all};
use std::string::*;
use utils;

pub fn cmd_new(matches: &ArgMatches, config: &Config) {
    let input_filepath = match matches.value_of("title") {
        Some(title) => title.to_string(),
        None => {
            println!("Input title :");
            utils::read()
        }
    };

    if input_filepath.is_empty() {
        println!("{}", "Title is required!!".red());
        return;
    }

    let mut dir = config.memos_dir().clone();
    let editor = config.editor();

    // The last is the file name, the other is the directory structure
    let mut element: Vec<&str> = input_filepath.split('/').collect();

    let title: String;
    if element.len() < 2 {
        title = element.first().unwrap().to_string();
    } else {
        title = element.last().unwrap().to_string();
        element.pop();

        for elm in element {
            dir.push('/');
            dir.push_str(&elm.to_string());
        }
    }

    let title = match config.enter_time_in_filename {
        Some(true) => {
            let now = chrono::Local::now().format("%Y-%m-%d").to_string();
            format!("{}{}.md", now, title)
        }
        _ => format!("{}.md", title),
    };

    let filepath = format!("{}/{}", dir, title);
    create_dir_all(dir).expect("faild create directory");

    if matches.is_present("template") && !config.template_file_path().is_empty() {
        copy(config.template_file_path(), &filepath).expect("faild template file copy");
    }

    run_editor(editor, &filepath);
}
