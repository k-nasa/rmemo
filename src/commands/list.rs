use crate::config::Config;
use crate::dir_tree::*;
use clap::ArgMatches;
use clap::{App, Arg, SubCommand};

pub fn cmd_list(matches: &ArgMatches, config: &Config) {
    let memo_dir = config.memos_dir();
    let dir_tree = DirTree::new(memo_dir);

    if matches.is_present("short-view") {
        dir_tree.short_print();
    } else {
        dir_tree.print();
    }
}

pub fn make_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("list")
        .alias("l")
        .about("Show memos list")
        .arg(Arg::with_name("pattern").help("Pattern search"))
        .arg(
            Arg::with_name("short-view")
                .help("Shallow the directory structure")
                .short("s")
                .long("short-view"),
        )
}
