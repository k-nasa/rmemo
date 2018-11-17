use super::run_editor;
use clap::ArgMatches;
use colored::*;
use crate::config::Config;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn cmd_quick(matches: &ArgMatches, config: &Config) {
    match matches.value_of("your idea") {
        Some(idea) => append_idea_to_file(idea, config),
        None => open_idea_file_with_editor(config),
    }
}

fn append_idea_to_file(idea: &str, config: &Config) {
    let dir = config.memos_dir();
    let filepath = format!("{}/storage_place_of_your_idea.md", dir);

    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .read(true)
        .open(filepath)
    {
        Ok(file) => file,
        Err(e) => panic!(e),
    };

    let idea = format!("{}\n", idea);
    match file.write_all(idea.as_bytes()) {
        Ok(_) => println!("{}", "Suceess".green()),
        Err(e) => println!("{}", e),
    }
}

fn open_idea_file_with_editor(config: &Config) {
    let dir = config.memos_dir();
    let editor = config.editor();
    let filepath = format!("{}/storage_place_of_your_idea.md", dir);

    run_editor(editor, &filepath);
}
