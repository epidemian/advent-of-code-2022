use std::collections::VecDeque;

pub fn run(input: &str) -> String {
    let (map_part, inst_part) = input
        .split_once("\n\n")
        .expect("input should have two parts");
    let map = parse_map(map_part);
    let instructions = parse_instructions(inst_part);

    let ans_1 = get_password(&map, &instructions, wrap_around_2d);
    let ans_2 = get_password(&map, &instructions, wrap_around_3d_cube);

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
type WrapFn = fn(pos: Point, dir: Point, map: &Map) -> (Point, Point);

const RIGHT: Point = (1, 0);
const DOWN: Point = (0, 1);
const LEFT: Point = (-1, 0);
const UP: Point = (0, -1);
const DIRECTIONS: [Point; 4] = [RIGHT, DOWN, LEFT, UP];

fn get_password(map: &Map, instructions: &[Instruction], wrap_fn: WrapFn) -> i32 {
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
                    let Some((new_pos, new_dir)) = try_advance(pos, dir, map, wrap_fn) else {
                        break;
                    };
                    pos = new_pos;
                    dir = new_dir;
                }
            }
        }
    }

    let (x, y) = pos;
    1000 * (y + 1) + 4 * (x + 1) + dir_to_rot(dir) as i32
}

fn dir_to_rot(dir: Point) -> usize {
    let Some(index) = DIRECTIONS.iter().position(|d| *d == dir) else {
        unreachable!("unexpected direction {dir:?}")
    };
    index
}

fn try_advance(pos: Point, dir: Point, map: &Map, wrap_fn: WrapFn) -> Option<(Point, Point)> {
    let mut new_pos = point_add(pos, dir);
    let mut new_dir = dir;

    let mut tile = get_tile_non_wrapping(new_pos, map);
    if tile == Empty {
        (new_pos, new_dir) = wrap_fn(pos, dir, map);
        tile = get_tile_non_wrapping(new_pos, map);
        assert!(tile != Empty)
    }
    if tile == Wall {
        None
    } else {
        Some((new_pos, new_dir))
    }
}

fn wrap_around_2d(pos: Point, dir: Point, map: &Map) -> (Point, Point) {
    let mut pos = pos;
    // Wrap around going in the opposite direction until we go out of bounds.
    loop {
        let back_pos = point_sub(pos, dir);
        if get_tile_non_wrapping(back_pos, map) == Empty {
            return (pos, dir);
        };
        pos = back_pos;
    }
}

fn wrap_around_3d_cube(pos: Point, dir: Point, map: &Map) -> (Point, Point) {
    let is_sample = map.len() < 50;
    let cube_size = if is_sample { 4 } else { 50 };
    // Note: these faces could be pre-computed, as it's always the same value for the same map. But
    // this is not a perf bottleneck.
    let faces = get_cube_faces(map, cube_size);

    let (x, y) = pos;
    let face_pos = (x - x % cube_size, y - y % cube_size);
    let Some(face_index) = faces.iter().position(|(p, _)| *p == face_pos) else {
        panic!("face at {face_pos:?} not found");
    };
    let face_rot = faces[face_index].1;

    let next_face = FACE_CONNECTIONS[face_index][(dir_to_rot(dir) + face_rot) % 4];
    let (next_face_pos, next_face_rot) = faces[next_face];

    let rot_from_next_face = FACE_CONNECTIONS[next_face]
        .iter()
        .position(|f| *f == face_index)
        .unwrap();
    let new_dir = DIRECTIONS[(rot_from_next_face + 2 + 4 - next_face_rot) % 4];
    let mut d = dir;
    let mut pos_in_face = point_mod(point_add(pos, dir), cube_size);
    while d != new_dir {
        pos_in_face = turn_right_in_face(pos_in_face, cube_size);
        d = turn_right(d);
    }
    let new_pos = point_add(next_face_pos, pos_in_face);

    (new_pos, new_dir)
}

// Note: these 4-neighbor arrays can start from any face; only their clockwise order matters.
const FACE_CONNECTIONS: [[usize; 4]; 6] = {
    let (f, u, r, b, d, l) = (0, 1, 2, 3, 4, 5);
    [
        [r, d, l, u],
        [r, f, l, b],
        [b, d, f, u],
        [l, d, r, u],
        [r, b, l, f],
        [f, d, b, u],
    ]
};

// Computes the position and rotation of each face in the cube. The cube faces are just numbers from
// 0 to 5 and are the indexes of this array. The rotation of a face is a number from 0 to 3 that
// represents how the face is rotated in the 2D map with respect to the neighbor order in
// FACE_CONNECTIONS.
fn get_cube_faces(map: &Map, cube_size: i32) -> [(Point, usize); 6] {
    let front_face_x = map[0].iter().position(|x| !matches!(x, Empty)).expect("") as i32;
    let mut faces = [None; 6];
    faces[0] = Some(((front_face_x, 0), 0));

    let mut face_queue = VecDeque::from_iter([0]);
    while let Some(face) = face_queue.pop_front() {
        let (face_pos, face_rot) = faces[face].expect("face info should exist");
        for (i, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            let neighbor_pos = point_add(face_pos, (dx * cube_size, dy * cube_size));
            if get_tile_non_wrapping(neighbor_pos, map) == Empty {
                continue;
            }
            let neighbor_face = FACE_CONNECTIONS[face][(i + face_rot) % 4];
            if faces[neighbor_face].is_some() {
                continue;
            }
            let face_rot_from_neighbor = FACE_CONNECTIONS[neighbor_face]
                .iter()
                .position(|f| *f == face)
                .unwrap();
            let neighbor_rot = (face_rot_from_neighbor + 2 + 4 - i) % 4;
            faces[neighbor_face] = Some((neighbor_pos, neighbor_rot));
            face_queue.push_back(neighbor_face);
        }
    }

    faces.map(|f| f.expect("expected to find all 6 faces"))
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
