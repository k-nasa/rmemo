[![Build Status](https://travis-ci.org/k-nasa/rmemo.svg?branch=master)](https://travis-ci.org/k-nasa/rmemo)
[![crate-name at crates.io](https://img.shields.io/crates/v/rmemo.svg)](https://crates.io/crates/rmemo)

# rmemo
rmemo is tool for taking notes fast on the command line

![demo](https://github.com/k-nasa/rmemo/blob/master/media/demo.gif)

## Installation
### On macOS
```
brew tap k-nasa/homebrew-rmemo
brew install rmemo
```

### From source
```
cargo install rmemo
```

## Usage
```
rmemo 0.1.3
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
    new       create new memo
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

## License
MIT

## Author
nasa
