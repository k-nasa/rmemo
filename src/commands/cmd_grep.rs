use clap::ArgMatches;
use colored::*;
use config::Config;
use std::fs::read_dir;
use std::process::Command;
use std::string::*;

pub fn cmd_grep(matches: &ArgMatches, config: &Config) {
    let argument = match matches.value_of("argument") {
        Some(argument) => argument,
        None => {
            println!("The following required arguments were not provided");
            return;
        }
    };

    let memo_dir = config.memos_dir();

    let files: Vec<String> = read_dir(memo_dir)
        .unwrap()
        .map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_string())
        .collect();

    if files.is_empty() {
        println!("{}", "file is nothing".yellow());
        return;
    }

    let mut grep_process = Command::new(config.grep_command())
        .arg(argument)
        .args(files)
        .spawn()
        .expect("faild run grep command");

    grep_process.wait().expect("failed to run");
}
