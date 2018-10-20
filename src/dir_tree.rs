use std::fs::DirEntry;
use std::fs::File;

struct DirTree {
    dir_tree: Vec<DirTree>,
    files: Vec<File>,
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
