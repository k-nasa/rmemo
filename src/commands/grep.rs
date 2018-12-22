use crate::config::Config;
use crate::file_or_dir::{file_paths, FileOrDir, FileOrDirs};
use clap::ArgMatches;
use clap::{App, Arg, SubCommand};
use colored::*;
use std::process::Command;

pub fn cmd_grep(matches: &ArgMatches, config: &Config) {
    let argument = match matches.value_of("argument") {
        Some(argument) => argument,
        None => {
            println!("The following required arguments were not provided");
            return;
        }
    };

    let memo_dir = config.memos_dir();
    let files: FileOrDirs = FileOrDir::files(&memo_dir);

    if files.is_empty() {
        println!("{}", "file is nothing".yellow());
        return;
    }

    let mut grep_process = Command::new(config.grep_command())
        .arg(argument)
        .args(file_paths(&files))
        .spawn()
        .expect("faild run grep command");

    grep_process.wait().expect("failed to run");
}

pub fn make_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("grep")
        .alias("g")
        .about("Grep memos")
        .arg(
            Arg::with_name("argument")
                .help("Grep command argument")
                .required(true),
        )
}
