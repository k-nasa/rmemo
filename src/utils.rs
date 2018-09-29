extern crate dirs;

use std::io::*;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn home_dir_string() -> String {
    match dirs::home_dir() {
        Some(dir) => dir.to_str().unwrap().to_string(),
        _ => panic!("Home directory is not set"),
    }
}
