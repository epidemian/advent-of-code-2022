pub fn run(input: &str) -> String {
    let (dirs, files) = parse_fs_from_terminal_output(input);
    let dir_sizes: Vec<usize> = dirs
        .iter()
        .map(|dir_path| {
            files
                .iter()
                .filter(|(file_path, _size)| file_path.starts_with(dir_path))
                .map(|(_file_path, size)| size)
                .sum()
        })
        .collect();

    let small_dirs_total_size: usize = dir_sizes
        .iter()
        .filter(|dir_size| **dir_size <= 100_000)
        .sum();

    let unused_space = 70_000_000 - dir_sizes[0]; // first dir is /
    let space_to_free_up = 30_000_000 - unused_space;

    let file_to_delete_size = dir_sizes
        .iter()
        .filter(|size| **size >= space_to_free_up)
        .min()
        .expect("there should be a big-enough directory to free up space for the update");

    format!("{small_dirs_total_size} {file_to_delete_size}")
}

fn parse_fs_from_terminal_output(terminal_output: &str) -> (Vec<String>, Vec<(String, usize)>) {
    let mut dir_paths: Vec<String> = vec!["/".into()];
    let mut sizes_by_file_paths: Vec<(String, usize)> = vec![];

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
            let name = words[1];

            let full_path = if curr_dir_stack.is_empty() {
                format!("/{name}")
            } else {
                format!("/{}/{}", curr_dir_stack.join("/"), name)
            };

            if dir_or_size == "dir" {
                dir_paths.push(format!("{full_path}/"));
            } else {
                let size: usize = dir_or_size.parse().expect("size should be a valid number");
                sizes_by_file_paths.push((full_path, size));
            }
        }
    }
    (dir_paths, sizes_by_file_paths)
}
