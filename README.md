[![Build Status](https://travis-ci.org/k-nasa/memo_command.svg?branch=master)](https://travis-ci.org/k-nasa/memo_command)
[![crate-name at crates.io](https://img.shields.io/crates/v/rmemo.svg)](https://crates.io/crates/rmemo)

# Memo Command
CLI memo tool


## Usage
```
rmemo 0.1.1
k-nasa <htilcs1115@gmail.com>
CLI memo command

USAGE:
    rmemo [SUBCOMMAND]

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


SUBCOMMANDS:
    help      help
    config    edit config file
    delete    delete memos
    edit      edit memo
    grep      grep memos
    list      show memos list
```

## Installation

```
$cargo install rmemo
```

## Configuration
```
memos_dir = "/Users/asan/.config/rmemo/memos"
editor = "nvim"
selector = "fzf"
grep_command = "grep"
template_file_path = "./"
enter_time_in_filename = true
```

## License
MIT

## Author
nasa
