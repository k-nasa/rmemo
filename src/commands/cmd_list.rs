use clap::ArgMatches;
use config::Config;
use dir_tree::*;
use std::fs::read_dir;

pub fn cmd_list(matches: &ArgMatches, config: &Config) {
    // TODO Implementation of filtering by patter
    // let pattern = match matches.value_of("pattern") {
    //     Some(pattern) => pattern.to_string(),
    //     None => String::new(),
    // };
    //
    let is_short_view = matches.is_present("short-view");

    let memo_dir = config.memos_dir();

    let dir_tree = DirTree::new(memo_dir);
    if is_short_view {
        dir_tree.short_print();
    } else {
        dir_tree.print();
    }
}
