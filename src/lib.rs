pub mod commands;
pub mod config;
pub mod dir_tree;
pub mod file_or_dir;
pub mod utils;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate colored;
extern crate dialoguer;
extern crate termion;

use crate::commands::build_app;
use crate::commands::cmd_config::cmd_config;
use crate::commands::cmd_delete::cmd_delete;
use crate::commands::cmd_edit::cmd_edit;
use crate::commands::cmd_grep::cmd_grep;
use crate::commands::cmd_list::cmd_list;
use crate::commands::cmd_new::cmd_new;
use crate::commands::cmd_quick::cmd_quick;
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

    match app.clone().get_matches().subcommand() {
        ("config", Some(_)) => cmd_config(&config),

        ("delete", Some(matches)) => cmd_delete(matches, &config),

        ("edit", Some(matches)) => cmd_edit(matches, &config),

        ("grep", Some(matches)) => cmd_grep(matches, &config),

        ("list", Some(matches)) => cmd_list(matches, &config),

        ("new", Some(matches)) => cmd_new(matches, &config),

        ("quick", Some(matches)) => cmd_quick(matches, &config),

        _ => {
            app.print_long_help().ok();
            return;
        }
    };
}
