use crate::config::Config;
use crate::dir_tree::*;
use clap::ArgMatches;

pub fn cmd_list(matches: &ArgMatches, config: &Config) {
    let memo_dir = config.memos_dir();
    let dir_tree = DirTree::new(memo_dir);

    if matches.is_present("short-view") {
        dir_tree.short_print();
    } else {
        dir_tree.print();
    }
}
