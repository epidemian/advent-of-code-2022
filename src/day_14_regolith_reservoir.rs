pub fn run(input: &str) -> String {
    let mut map = parse_map(input);

    let mut sand_grains_count = 0;
    while drop_sand_grain(&mut map) {
        sand_grains_count += 1;
    }
    let part_1_count = sand_grains_count;

    // Add floor to map.
    map.push(vec![Air; 1000]);
    map.push(vec![Rock; 1000]);

    while drop_sand_grain(&mut map) {
        sand_grains_count += 1;
    }

    format!("{part_1_count} {sand_grains_count}")
}

fn drop_sand_grain(map: &mut Map) -> bool {
    let (mut x, mut y) = (500, 0);
    if !matches!(map[y][x], Air) {
        // Start position already occupied, cannot drop more sand.
        return false;
    }
    loop {
        if y + 1 >= map.len() {
            // Fallen into the void.
            return false;
        }
        let empty_tile = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
            .into_iter()
            .find(|(x, y)| matches!(map[*y][*x], Air));

        let Some((next_x, next_y)) = empty_tile else {
            map[y][x] = Sand;
            return true;
        };
        x = next_x;
        y = next_y;
    }
}

#[derive(Clone)]
enum Tile {
    Air,
    Sand,
    Rock,
}
use Tile::*;

type Point = (usize, usize);
type Map = Vec<Vec<Tile>>;

fn parse_map(input: &str) -> Map {
    let paths: Vec<_> = input.lines().map(parse_path).collect();

    let max_y = *paths
        .iter()
        .flat_map(|path| path.iter())
        .map(|(_x, y)| y)
        .max()
        .expect("there should be at least one path");

    let mut map = vec![vec![Air; 1000]; max_y + 1];
    for path in paths {
        for ((x1, y1), (x2, y2)) in path_segments(&path) {
            if x1 == x2 {
                // Vertical line.
                for y in y1.min(y2)..=y1.max(y2) {
                    map[y][x1] = Rock;
                }
            } else if y1 == y2 {
                // Horizontal line.
                for x in x1.min(x2)..=x1.max(x2) {
                    map[y1][x] = Rock;
                }
            } else {
                panic!("path segment should be either horizontal or vertical")
            }
        }
    }
    map
}

fn parse_path(line: &str) -> Vec<Point> {
    line.split(" -> ").map(parse_point).collect()
}

fn parse_point(s: &str) -> Point {
    let (x, y) = s.split_once(',').expect("point should have a comma");
    (
        x.parse().expect("invalid number"),
        y.parse().expect("invalid number"),
    )
}

fn path_segments(path: &[Point]) -> impl Iterator<Item = (Point, Point)> + '_ {
    path.windows(2).map(|w| (w[0], w[1]))
}
