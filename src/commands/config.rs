use super::run_editor;
use crate::config::Config;
use crate::utils::*;
use std::path::Path;

pub fn cmd_config(config: &Config) {
    let dir = match dirs::home_dir() {
        Some(dir) => Path::new(&dir.to_str().unwrap()).join(".config/rmemo/"),
        None => panic!("faild fetch home_dir name"),
    };

    let filepath = &dir.join("config.toml");
    let filepath = filepath.to_str().unwrap();

    let editor = config.editor();
    run_editor(editor, filepath);
}

pub fn make_subcommand() -> App {
    SubCommand::with_name("config")
        .alias("c")
        .about("Edit config file")
}
