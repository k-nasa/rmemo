use clap::{App, AppSettings, Arg, SubCommand};
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::string::*;

pub mod cmd_config;
pub mod cmd_delete;
pub mod cmd_edit;
pub mod cmd_grep;
pub mod cmd_list;
pub mod cmd_new;
pub mod cmd_quick;

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
                .about("Edit config file"),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .alias("d")
                .about("Delete memos")
                .arg(Arg::with_name("pattern").help("Pattern search")),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .alias("e")
                .about("Edit memo")
                .arg(Arg::with_name("title").help("edit file title")),
        )
        .subcommand(
            SubCommand::with_name("grep")
                .alias("g")
                .about("Grep memos")
                .arg(
                    Arg::with_name("argument")
                        .help("Grep command argument")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("l")
                .about("Show memos list")
                .arg(Arg::with_name("pattern").help("Pattern search"))
                .arg(
                    Arg::with_name("short view")
                        .help("Shallow the directory structure")
                        .short("s")
                        .long("short-view"),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .alias("n")
                .about("Create new memo")
                .arg(
                    Arg::with_name("template")
                        .help("Create based on template file")
                        .short("t")
                        .long("template"),
                )
                .arg(Arg::with_name("title").help("create file title")),
        )
        .subcommand(
            SubCommand::with_name("quick")
                .alias("q")
                .about("Fast memo not to forget idea")
                .arg(Arg::with_name("your idea").help("Input your idea")),
        )
}

fn run_editor(editor: &str, filepath: &str) {
    let mut editor_process = Command::new(editor)
        .arg(filepath)
        .spawn()
        .expect("Failed open editor");

    editor_process.wait().expect("Failed to run");
}

fn run_selector(selector: &str, dir: &str) -> String {
    let selector_process = Command::new(selector)
        .current_dir(dir)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed run selector command");

    let output = selector_process.wait_with_output().unwrap();
    let filename = from_utf8(&output.stdout).unwrap().to_string();

    filename.chars().filter(|c| c != &'\n').collect::<String>()
}
