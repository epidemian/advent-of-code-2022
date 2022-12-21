use std::collections::HashMap;

// Another attempt at day 7, trying to use a "more reified" data structure for
// the file system, where directories and files are represented using a tree
// structure.
#[allow(dead_code)]
pub fn run(input: &str) -> String {
    let root = parse_fs_from_terminal_output(input);

    let unused_space = 70_000_000 - root.size();
    let space_to_free_up = 30_000_000 - unused_space;

    let mut small_dirs_total_size = 0;
    let mut file_to_delete_size = usize::MAX;

    root.walk(&mut |node| {
        if let FsNode::Dir(..) = node {
            let size = node.size();
            if size <= 100_000 {
                small_dirs_total_size += size;
            }
            if size >= space_to_free_up && size < file_to_delete_size {
                file_to_delete_size = size;
            }
        }
    });

    format!("{small_dirs_total_size} {file_to_delete_size}")
}

enum FsNode {
    File(File),
    Dir(Dir),
}

struct File {
    size: usize,
}

struct Dir {
    children: HashMap<String, FsNode>,
}

impl FsNode {
    fn size(&self) -> usize {
        match self {
            FsNode::File(file) => file.size,
            FsNode::Dir(dir) => dir.children.values().map(FsNode::size).sum(),
        }
    }

    fn walk<F>(&self, walk_fn: &mut F)
    where
        F: FnMut(&FsNode),
    {
        walk_fn(self);
        if let FsNode::Dir(dir) = self {
            for child in dir.children.values() {
                child.walk(walk_fn);
            }
        }
    }
}

fn parse_fs_from_terminal_output(terminal_output: &str) -> FsNode {
    let mut root = Dir {
        children: HashMap::new(),
    };

    let mut curr_dir_stack: Vec<&str> = vec![];
    for line in terminal_output.lines() {
        let words: Vec<_> = line.split(' ').collect();
        if words[0] == "$" {
            match words[1] {
                "cd" => match words[2] {
                    "/" => curr_dir_stack.clear(),
                    ".." => {
                        curr_dir_stack.pop().expect("dir stack should not be empty");
                    }
                    dir_name => curr_dir_stack.push(dir_name),
                },
                "ls" => {}
                cmd => unreachable!("unknown command {cmd}"),
            }
        } else {
            // line is part of `ls` output.
            let dir_or_size = words[0];
            let name = words[1].to_string();
            let new_node = if dir_or_size == "dir" {
                FsNode::Dir(Dir {
                    children: HashMap::new(),
                })
            } else {
                let size: usize = dir_or_size.parse().expect("size should be a valid number");
                FsNode::File(File { size })
            };

            let mut curr_dir = &mut root;
            for &name in curr_dir_stack.iter() {
                let child = curr_dir.children.get_mut(name);
                let Some(FsNode::Dir(child_dir)) = child else {
                    panic!("curr_dir should have child directory {name}");
                };
                curr_dir = child_dir;
            }
            curr_dir.children.insert(name, new_node);
        }
    }
    FsNode::Dir(root)
}
