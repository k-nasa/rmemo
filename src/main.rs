use std::fs::*;
use std::io::prelude::*;
use std::io::stderr;
use std::io::Result;
use std::path::*;
use std::str::from_utf8;

extern crate dirs;
extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate toml;

fn main() {
    let config = match Config::load_config() {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
}

pub struct Config;

/// Read the file in which the setting file is described.
/// If not, create it
impl Config {
    fn load_config() -> Result<Config> {
        //FIXME Not compatible with windows
        let dir = match dirs::home_dir() {
            Some(dir) => Path::new(&dir.to_str().unwrap().to_string()).join(".config/memo/"),
            _ => Path::new("./").join(".config/memo/"),
        };

        let filepath = &dir.join("config.toml");

        // will create it every time, but it may be easy
        let file = File::create(filepath);

        Ok(Config)
    }
}
