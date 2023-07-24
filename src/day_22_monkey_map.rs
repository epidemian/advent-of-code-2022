pub fn run(input: &str) -> String {
    let (map_part, inst_part) = input
        .split_once("\n\n")
        .expect("input should have two parts");
    let map = parse_map(map_part);
    let instructions = parse_instructions(inst_part);

    let mut x = map[0]
        .iter()
        .position(|tile| matches!(tile, Open))
        .expect("first row should have an open tile") as i32;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;
    for ins in instructions {
        match ins {
            TurnLeft => (dx, dy) = (dy, -dx),
            TurnRight => (dx, dy) = (-dy, dx),
            Advance(n) => {
                for _ in 0..n {
                    let Some((new_x, new_y)) = try_advance(x, y, dx, dy, &map) else {
                        break
                    };
                    (x, y) = (new_x, new_y);
                }
            }
        }
    }

    let dir_num = match (dx, dy) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        dir => unreachable!("unexpected final direction {dir:?}"),
    };
    let ans_1 = 1000 * (y + 1) + 4 * (x + 1) + dir_num;

    format!("{ans_1}")
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
