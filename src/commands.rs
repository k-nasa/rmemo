extern crate chrono;
extern crate dirs;
extern crate serde;
extern crate termion;
extern crate toml;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;
use config::Config;
use std::fs::remove_file;
use std::fs::{copy, create_dir_all, read_dir, DirBuilder};
use std::io::*;
use std::path::*;
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::string::*;
use termion::event::{Event, Key};
use termion::input::TermRead;
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
            SubCommand::with_name("delete")
                .alias("d")
                .about("delete memos")
                .arg(Arg::with_name("pattern").help("Pattern search")),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .alias("e")
                .about("edit memo")
                .arg(Arg::with_name("title").help("edit file title")),
        )
        .subcommand(
            SubCommand::with_name("grep")
                .alias("g")
                .about("grep memos")
                .arg(
                    Arg::with_name("argument")
                        .help("grep command argument")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("l")
                .about("show memos list")
                .arg(Arg::with_name("pattern").help("Pattern search"))
                .arg(
                    Arg::with_name("full_path")
                        .help("full_path")
                        .short("f")
                        .long("full_path"),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .alias("n")
                .about("create new memo")
                .arg(
                    Arg::with_name("template")
                        .help("create based on template file")
                        .short("t")
                        .long("template"),
                )
                .arg(Arg::with_name("title").help("create file title")),
        )
}

pub fn cmd_config(config: &Config) {
    let dir = match dirs::home_dir() {
        Some(dir) => Path::new(&dir.to_str().unwrap().to_string()).join(".config/rmemo/"),
        _ => Path::new("./").join(".config/rmemo/"),
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

pub fn cmd_delete(matches: &ArgMatches, config: &Config) {
    let pattern = match matches.value_of("pattern") {
        Some(pattern) => pattern.to_string(),
        None => String::new(),
    };

    let memo_dir = config.memos_dir();
    create_dir_all(memo_dir).expect("faild create memos_dir");

    let full_path_files: Vec<String> = read_dir(memo_dir)
        .unwrap()
        .map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_string())
        .filter(|c| c.contains(&pattern))
        .collect();

    let files: Vec<String> = read_dir(memo_dir)
        .unwrap()
        .map(|dir_entry| dir_entry.unwrap().file_name().into_string().unwrap())
        .filter(|c| c.contains(&pattern))
        .collect();

    if files.is_empty() {
        println!("{}", "No matched file".yellow());
        return;
    }

    for file in files.clone() {
        println!("{}", file);
    }

    println!("{}", "Will delete those entry. Are you sure?".red());
    println!("Are you sure?(y/n) :");

    match stdin().events().nth(0).unwrap().unwrap() {
        Event::Key(Key::Char('y')) => (),
        Event::Key(Key::Char('Y')) => (),
        _ => return,
    }

    println!("Really?(y/n) :");
    match stdin().events().nth(0).unwrap().unwrap() {
        Event::Key(Key::Char('y')) => (),
        Event::Key(Key::Char('Y')) => (),
        _ => return,
    }

    for file in full_path_files {
        remove_file(file).expect("failed remove files");
    }

    println!("{}", "All file delete".green());
}

pub fn cmd_edit(matches: &ArgMatches, config: &Config) {
    let mut title = match matches.value_of("title") {
        Some(title) => title.to_string(),
        None => String::new(),
    };

    let dir = config.memos_dir();
    create_dir_all(dir).expect("faild create memos_dir");

    let selector = config.selector();
    if title.is_empty() {
        title = run_selector(&selector, dir);
    }

    if title.is_empty() {
        println!("File is not selected!");
        return;
    }

    let editor = config.editor();
    let filepath = format!("{}/{}", dir, title);

    run_editor(editor, &filepath);
}

pub fn cmd_grep(matches: &ArgMatches, config: &Config) {
    let argument = match matches.value_of("argument") {
        Some(argument) => argument,
        None => {
            println!("The following required arguments were not provided");
            return;
        }
    };

    let memo_dir = config.memos_dir();

    let files: Vec<String> = read_dir(memo_dir)
        .unwrap()
        .map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_string())
        .collect();

    if files.is_empty() {
        println!("{}", "file is nothing".yellow());
        return;
    }

    let mut grep_process = Command::new(config.grep_command())
        .arg(argument)
        .args(files)
        .spawn()
        .expect("faild run grep command");

    grep_process.wait().expect("failed to run");
}

pub fn cmd_list(matches: &ArgMatches, config: &Config) {
    let pattern = match matches.value_of("pattern") {
        Some(pattern) => pattern.to_string(),
        None => String::new(),
    };

    let is_full_path = matches.is_present("full_path");

    let memo_dir = config.memos_dir();
    create_dir_all(memo_dir).expect("faild create memos_dir");

    let files: Vec<String> = read_dir(memo_dir)
        .unwrap()
        .map(|dir_entry| {
            if is_full_path {
                dir_entry.unwrap().path().to_str().unwrap().to_string()
            } else {
                dir_entry.unwrap().file_name().into_string().unwrap()
            }
        })
        .filter(|c| c.contains(&pattern))
        .collect();

    for file in files {
        println!("{}", file);
    }
}

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

    create_dir_all(dir).expect("faild create memos_dir");

    if matches.is_present("template") && !config.template_file_path().is_empty() {
        copy(config.template_file_path(), &filepath).expect("faild template file copy");
    }

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
