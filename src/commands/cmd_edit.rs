use super::{run_editor, run_selector};
use crate::config::Config;
use clap::ArgMatches;
use clap::{App, Arg, SubCommand};

pub fn cmd_edit(matches: &ArgMatches, config: &Config) {
    let mut title = match matches.value_of("title") {
        Some(title) => title.to_string(),
        None => String::new(),
    };

    let dir = config.memos_dir();
    let selector = config.selector();

    if title.is_empty() {
        title = run_selector(&selector, dir);
    }

    if title.is_empty() {
        println!("File is not selected!");
        return;
    }

    let editor = config.editor();
    let filepath = format!("{}/{}", dir, title);

    run_editor(editor, &filepath);
}

pub fn make_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("edit")
        .alias("e")
        .about("Edit memo")
        .arg(Arg::with_name("title").help("edit file title"))
}
