use super::run_editor;
use clap::ArgMatches;
use config::Config;

pub fn cmd_quick(matches: &ArgMatches, config: &Config) {
    match matches.value_of("your idea") {
        Some(idea) => append_idea_to_file(idea, config),
        None => open_idea_file_with_editor(config),
    }
}

fn append_idea_to_file(idea: &str, config: &Config) {}

fn open_idea_file_with_editor(config: &Config) {}
