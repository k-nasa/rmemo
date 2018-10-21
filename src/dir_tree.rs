use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::*;
use std::io::Result;

#[derive(Debug)]
pub struct DirTree {
    dir_path: String,
    dir_name: String,
    dir_tree: Vec<DirTree>,
    files: Vec<File>,
    tree_branches: Vec<TreeBranch>,
    is_last: bool,
}

#[derive(Debug)]
pub struct File {
    name: String,
    path: String,
    tree_branches: Vec<TreeBranch>,
    is_last: bool,
}

#[derive(Debug, Clone)]
pub enum TreeBranch {
    Edge,
    Line,
    Corner,
    Blank,
}

impl TreeBranch {
    pub fn tree_branch_string(&self) -> &'static str {
        match *self {
            TreeBranch::Edge => "├──",
            TreeBranch::Line => "│  ",
            TreeBranch::Corner => "└──",
            TreeBranch::Blank => "   ",
        }
    }
}

impl DirTree {
    pub fn new(root_dir: &str) -> Self {
        DirTree::_new(root_dir, false, Vec::new())
    }

    fn _new(root_dir: &str, is_last: bool, branch: Vec<TreeBranch>) -> Self {
    }
}

fn path2name(path: &str) -> String {
    let name: Vec<&str> = path.split('/').collect();
    let name = name.last().unwrap().to_string();
    name
}
