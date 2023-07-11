pub fn run(input: &str) -> String {
    let droplet_points: Vec<_> = input.lines().map(parse_point).collect();

    let mut grid = [[[Air; SCAN_SIZE]; SCAN_SIZE]; SCAN_SIZE];

    for &(x, y, z) in droplet_points.iter() {
        grid[x][y][z] = Lava;
    }

    let naive_exposed_area: usize = get_droplet_area(&droplet_points, &grid, Air);

    flood_fill_exposed_points(&mut grid);
    let exposed_area: usize = get_droplet_area(&droplet_points, &grid, Exposed);

    format!("{naive_exposed_area} {exposed_area}")
}

const SCAN_SIZE: usize = 22;

type Point = (usize, usize, usize);
type Grid = [[[CubeType; SCAN_SIZE]; SCAN_SIZE]; SCAN_SIZE];

#[derive(Copy, Clone, PartialEq)]
enum CubeType {
    Lava,
    Air,
    Exposed,
}
use CubeType::*;

fn parse_point(s: &str) -> Point {
    let coordinates: Vec<usize> = s
        .split(',')
        .map(|s| {
            // Hack: add 1 to have the boulder not sticking to one of the grid faces.
            // The absolute coordinates are not important if everything is displaced the same way.
            let num = s.parse::<usize>().expect("invalid number") + 1;
            if num >= SCAN_SIZE - 1 {
                panic!("coordinate {num} too big")
            }
            num
        })
        .collect();
    let [x, y, z] = coordinates[..] else {
        panic!("there should be 3 coordinates");
    };
    (x, y, z)
}

fn get_droplet_area(droplet_points: &[Point], grid: &Grid, neighbor_type: CubeType) -> usize {
    droplet_points
        .iter()
        .map(|&(x, y, z)| {
            let neighbors = [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ];
            neighbors
                .into_iter()
                .filter(|&(nx, ny, nz)| grid[nx][ny][nz] == neighbor_type)
                .count()
        })
        .sum()
}

fn flood_fill_exposed_points(grid: &mut Grid) {
    let mut unvisited = vec![(0, 0, 0)];
    while let Some((x, y, z)) = unvisited.pop() {
        if matches!(grid[x][y][z], Exposed | Lava) {
            continue;
        }
        grid[x][y][z] = Exposed;
        let neighbors = [
            (x + 1, y, z),
            (x.wrapping_sub(1), y, z),
            (x, y + 1, z),
            (x, y.wrapping_sub(1), z),
            (x, y, z + 1),
            (x, y, z.wrapping_sub(1)),
        ];
        for (nx, ny, nz) in neighbors {
            if nx < SCAN_SIZE && ny < SCAN_SIZE && nz < SCAN_SIZE {
                unvisited.push((nx, ny, nz))
            }
        }
    }
}
