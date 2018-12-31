use crate::config::Config;
use crate::dir_tree::DirTree;
use clap::{App, SubCommand};
use failure::Error;
use mdbook::MDBook;
use rocket_contrib::serve::StaticFiles;
use std::fs::*;
use std::io::Write;
use std::path::PathBuf;
use std::path::*;

pub fn make_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("serve")
        .alias("s")
        .about("start http server")
}

const BOOK_TOML_PREFIX: &str = "[book]\n";

pub fn cmd_serve(config: &Config) {
    // TODO configに入れるように変更
    let book_dir = Path::new(config.memos_dir()).parent().unwrap();
    let memo_dir = config.memos_dir();

    make_book_toml(book_dir, memo_dir).unwrap();
    make_summary_md(memo_dir).unwrap();

    let book = MDBook::load(book_dir).unwrap();
    book.build().unwrap();

    rocket::ignite()
        .mount("/", StaticFiles::from("/Users/asan/.config/rmemo/book"))
        .launch();
}

fn make_book_toml(
    project_dir: impl Into<PathBuf>,
    memo_dir: impl Into<PathBuf>,
) -> Result<(), Error> {
    let book_toml = project_dir.into().join("book.toml");
    let memo_dir = memo_dir.into();

    if book_toml.exists() {
        return Ok(());
    }

    let book_toml_string = format!(
        "{}\nsrc = \"{}\"",
        BOOK_TOML_PREFIX,
        memo_dir.to_str().unwrap()
    );

    File::create(book_toml)?.write_all(book_toml_string.as_bytes())?;

    Ok(())
}

fn make_summary_md(memo_dir: &str) -> Result<(), Error> {
    let summary = Path::new(memo_dir).join("SUMMARY.md");
    let dir_tree = DirTree::new(memo_dir);
    let files = dir_tree.files_list();

    if summary.exists() {
        // Initialize
        std::fs::remove_file(&summary)?;
    }

    let mut summary_links = Vec::new();
    for file in files {
        // SUMMARY.md is not needed for preview
        if file.name == "SUMMARY.md".to_string() {
            continue;
        }

        let link_string = format!("- [{}]({})\n", file.name, file.path);
        summary_links.push(link_string);
    }

    let summary_links = summary_links.into_iter().collect::<String>();
    File::create(summary)?.write_all(summary_links.as_bytes())?;

    Ok(())
}
