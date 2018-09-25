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

    println!("{:?}", config);
}

#[derive(Deserialize, Debug)]
pub struct Config {
    memos_dir: Option<String>,
    editor: Option<String>,
    template_file_path: Option<String>,
}

/// Read the file in which the setting file is described.
/// If not, create it
impl Config {
    fn load_config() -> Result<Config> {
        let mut file = Config::load_or_create_file();

        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let toml_str = match from_utf8(&buf) {
            Ok(toml_str) => toml_str,
            Err(e) => panic!(e),
        };

        if toml_str.len() == 0 {
            //TODO write default config to config.toml
        }

        let config: Config = match toml::from_str(toml_str) {
            Ok(config) => config,
            Err(e) => panic!(e),
        };

        Ok(config)
    }

    ///Get the file pointer of the setting file.
    ///When there is no file, a setting file is created.
    fn load_or_create_file() -> File {
        //FIXME Not compatible with windows
        let dir = match dirs::home_dir() {
            Some(dir) => Path::new(&dir.to_str().unwrap().to_string()).join(".config/memo/"),
            _ => Path::new("./").join(".config/memo/"),
        };

        let filepath = &dir.join("config.toml");

        match File::open(filepath) {
            Ok(file) => file,
            Err(_) => File::create(filepath).unwrap(),
        }
    }
}
