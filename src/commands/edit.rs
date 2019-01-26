use super::{run_editor, run_selector};
use crate::config::Config;
use crate::utils::*;

pub fn cmd_edit(matches: &ArgMatches, config: &Config) {
    let dir = config.memos_dir();
    let selector = config.selector();

    let title = if let Some(title) = matches.value_of("title") {
        title.to_string()
    } else {
        run_selector(&selector, dir)
    };

    if title.is_empty() {
        println!("File is not selected!");
        return;
    }

    let editor = config.editor();
    let filepath = format!("{}/{}", dir, title);

    run_editor(editor, &filepath);
}

pub fn make_subcommand() -> App {
    SubCommand::with_name("edit")
        .alias("e")
        .about("Edit memo")
        .arg(Arg::with_name("title").help("edit file title"))
}
