use std::fs::*;
use std::io::prelude::*;
use std::io::Result;
use std::path::*;
use std::str::from_utf8;

extern crate dirs;
extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("help").alias("h").about("help"))
        .subcommand(
            SubCommand::with_name("config")
                .alias("c")
                .about("edit config file"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("l")
                .about("show memos list"),
        )
        .subcommand(SubCommand::with_name("edit").alias("e").about("edit memo"))
        .subcommand(
            SubCommand::with_name("delete")
                .alias("d")
                .about("delete memos"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .alias("n")
                .about("create new memo"),
        );

    let matches = app.get_matches();

    let config = match Config::load_config() {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("{:?}", config);
}

#[derive(Deserialize, Serialize, Debug)]
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

        let config: Config = if toml_str.len() == 0 {
            let config = Config::default();
            let toml_str = toml::to_string(&config).unwrap();

            match file.write_all(toml_str.as_bytes()) {
                Ok(_) => config,
                Err(e) => panic!(e),
            }
        } else {
            match toml::from_str(toml_str) {
                Ok(config) => config,
                Err(e) => panic!(e),
            }
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

        match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .read(true)
            .open(filepath)
        {
            Ok(file) => file,
            Err(e) => panic!(e),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let memos_dir = Some(String::from("./memos/"));
        let editor = Some(String::from("vim"));
        let template_file_path = Some(String::from("./"));

        Config {
            memos_dir,
            editor,
            template_file_path,
        }
    }
}
