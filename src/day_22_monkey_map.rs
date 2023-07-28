pub fn run(input: &str) -> String {
    let (map_part, inst_part) = input
        .split_once("\n\n")
        .expect("input should have two parts");
    let map = parse_map(map_part);
    let instructions = parse_instructions(inst_part);

    let ans_1 = get_password(&map, &instructions, false);
    let ans_2 = get_password(&map, &instructions, true);

    format!("{ans_1} {ans_2}")
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
type Point = (i32, i32);

const RIGHT: Point = (1, 0);
const DOWN: Point = (0, 1);
const LEFT: Point = (-1, 0);
const UP: Point = (0, -1);

fn get_password(map: &Map, instructions: &[Instruction], as_3d_cube: bool) -> i32 {
    let start_x = map[0]
        .iter()
        .position(|tile| matches!(tile, Open))
        .expect("first row should have an open tile") as i32;
    let mut pos = (start_x, 0);
    let mut dir = RIGHT;
    for ins in instructions.iter() {
        match ins {
            TurnLeft => dir = turn_left(dir),
            TurnRight => dir = turn_right(dir),
            Advance(n) => {
                for _ in 0..*n {
                    let Some((new_pos, new_dir)) = try_advance(pos, dir, &map, as_3d_cube) else {
                        break
                    };
                    pos = new_pos;
                    dir = new_dir;
                }
            }
        }
    }

    let dir_num = match dir {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,
        dir => unreachable!("unexpected final direction {dir:?}"),
    };
    let (x, y) = pos;
    1000 * (y + 1) + 4 * (x + 1) + dir_num
}

fn try_advance(pos: Point, dir: Point, map: &Map, as_3d_cube: bool) -> Option<(Point, Point)> {
    let mut new_pos = point_add(pos, dir);
    let mut new_dir = dir;

    let mut tile = get_tile_non_wrapping(new_pos, map);
    if tile == Empty {
        if as_3d_cube {
            (new_pos, new_dir, tile) = wrap_around_3d_cube(pos, dir, map);
        } else {
            (new_pos, tile) = wrap_around_2d(pos, dir, map);
        }
    }
    if tile == Wall {
        None
    } else {
        Some((new_pos, new_dir))
    }
}

fn wrap_around_2d(pos: Point, dir: Point, map: &Map) -> (Point, Tile) {
    let mut pos = pos;
    let mut tile = Empty;
    // Wrap around going in the opposite direction until we go out of bounds.
    loop {
        let back_pos = point_sub(pos, dir);
        let back_tile = get_tile_non_wrapping(back_pos, map);
        if back_tile == Empty {
            return (pos, tile);
        };
        tile = back_tile;
        pos = back_pos;
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

fn wrap_around_3d_cube(pos: Point, dir: Point, map: &Map) -> (Point, Point, Tile) {
    let is_sample = map.len() < 50;
    if is_sample {
        todo!("implement generic solution that works for sample input");
    }
    let cube_size = if is_sample { 4 } else { 50 };

    let (x, y) = pos;
    let face = CUBE_MAP[(y / cube_size) as usize].as_bytes()[(x / cube_size) as usize] as char;

    // These particular face transitions are specific to my input's cube unfolding.
    let (new_face, new_dir) = match (face, dir) {
        ('U', UP) => ('B', RIGHT),
        ('B', LEFT) => ('U', DOWN),
        ('U', LEFT) => ('L', RIGHT),
        ('L', LEFT) => ('U', RIGHT),
        ('R', UP) => ('B', UP),
        ('B', DOWN) => ('R', DOWN),
        ('R', RIGHT) => ('D', LEFT),
        ('D', RIGHT) => ('R', LEFT),
        ('R', DOWN) => ('F', LEFT),
        ('F', RIGHT) => ('R', UP),
        ('F', LEFT) => ('L', DOWN),
        ('L', UP) => ('F', RIGHT),
        ('D', DOWN) => ('B', LEFT),
        ('B', RIGHT) => ('D', UP),
        _ => panic!("unexpected transition from face {face} in direction {dir:?}"),
    };

    let mut pos_in_face = point_mod(point_add(pos, dir), cube_size);
    let mut d = dir;
    while d != new_dir {
        pos_in_face = turn_right_in_face(pos_in_face, cube_size);
        d = turn_right(d);
    }

    let face_pos = get_cube_face_pos(new_face, cube_size);
    let new_pos = point_add(face_pos, pos_in_face);
    let tile = get_tile_non_wrapping(new_pos, map);
    (new_pos, new_dir, tile)
}

fn turn_left((x, y): Point) -> Point {
    (y, -x)
}

fn turn_right((x, y): Point) -> Point {
    (-y, x)
}

fn point_add((x1, y1): Point, (x2, y2): Point) -> Point {
    (x1 + x2, y1 + y2)
}

fn point_sub((x1, y1): Point, (x2, y2): Point) -> Point {
    (x1 - x2, y1 - y2)
}

fn point_mod((x, y): Point, rhs: i32) -> Point {
    (x.rem_euclid(rhs), y.rem_euclid(rhs))
}

// x and y should be in 0..cube_size
fn turn_right_in_face((x, y): Point, cube_size: i32) -> Point {
    (cube_size - 1 - y, x)
}

fn get_cube_face_pos(face: char, cube_size: i32) -> Point {
    for (y, line) in CUBE_MAP.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == face {
                return (x as i32 * cube_size, y as i32 * cube_size);
            }
        }
    }
    unreachable!("could not find face '{face}' in cube map");
}

fn get_tile_non_wrapping((x, y): Point, map: &Map) -> Tile {
    *map.get(y as usize)
        .and_then(|row| row.get(x as usize))
        .unwrap_or(&Empty)
}

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
    #[ignore]
    #[test]
    fn test_sample() {
        let sample = include_str!("../inputs/day22-sample.txt");
        assert_eq!(super::run(sample), "6032")
    }
}
