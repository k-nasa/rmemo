use chrono::prelude::*;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::fs::*;
use std::io::prelude::*;
use std::io::Result;
use std::path::*;
use std::process::{Command, Stdio};
use std::str::from_utf8;

extern crate chrono;
extern crate dirs;
extern crate serde;
extern crate toml;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

fn main() {
    run(build_app());
}

fn run(mut app: clap::App) {
    let config = match Config::load_config() {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match app.clone().get_matches().subcommand() {
        ("config", Some(_)) => cmd_config(),

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

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    memos_dir: Option<String>,
    editor: Option<String>,
    template_file_path: Option<String>,
    enter_time_in_filename: Option<bool>,
}

impl Config {
    /// Read the file in which the setting file is described.
    /// If not, create it
    pub fn load_config() -> Result<Config> {
        let mut file = Config::load_or_create_file();

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
    fn load_or_create_file() -> File {
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

fn build_app() -> clap::App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
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
        .subcommand(
            SubCommand::with_name("edit")
                .alias("e")
                .about("edit memo")
                .arg(Arg::with_name("title").help("edit file title")),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .alias("d")
                .about("delete memos"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .alias("n")
                .about("create new memo")
                .arg(Arg::with_name("title").help("create file title")),
        )
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn home_dir_string() -> String {
    match dirs::home_dir() {
        Some(dir) => dir.to_str().unwrap().to_string(),
        _ => panic!("Home directory is not set"),
    }
}

fn cmd_config() {}
fn cmd_delete() {}

fn cmd_edit(matches: &ArgMatches, config: &Config) {
    let mut title = match matches.value_of("title") {
        Some(title) => title.to_string(),
        None => String::new(),
    };

    let dir = config.memos_dir();
    create_dir_all(dir).expect("faild create memos_dir");

    if title.is_empty() {
        title = run_selector(&"fzf".to_string(), dir);
    }

    if title.is_empty() {
        println!("File is not selected!");
        return;
    }

    let editor = config.editor();
    let filepath = format!("{}/{}", dir, title);

    run_editor(editor, &filepath);
}

fn cmd_list() {}

fn cmd_new(matches: &ArgMatches, config: &Config) {
    let title = match matches.value_of("title") {
        Some(title) => title.to_string(),
        None => {
            println!("Input title :");
            read::<String>()
        }
    };

    let dir = config.memos_dir();
    let editor = config.editor();

    let title = match config.enter_time_in_filename {
        Some(true) => {
            let now = Local::now().format("%Y-%m-%d").to_string();
            format!("{}{}.md", now, title)
        }
        _ => format!("{}.md", title),
    };

    let filepath = format!("{}/{}", dir, title);

    create_dir_all(dir).expect("faild create memos_dir");

    run_editor(editor, &filepath);
}

fn run_editor(editor: &str, filepath: &str) {
    let mut editor_process = Command::new(editor)
        .arg(filepath)
        .spawn()
        .expect("failed open editor");

    editor_process.wait().expect("failed to run");
}

fn run_selector(selector: &str, dir: &str) -> String {
    let selector_process = Command::new(selector)
        .current_dir(dir)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed run selector command");

    let output = selector_process.wait_with_output().unwrap();
    let filename = from_utf8(&output.stdout).unwrap().to_string();

    filename.chars().filter(|c| c != &'\n').collect::<String>()
}
