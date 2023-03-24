pub fn run(input: &str) -> String {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|ch| ch - b'0').collect())
        .collect();
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut visible_trees = 0;
    let mut best_scenic_score = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, tree_height) in row.iter().enumerate() {
            let mut visible = false;
            let mut viewing_distances = [0; 4];
            for (i, &(dx, dy)) in directions.iter().enumerate() {
                let (mut x, mut y) = (x, y);
                loop {
                    // Let negative out-of-bounds wrap around and only check for going over the grid
                    // size.
                    x = x.wrapping_add_signed(dx);
                    y = y.wrapping_add_signed(dy);
                    if x >= grid_width || y >= grid_height {
                        // Viewing distance extends to the edge of the grid, so the tree is visible.
                        visible = true;
                        break;
                    }
                    viewing_distances[i] += 1;
                    if grid[y][x] >= *tree_height {
                        break;
                    }
                }
            }
            visible_trees += visible as usize;
            let scenic_score = viewing_distances.iter().product();
            best_scenic_score = best_scenic_score.max(scenic_score);
        }
    }

    format!("{visible_trees} {best_scenic_score}")
}
