use std::fs::DirEntry;
use std::fs::File;

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
