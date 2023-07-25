pub fn run(input: &str) -> String {
    let (map_part, inst_part) = input
        .split_once("\n\n")
        .expect("input should have two parts");
    let map = parse_map(map_part);
    let instructions = parse_instructions(inst_part);

    let ans_1 = part_1(&map, &instructions);
    let ans_2 = part_2(&map, &instructions);

    format!("{ans_1} {ans_2}")
}

fn part_1(map: &Map, instructions: &[Instruction]) -> i32 {
    let mut x = map[0]
        .iter()
        .position(|tile| matches!(tile, Open))
        .expect("first row should have an open tile") as i32;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;
    for ins in instructions.iter() {
        match ins {
            TurnLeft => (dx, dy) = (dy, -dx),
            TurnRight => (dx, dy) = (-dy, dx),
            Advance(n) => {
                for _ in 0..*n {
                    let Some((new_x, new_y)) = try_advance(x, y, dx, dy, &map) else {
                        break
                    };
                    (x, y) = (new_x, new_y);
                }
            }
        }
    }

    let dir_num = match (dx, dy) {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,
        dir => unreachable!("unexpected final direction {dir:?}"),
    };
    1000 * (y + 1) + 4 * (x + 1) + dir_num
}

fn part_2(map: &Map, instructions: &[Instruction]) -> i32 {
    let mut x = map[0]
        .iter()
        .position(|tile| matches!(tile, Open))
        .expect("first row should have an open tile") as i32;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;
    for ins in instructions.iter() {
        match ins {
            TurnLeft => (dx, dy) = turn_left(dx, dy),
            TurnRight => (dx, dy) = turn_right(dx, dy),
            Advance(n) => {
                for _ in 0..*n {
                    let Some((new_x, new_y, new_dx, new_dy)) = try_advance_part_2(x, y, dx, dy, &map) else {
                        break
                    };
                    (x, y) = (new_x, new_y);
                    (dx, dy) = (new_dx, new_dy);
                }
            }
        }
    }

    let dir_num = match (dx, dy) {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,
        dir => unreachable!("unexpected final direction {dir:?}"),
    };
    1000 * (y + 1) + 4 * (x + 1) + dir_num
}

fn try_advance(x: i32, y: i32, dx: i32, dy: i32, map: &Map) -> Option<(i32, i32)> {
    let mut new_x = x + dx;
    let mut new_y = y + dy;
    let mut tile = get_tile_non_wrapping(new_x, new_y, map);
    if tile == Empty {
        // Wrap around going in the opposite direction until we go out of bounds.
        loop {
            let backward_tile = get_tile_non_wrapping(new_x - dx, new_y - dy, map);
            if backward_tile == Empty {
                break;
            };
            tile = backward_tile;
            new_x -= dx;
            new_y -= dy;
        }
    }
    if tile == Wall {
        None
    } else {
        Some((new_x, new_y))
    }
}

// An ad-hoc mapping of the cube faces for the cube unfolding on my particular input.
#[rustfmt::skip]
const CUBE_MAP: [&str; 4] = [
    " UR",
    " F ",
    "LD ",
    "B  "
];

const RIGHT: (i32, i32) = (1, 0);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const UP: (i32, i32) = (0, -1);
const FACE_SIDE: i32 = 50;

fn turn_left(x: i32, y: i32) -> (i32, i32) {
    (y, -x)
}

fn turn_right(x: i32, y: i32) -> (i32, i32) {
    (-y, x)
}

// x and y should be in 0..FACE_SIZE
fn turn_right_in_face(x: i32, y: i32) -> (i32, i32) {
    let (new_x, new_y) = turn_right(x, y);
    (new_x + FACE_SIDE - 1, new_y)
}

fn try_advance_part_2(x: i32, y: i32, dx: i32, dy: i32, map: &Map) -> Option<(i32, i32, i32, i32)> {
    let mut new_x = x + dx;
    let mut new_y = y + dy;
    let mut new_dx = dx;
    let mut new_dy = dy;

    let mut tile = get_tile_non_wrapping(new_x, new_y, map);
    if tile == Empty {
        let face = CUBE_MAP[y as usize / 50].as_bytes()[x as usize / 50] as char;
        let (new_face, new_dir) = match (face, (dx, dy)) {
            ('U', UP) => ('B', RIGHT),
            ('B', DOWN) => ('R', DOWN),
            ('R', UP) => ('B', UP),
            ('B', RIGHT) => ('D', UP),
            ('D', RIGHT) => ('R', LEFT),
            ('R', RIGHT) => ('D', LEFT),
            ('B', LEFT) => ('U', DOWN),
            ('U', LEFT) => ('L', RIGHT),
            ('L', UP) => ('F', RIGHT),
            ('L', LEFT) => ('U', RIGHT),
            ('R', DOWN) => ('F', LEFT),
            ('F', LEFT) => ('L', DOWN),
            ('F', RIGHT) => ('R', UP),
            ('D', DOWN) => ('B', LEFT),
            _ => panic!("i don't know how to go from face {face} in direction {dx},{dy}"),
        };
        let mut x_mod = (x + dx).rem_euclid(50);
        let mut y_mod = (y + dy).rem_euclid(50);
        let mut dir = (dx, dy);
        while dir != new_dir {
            (x_mod, y_mod) = turn_right_in_face(x_mod, y_mod);
            dir = turn_right(dir.0, dir.1);
        }

        let (face_x, face_y) = get_cube_face_pos(new_face);
        new_x = face_x + x_mod;
        new_y = face_y + y_mod;
        (new_dx, new_dy) = new_dir;
        tile = get_tile_non_wrapping(new_x, new_y, map)
    }
    if tile == Wall {
        None
    } else {
        Some((new_x, new_y, new_dx, new_dy))
    }
}

fn get_cube_face_pos(face: char) -> (i32, i32) {
    CUBE_MAP
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars().enumerate().find_map(|(x, ch)| {
                if ch == face {
                    Some((x as i32 * 50, y as i32 * 50))
                } else {
                    None
                }
            })
        })
        .expect("could not find face in cube map")
}

fn get_tile_non_wrapping(x: i32, y: i32, map: &Vec<Vec<Tile>>) -> Tile {
    *map.get(y as usize)
        .and_then(|row| row.get(x as usize))
        .unwrap_or(&Empty)
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Open,
    Wall,
}

use Tile::*;

enum Instruction {
    TurnLeft,
    TurnRight,
    Advance(u32),
}
use Instruction::*;

type Map = Vec<Vec<Tile>>;

fn parse_map(s: &str) -> Map {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    ' ' => Empty,
                    '.' => Open,
                    '#' => Wall,
                    _ => unreachable!("unexpected character '{ch}'"),
                })
                .collect()
        })
        .collect()
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.replace('L', " L ")
        .replace('R', " R ")
        .split_whitespace()
        .map(|token| {
            if let Ok(num) = token.parse() {
                return Advance(num);
            }
            match token {
                "L" => TurnLeft,
                "R" => TurnRight,
                _ => unreachable!("unexpected instruction '{token}'"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        let sample = include_str!("../inputs/day22-sample.txt");
        assert_eq!(super::run(sample), "6032")
    }
}
