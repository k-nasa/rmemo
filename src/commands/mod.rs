use crate::commands::{
    config::cmd_config, delete::cmd_delete, edit::cmd_edit, grep::cmd_grep, list::cmd_list,
    new::cmd_new,
};
use crate::config::Config;
use clap::{App, AppSettings};
use std::fs::create_dir_all;
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::string::*;

pub mod config;
pub mod delete;
pub mod edit;
pub mod grep;
pub mod list;
pub mod new;

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ColoredHelp)
        .subcommand(config::make_subcommand())
        .subcommand(delete::make_subcommand())
        .subcommand(edit::make_subcommand())
        .subcommand(grep::make_subcommand())
        .subcommand(list::make_subcommand())
        .subcommand(new::make_subcommand())
}

pub fn run() {
    let mut app = build_app();
    let config = match Config::load_config() {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let memo_dir = config.memos_dir();
    create_dir_all(memo_dir).expect("faild create memos_dir");

    match build_app().get_matches().subcommand() {
        ("config", Some(_)) => cmd_config(&config),

        ("delete", Some(matches)) => cmd_delete(matches, &config),

        ("edit", Some(matches)) => cmd_edit(matches, &config),

        ("grep", Some(matches)) => cmd_grep(matches, &config),

        ("list", Some(matches)) => cmd_list(matches, &config),

        ("new", Some(matches)) => cmd_new(matches, &config),

        _ => app.print_help().expect("faild print help"),
    };
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
