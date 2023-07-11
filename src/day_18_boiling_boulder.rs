pub fn run(input: &str) -> String {
    let droplet_points: Vec<_> = input.lines().map(parse_point).collect();

    let mut max_coord = 0;
    for &(x, y, z) in droplet_points.iter() {
        max_coord = max_coord.max(x.max(y.max(z)))
    }
    // Instead of having the grid size be max_coord + 1, add 1 more space to allow the flood-filling
    // to fill spaces where the droplet touches the grid sides.
    let grid_size = max_coord + 2;
    let mut grid = vec![vec![vec![false; grid_size]; grid_size]; grid_size];

    for &(x, y, z) in droplet_points.iter() {
        grid[x][y][z] = true;
    }

    let naive_exposed_area: usize = count_exposed_faces(&droplet_points, &grid);

    let filled_grid = fill_air_pockets(&grid);
    let exposed_area: usize = count_exposed_faces(&droplet_points, &filled_grid);

    format!("{naive_exposed_area} {exposed_area}")
}

type Point = (usize, usize, usize);
type Grid = Vec<Vec<Vec<bool>>>; // true = lava

fn parse_point(s: &str) -> Point {
    let coordinates: Vec<usize> = s
        .split(',')
        .map(|s| s.parse::<usize>().expect("invalid number"))
        // Hack: add 1 to have the boulder not sticking to one of the grid faces.
        // The absolute coordinates are not important if everything is displaced the same way.
        .map(|num| num + 1)
        .collect();
    let [x, y, z] = coordinates[..] else {
        panic!("there should be 3 coordinates");
    };
    (x, y, z)
}

fn count_exposed_faces(droplet_points: &[Point], grid: &Grid) -> usize {
    let grid_size = grid.len();
    droplet_points
        .iter()
        .map(|point| {
            neighbors(*point, grid_size)
                .filter(|&(nx, ny, nz)| !grid[nx][ny][nz])
                .count()
        })
        .sum()
}

fn fill_air_pockets(grid: &Grid) -> Grid {
    let grid_size = grid.len();
    let mut filled_grid = vec![vec![vec![true; grid_size]; grid_size]; grid_size];
    let mut unvisited = vec![(0, 0, 0)];
    while let Some((x, y, z)) = unvisited.pop() {
        if grid[x][y][z] || !filled_grid[x][y][z] {
            continue;
        }
        filled_grid[x][y][z] = false;
        for (nx, ny, nz) in neighbors((x, y, z), grid_size) {
            unvisited.push((nx, ny, nz))
        }
    }
    filled_grid
}

fn neighbors((x, y, z): Point, grid_size: usize) -> impl Iterator<Item = Point> {
    [
        (x + 1, y, z),
        (x.wrapping_sub(1), y, z),
        (x, y + 1, z),
        (x, y.wrapping_sub(1), z),
        (x, y, z + 1),
        (x, y, z.wrapping_sub(1)),
    ]
    .into_iter()
    .filter(move |&(nx, ny, nz)| nx < grid_size && ny < grid_size && nz < grid_size)
}
