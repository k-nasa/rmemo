extern crate chrono;
extern crate dirs;
extern crate serde;
extern crate toml;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use config::Config;
use std::fs::*;
use std::path::*;
use std::process::{Command, Stdio};
use std::str::from_utf8;
use utils;

pub fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(SubCommand::with_name("help").alias("h").about("help"))
        .subcommand(
            SubCommand::with_name("config")
                .alias("c")
                .about("edit config file"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("l")
                .about("show memos list"),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .alias("e")
                .about("edit memo")
                .arg(Arg::with_name("title").help("edit file title")),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .alias("d")
                .about("delete memos"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .alias("n")
                .about("create new memo")
                .arg(Arg::with_name("title").help("create file title")),
        )
}

pub fn cmd_config(config: &Config) {
    let dir = match dirs::home_dir() {
        Some(dir) => Path::new(&dir.to_str().unwrap().to_string()).join(".config/rsmemo/"), // Change path as test
        _ => Path::new("./").join(".config/memo/"),
    };

    DirBuilder::new()
        .recursive(true)
        .create(dir.clone())
        .unwrap();

    let filepath = &dir.join("config.toml");
    let filepath = filepath.to_str().unwrap();

    let editor = config.editor();
    run_editor(editor, filepath);
}

pub fn cmd_delete() {}

pub fn cmd_edit(matches: &ArgMatches, config: &Config) {
    let mut title = match matches.value_of("title") {
        Some(title) => title.to_string(),
        None => String::new(),
    };

    let dir = config.memos_dir();
    create_dir_all(dir).expect("faild create memos_dir");

    if title.is_empty() {
        title = run_selector(&"fzf".to_string(), dir);
    }

    if title.is_empty() {
        println!("File is not selected!");
        return;
    }

    let editor = config.editor();
    let filepath = format!("{}/{}", dir, title);

    run_editor(editor, &filepath);
}

pub fn cmd_list() {}

pub fn cmd_new(matches: &ArgMatches, config: &Config) {
    let title = match matches.value_of("title") {
        Some(title) => title.to_string(),
        None => {
            println!("Input title :");
            utils::read()
        }
    };

    let dir = config.memos_dir();
    let editor = config.editor();

    let title = match config.enter_time_in_filename {
        Some(true) => {
            let now = chrono::Local::now().format("%Y-%m-%d").to_string();
            format!("{}{}.md", now, title)
        }
        _ => format!("{}.md", title),
    };

    let filepath = format!("{}/{}", dir, title);

    create_dir_all(dir).expect("faild create memos_dir");

    run_editor(editor, &filepath);
}

fn run_editor(editor: &str, filepath: &str) {
    let mut editor_process = Command::new(editor)
        .arg(filepath)
        .spawn()
        .expect("failed open editor");

    editor_process.wait().expect("failed to run");
}

fn run_selector(selector: &str, dir: &str) -> String {
    let selector_process = Command::new(selector)
        .current_dir(dir)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed run selector command");

    let output = selector_process.wait_with_output().unwrap();
    let filename = from_utf8(&output.stdout).unwrap().to_string();

    filename.chars().filter(|c| c != &'\n').collect::<String>()
}
