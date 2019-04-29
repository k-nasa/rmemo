use crate::commands::run_editor;
use crate::config::Config;
use crate::utils::*;
use colored::*;
use std::fs::{copy, create_dir_all};
use std::io::Write;
use std::string::*;

pub fn cmd_new(matches: &ArgMatches, config: &Config) {
    let input_filepath = match matches.value_of("title") {
        Some(title) => title.to_string(),
        None => {
            print!("Input title :");
            std::io::stdout().flush().expect("print! is faild");
            let input: String = crate::utils::read();
            inflector::cases::snakecase::to_snake_case(&input)
        }
    };

    if input_filepath.is_empty() {
        println!("{}", "Title is required!!".red());
        return;
    }

    let mut dir = config.memos_dir().clone();
    let editor = config.editor();

    // The last is the file name, the other is the directory structure
    let mut element: Vec<&str> = input_filepath.split('/').collect();

    let title = element.last().unwrap().to_string();
    element.pop();

    for elm in element {
        dir.push('/');
        dir.push_str(&elm.to_string());
    }

    let title = match config.enter_time_in_filename {
        Some(true) => {
            let now = chrono::Local::now().format("%Y-%m-%d").to_string();
            format!("{}{}.md", now, title)
        }
        _ => format!("{}.md", title),
    };

    let filepath = format!("{}/{}", dir, title);
    create_dir_all(dir).expect("faild create directory");

    if matches.is_present("template") && !config.template_file_path().is_empty() {
        copy(config.template_file_path(), &filepath).expect("faild template file copy");
    }

    run_editor(editor, &filepath);
}

pub fn make_subcommand() -> App {
    SubCommand::with_name("new")
        .alias("n")
        .about("Create new memo")
        .arg(
            Arg::with_name("template")
                .help("Create based on template file")
                .short("t")
                .long("template"),
        )
        .arg(Arg::with_name("title").help("create file title"))
}
