[![CircleCI](https://circleci.com/gh/k-nasa/rmemo.svg?style=svg)](https://circleci.com/gh/k-nasa/rmemo)
[![crate-name at crates.io](https://img.shields.io/crates/v/rmemo.svg)](https://crates.io/crates/rmemo)

# rmemo
rmemo is tool for taking notes fast on the command line

[![demo](https://asciinema.org/a/F65yWTfdnMyNgiyFhLjGO3mRZ.png)](https://asciinema.org/a/F65yWTfdnMyNgiyFhLjGO3mRZ)

## Installation

```
cargo install rmemo
```

## Usage
```
USAGE:
    rmemo [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    config    Edit config file
    delete    Delete memos
    edit      Edit memo
    grep      Grep memos
    list      Show memos list
    new       Create new memo
    serve     start http server
    help      Prints this message or the help of the given subcommand(s)
```

## Usage example
Create new note
```
$rmemo new
Title:
```
If you have set up a template, create a note based on it
```
$rmemo new -t
Title:
```
Deletes the note of the file name matching the argument pattern
```
rmemo delete hoge
hoge
2018-10-15hoge.md
2018-10-12hoge.md
2018-10-08hoge.md
Will delete those entry. Are you sure?
Are you sure?(y/n) :
```

## Configuration
```toml
memos_dir = "/Users/asan/.config/rmemo/memos"  # Directory where note is stored
editor = "nvim"                                # The editor you want to use. I recommend nvim for no particular reason
selector = "fzf"                               # Selector you want to use. Please choose your favorite one
grep_command = "grep"                          # Set your favorite grep
template_file_path = ""                        # Set the template you want to use
enter_time_in_filename = true                  # Set it to false if timestamp is not required for file name
```

## Author
nasa
