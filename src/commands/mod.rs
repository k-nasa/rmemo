use clap::{App, AppSettings};
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::string::*;

pub mod config;
pub mod delete;
pub mod edit;
pub mod grep;
pub mod list;
pub mod new;

pub fn build_app() -> App<'static, 'static> {
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
