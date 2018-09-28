pub mod commands;
pub mod config;
pub mod utils;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

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

        ("delete", Some(_)) => cmd_delete(),

        ("edit", Some(matches)) => cmd_edit(matches, &config),

        ("list", Some(_)) => cmd_list(),

        ("new", Some(matches)) => cmd_new(matches, &config),

        _ => {
            app.print_long_help().ok();
            return;
        }
    };
}
