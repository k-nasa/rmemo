use colored::*;
use std::fs::read_dir;

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
                let mut tree_branches = branch.clone();
                if len == i + 1 {
                    tree_branches.push(TreeBranch::Corner);
                } else {
                    tree_branches.push(TreeBranch::Edge);
                };

                File {
                    path: path.to_string(),
                    name,
                    tree_branches,
                    is_last: len == i + 1,
                }
            })
            .collect();

        let mut dir_tree: Vec<DirTree> = Vec::new();
        if !dirs.is_empty() {
            let len = dirs.len();
            for (i, dir) in dirs.iter().enumerate() {
                let mut tree_branches = branch.clone();
                tree_branches.push(TreeBranch::Line);
                dir_tree.push(DirTree::_new(&dir, len == i + 1, tree_branches));
            }
        };

        let dir_name = path2name(root_dir);

        DirTree {
            dir_tree,
            dir_name,
            files,
            is_last,
            dir_path: root_dir.to_string(),
            tree_branches: branch,
        }
    }

    pub fn print(self) {
        if self.tree_branches.is_empty() {
            println!("{}", self.dir_path.yellow());
        } else {
            for branch in self.tree_branches {
                print!("{}", branch.tree_branch_string());
            }
            println!("{}", self.dir_name.cyan());
        }

        for dir_tree in self.dir_tree {
            dir_tree.print();
        }

        for file in self.files {
            for branch in file.tree_branches {
                print!("{}", branch.tree_branch_string());
            }
            println!(" {}", file.name);
        }
    }

    pub fn short_print(self) {
        println!("{}", self.dir_path.yellow());

        for dir in self.dir_tree {
            for branch in dir.tree_branches {
                print!("{}", branch.tree_branch_string());
            }
            println!(" {}{}", dir.dir_name.cyan(), "/".cyan());
        }

        for file in self.files {
            for branch in file.tree_branches {
                print!("{}", branch.tree_branch_string());
            }
            println!(" {}", file.name);
        }
    }
}

fn path2name(path: &str) -> String {
    let name: Vec<&str> = path.split('/').collect();
    let name = name.last().unwrap().to_string();
    name
}
