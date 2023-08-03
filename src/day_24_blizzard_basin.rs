use crate::dijkstra;

pub fn run(input: &str) -> String {
    let (map, start, end) = parse_map(input);
    let first_trip_time = shortest_path(&map, start, end, 0);
    let mut total_time = first_trip_time;
    total_time += shortest_path(&map, end, start, total_time as i32);
    total_time += shortest_path(&map, start, end, total_time as i32);

    format!("{first_trip_time} {total_time}")
}

// Note: `start` and `end` may be off-bounds by one.
fn shortest_path(map: &Map, start: Point, end: Point, start_time: i32) -> usize {
    dijkstra::shortest_path(
        &(start, start_time),
        |&(pos, _t)| pos == end,
        |&((x, y), t)| {
            [(x + 1, y), (x, y + 1), (x, y), (x, y - 1), (x - 1, y)]
                .into_iter()
                .filter(|&(x, y)| {
                    let in_bounds =
                        x >= 0 && x < map[0].len() as i32 && y >= 0 && y < map.len() as i32;
                    in_bounds || (x, y) == start || (x, y) == end
                })
                .filter(move |&(x, y)| {
                    (x, y) == start || (x, y) == end || tile_is_empty_at(map, x, y, t + 1)
                })
                .map(move |pos| (pos, t + 1))
        },
    )
    .expect("there must be a path from start to end")
}

fn tile_is_empty_at(map: &Map, x: i32, y: i32, t: i32) -> bool {
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    // The tile at (x, y) is not empty if there is an up-moving blizzard `t` rows below (wrapping
    // around).
    if let Tile::Up = map[(y + t).rem_euclid(height) as usize][x as usize] {
        return false;
    }
    // And do a similar check for all other directions.
    if let Tile::Down = map[(y - t).rem_euclid(height) as usize][x as usize] {
        return false;
    }
    if let Tile::Left = map[y as usize][(x + t).rem_euclid(width) as usize] {
        return false;
    }
    if let Tile::Right = map[y as usize][(x - t).rem_euclid(width) as usize] {
        return false;
    }
    true
}

enum Tile {
    Empty,
    Up,
    Down,
    Left,
    Right,
}

type Map = Vec<Vec<Tile>>;
type Point = (i32, i32);

fn parse_map(input: &str) -> (Map, Point, Point) {
    let lines: Vec<_> = input.lines().collect();
    let [start_line, middle_lines @ .., end_line] = &lines[..] else {
        panic!("input should have more than 2 lines")
    };

    let [start_x, end_x] = [start_line, end_line].map(|line| {
        line.chars()
            .position(|ch| ch == '.')
            .expect("first row must have empty tile") as i32
            - 1
    });

    let map: Map = middle_lines
        .into_iter()
        .map(|line| {
            line.trim_matches('#')
                .chars()
                .map(|ch| match ch {
                    '.' => Tile::Empty,
                    '^' => Tile::Up,
                    'v' => Tile::Down,
                    '<' => Tile::Left,
                    '>' => Tile::Right,
                    _ => unreachable!("unexpected character '{ch}'"),
                })
                .collect()
        })
        .collect();

    let start = (start_x, -1);
    let end = (end_x, map.len() as i32);

    (map, start, end)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        assert_eq!(super::run(SAMPLE.trim()), "18 54")
    }
    const SAMPLE: &str = "
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";
}
