pub mod commands;
pub mod config;
pub mod dir_tree;
pub mod file_or_dir;
pub mod utils;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use crate::commands::{
    build_app, config::cmd_config, delete::cmd_delete, edit::cmd_edit, grep::cmd_grep,
    list::cmd_list, new::cmd_new,
};

use crate::config::Config;
use std::fs::create_dir_all;

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
