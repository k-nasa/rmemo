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
        let mut file_paths: Vec<String> = Vec::new();
        let mut dirs: Vec<String> = Vec::new();

        for entry in read_dir(root_dir).unwrap() {
            let entry = entry.unwrap();
            let file_type = entry.file_type().unwrap();
            if file_type.is_dir() {
                dirs.push(entry.path().to_str().unwrap().to_string());
            }
            if file_type.is_file() {
                file_paths.push(entry.path().to_str().unwrap().to_string());
            }
        }

        let len = file_paths.len();
        let files: Vec<File> = file_paths
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let name = path2name(path);

                File {
                    path: path.to_string(),
                    name,
                    tree_branches: Vec::new(),
                    is_last: len == i + 1,
                }
            })
            .collect();
    }
}

fn path2name(path: &str) -> String {
    let name: Vec<&str> = path.split('/').collect();
    let name = name.last().unwrap().to_string();
    name
}
