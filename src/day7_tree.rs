use std::collections::HashMap;

// Another attempt at day 7, trying to use a "more reified" data structure for
// the file system, where directories and files are represented using a tree
// structure.
#[allow(dead_code)]
pub fn run(input: &str) -> String {
    let root = parse_fs_from_terminal_output(input);

    let dir_sizes: Vec<usize> = root
        .iter()
        .filter(|node| matches!(node, FsNode::Dir(..)))
        .map(|node| node.size())
        .collect();

    let small_dirs_total_size: usize = dir_sizes
        .iter()
        .filter(|dir_size| **dir_size <= 100_000)
        .sum();

    let unused_space = 70_000_000 - root.size();
    let space_to_free_up = 30_000_000 - unused_space;

    let file_to_delete_size = dir_sizes
        .iter()
        .filter(|size| **size >= space_to_free_up)
        .min()
        .expect("there should be a big-enough directory to free up space for the update");

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

    fn iter(&self) -> FsIterator {
        FsIterator::new(self)
    }
}

struct FsIterator<'a> {
    node: Option<&'a FsNode>,
    children_iter: Option<Box<dyn Iterator<Item = &'a FsNode> + 'a>>,
}

impl FsIterator<'_> {
    fn new(node: &FsNode) -> FsIterator {
        FsIterator {
            node: Some(node),
            children_iter: None,
        }
    }
}

impl<'a> Iterator for FsIterator<'a> {
    type Item = &'a FsNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node {
            if let FsNode::Dir(dir) = node {
                let children_iter = dir.children.values().flat_map(|node| FsIterator::new(node));
                self.children_iter = Some(Box::new(children_iter));
            }
            self.node = None;
            return Some(node);
        }
        if let Some(ref mut children_iter) = self.children_iter {
            return children_iter.next();
        }
        None
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
