use clap::ArgMatches;
use crate::config::Config;
use crate::dir_tree::*;

pub fn cmd_list(matches: &ArgMatches, config: &Config) {
    // TODO Implementation of filtering by patter
    // let pattern = match matches.value_of("pattern") {
    //     Some(pattern) => pattern.to_string(),
    //     None => String::new(),
    // };

    let memo_dir = config.memos_dir();
    let dir_tree = DirTree::new(memo_dir);

    if matches.is_present("short-view") {
        dir_tree.short_print();
    } else {
        dir_tree.print();
    }
}
