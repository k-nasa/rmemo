pub mod commands;
pub mod config;
pub mod utils;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate colored;
extern crate termion;

use commands::*;
use config::Config;

pub fn run() {
    let mut app = build_app();
    let config = match Config::load_config() {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match app.clone().get_matches().subcommand() {
        ("config", Some(_)) => cmd_config(&config),

        ("delete", Some(matches)) => cmd_delete(matches, &config),

        ("edit", Some(matches)) => cmd_edit(matches, &config),

        ("grep", Some(matches)) => cmd_grep(matches, &config),

        ("list", Some(matches)) => cmd_list(matches, &config),

        ("new", Some(matches)) => cmd_new(matches, &config),

        _ => {
            app.print_long_help().ok();
            return;
        }
    };
}
