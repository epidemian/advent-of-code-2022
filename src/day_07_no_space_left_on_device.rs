use std::collections::HashMap;

pub fn run(input: &str) -> String {
    let dir_sizes = calc_directory_sizes_from_terminal_output(input);

    let small_dirs_total_size: usize = dir_sizes
        .values()
        .filter(|dir_size| **dir_size <= 100_000)
        .sum();

    let unused_space = 70_000_000 - dir_sizes["/"];
    let space_to_free_up = 30_000_000 - unused_space;

    let file_to_delete_size = dir_sizes
        .values()
        .filter(|size| **size >= space_to_free_up)
        .min()
        .expect("there should be a big-enough directory to free up space for the update");

    format!("{small_dirs_total_size} {file_to_delete_size}")
}

fn calc_directory_sizes_from_terminal_output(terminal_output: &str) -> HashMap<String, usize> {
    // Keys are full directory paths, like "/foo/bar"
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

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
            // Line is part of `ls` output.
            let dir_or_size = words[0];
            if dir_or_size != "dir" {
                let size: usize = dir_or_size.parse().expect("size should be a valid number");
                for i in 0..=curr_dir_stack.len() {
                    let dir_path = format!("/{}", curr_dir_stack[0..i].join("/"));
                    *dir_sizes.entry(dir_path).or_insert(0) += size;
                }
            }
        }
    }
    dir_sizes
}
