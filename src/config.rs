extern crate chrono;
extern crate dirs;
extern crate serde;
extern crate toml;

use std::fs::*;
use std::io::prelude::*;
use std::io::Result;
use std::path::*;
use std::str::from_utf8;
use utils::home_dir_string;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    memos_dir: Option<String>,
    editor: Option<String>,
    template_file_path: Option<String>,
    pub enter_time_in_filename: Option<bool>,
}

impl Config {
    /// Read the file in which the setting file is described.
    /// If not, create it
    pub fn load_config() -> Result<Config> {
        let mut file = Config::load_or_create_config_file();

        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let toml_str = match from_utf8(&buf) {
            Ok(toml_str) => toml_str,
            Err(e) => panic!(e),
        };

        let config: Config = if toml_str.is_empty() {
            let config = Config::default();
            let toml_str = toml::to_string(&config).unwrap();

            match file.write_all(toml_str.as_bytes()) {
                Ok(_) => config,
                Err(e) => panic!(e),
            }
        } else {
            match toml::from_str(toml_str) {
                Ok(config) => config,
                _ => panic!("Analysis of configuration file failed"),
            }
        };

        Ok(config)
    }

    ///Get the file pointer of the setting file.
    ///When there is no file, a setting file is created.
    pub fn load_or_create_config_file() -> File {
        //FIXME Not compatible with windows
        let dir = match dirs::home_dir() {
            Some(dir) => Path::new(&dir.to_str().unwrap().to_string()).join(".config/rsmemo/"), // Change path as test
            _ => Path::new("./").join(".config/memo/"),
        };

        DirBuilder::new()
            .recursive(true)
            .create(dir.clone())
            .unwrap();

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

    pub fn memos_dir(&self) -> &String {
        match self.memos_dir {
            Some(ref dir) => dir,
            None => panic!("Memos directory is not set"),
        }
    }

    pub fn enter_time_in_filename(&self) -> bool {
        match self.enter_time_in_filename {
            Some(true) => true,
            _ => false,
        }
    }

    pub fn editor(&self) -> &String {
        match self.editor {
            Some(ref editor) => editor,
            None => panic!("Editor is not set"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let memos_dir = Some(home_dir_string() + "/.config/memo/memos");
        let editor = Some(String::from("vim"));
        let template_file_path = Some(String::from("./")); //FIXME
        let enter_time_in_filename = Some(true);

        Config {
            memos_dir,
            editor,
            template_file_path,
            enter_time_in_filename,
        }
    }
}
