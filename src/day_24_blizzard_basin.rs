use crate::dijkstra::shortest_path;

pub fn run(input: &str) -> String {
    let (map, start, end) = parse_map(input);
    let first_trip_time = shortest_travel_time(&map, start, end, 0);
    let back_to_start_time = shortest_travel_time(&map, end, start, first_trip_time);
    let back_to_end_time = shortest_travel_time(&map, start, end, back_to_start_time);

    format!("{first_trip_time} {back_to_end_time}")
}

// `start` and `end` can be off-bounds by one.
fn shortest_travel_time(map: &Map, start: Point, end: Point, start_time: i32) -> i32 {
    let next_moves = |&((x, y), t): &_| {
        // Note: include current (x, y) as a possible move for waiting a turn.
        [(x + 1, y), (x, y + 1), (x, y), (x, y - 1), (x - 1, y)]
            .into_iter()
            .filter(move |&(x, y)| {
                let start_or_end = (x, y) == start || (x, y) == end;
                let in_bounds = x >= 0 && x < map[0].len() as i32 && y >= 0 && y < map.len() as i32;
                start_or_end || (in_bounds && tile_is_empty_at(map, x, y, t + 1))
            })
            .map(move |pos| (pos, t + 1))
    };
    let dist = shortest_path(&(start, start_time), |&(pos, _t)| pos == end, next_moves)
        .expect("there must be a path from start to end");
    dist as i32 + start_time
}

// Determines whether an (x, y) tile has no blizzards at a given time. x and y must be in-bounds.
fn tile_is_empty_at(map: &Map, x: i32, y: i32, time: i32) -> bool {
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    // The tile at (x, y) is not empty if there is an up-moving blizzard `time` rows below (wrapping
    // around).
    if let UpBlizzard = map[(y + time).rem_euclid(height) as usize][x as usize] {
        return false;
    }
    // And do a similar check for all other directions...
    if let DownBlizzard = map[(y - time).rem_euclid(height) as usize][x as usize] {
        return false;
    }
    if let LeftBlizzard = map[y as usize][(x + time).rem_euclid(width) as usize] {
        return false;
    }
    if let RightBlizzard = map[y as usize][(x - time).rem_euclid(width) as usize] {
        return false;
    }
    true
}

enum Tile {
    Empty,
    UpBlizzard,
    DownBlizzard,
    LeftBlizzard,
    RightBlizzard,
}
use Tile::*;

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
        .iter()
        .map(|line| {
            line.trim_matches('#')
                .chars()
                .map(|ch| match ch {
                    '.' => Empty,
                    '^' => UpBlizzard,
                    'v' => DownBlizzard,
                    '<' => LeftBlizzard,
                    '>' => RightBlizzard,
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
        assert_eq!(super::run(SAMPLE), "18 54")
    }
    const SAMPLE: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";
}
