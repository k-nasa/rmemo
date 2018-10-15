use clap::ArgMatches;
use config::Config;
use std::fs::read_dir;

pub fn cmd_list(matches: &ArgMatches, config: &Config) {
    let pattern = match matches.value_of("pattern") {
        Some(pattern) => pattern.to_string(),
        None => String::new(),
    };

    let is_full_path = matches.is_present("full_path");

    let memo_dir = config.memos_dir();

    let files: Vec<String> = read_dir(memo_dir)
        .unwrap()
        .map(|dir_entry| {
            if is_full_path {
                dir_entry.unwrap().path().to_str().unwrap().to_string()
            } else {
                dir_entry.unwrap().file_name().into_string().unwrap()
            }
        })
        .filter(|c| c.contains(&pattern))
        .collect();

    for file in files {
        println!("{}", file);
    }
}
