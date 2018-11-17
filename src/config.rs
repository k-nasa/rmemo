use crate::utils::home_dir_string;
use std::fs::*;
use std::io::prelude::*;
use std::io::Result;
use std::path::*;
use std::str::from_utf8;

#[derive(Deserialize, Serialize, Debug)]
/// Structure that loads setting information from file and mapping
pub struct Config {
    memos_dir: Option<String>,
    editor: Option<String>,
    selector: Option<String>,
    grep_command: Option<String>,
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
            Some(dir) => Path::new(&dir.to_str().unwrap().to_string()).join(".config/rmemo/"),
            None => panic!("faild fetch home_dir name"),
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

    /// Unwrap and return the memo_dir property
    pub fn memos_dir(&self) -> &String {
        match self.memos_dir {
            Some(ref dir) => dir,
            None => panic!("Memos directory is not set"),
        }
    }

    /// Unwrap and return the enter_time_in_filename property
    pub fn enter_time_in_filename(&self) -> bool {
        match self.enter_time_in_filename {
            Some(true) => true,
            _ => false,
        }
    }

    /// Unwrap and return the editor property
    pub fn editor(&self) -> &String {
        match self.editor {
            Some(ref editor) => editor,
            None => panic!("Editor is not set"),
        }
    }

    /// Unwrap and return the selector property
    pub fn selector(&self) -> &String {
        match self.selector {
            Some(ref selector) => selector,
            None => panic!("Editor is not set"),
        }
    }

    /// Unwrap and return the grep_command property
    pub fn grep_command(&self) -> &String {
        match self.grep_command {
            Some(ref grep_command) => grep_command,
            None => panic!("grep_command is not set"),
        }
    }

    /// Unwrap and return the template_file_path property
    pub fn template_file_path(&self) -> &String {
        match self.template_file_path {
            Some(ref template_file_path) => template_file_path,
            None => panic!("template_file_path is not set"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let memos_dir = Some(home_dir_string() + "/.config/rmemo/memos");
        let editor = Some(String::from("vim"));
        let selector = Some(String::from("fzf"));
        let grep_command = Some(String::from("grep"));
        let template_file_path = Some(String::from(""));
        let enter_time_in_filename = Some(true);

        Config {
            memos_dir,
            editor,
            selector,
            grep_command,
            template_file_path,
            enter_time_in_filename,
        }
    }
}
