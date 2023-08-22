use fxhash::FxHashMap as HashMap;

pub fn run(input: &str) -> String {
    let (mut elves, mut map) = parse_input(input);

    let directions = {
        let n = (0, -1);
        let ne = (1, -1);
        let e = (1, 0);
        let se = (1, 1);
        let s = (0, 1);
        let sw = (-1, 1);
        let w = (-1, 0);
        let nw = (-1, -1);
        [[n, ne, nw], [s, se, sw], [w, nw, sw], [e, ne, se]]
    };

    let mut empty_tiles_at_round_10 = 0;
    let mut round = 0;
    let mut proposed_moves = HashMap::default();
    loop {
        for (elf_index, (x, y)) in elves.iter().enumerate() {
            let empty_dirs = directions.map(|dirs| {
                dirs.iter()
                    .all(|&(dx, dy)| !map[y.wrapping_add_signed(dy)][x.wrapping_add_signed(dx)])
            });

            if empty_dirs.into_iter().all(|x| x) {
                continue;
            }

            for i in 0..4 {
                let dir_index = (round + i) % 4;
                if empty_dirs[dir_index] {
                    let (dx, dy) = directions[dir_index][0];
                    let dst = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                    proposed_moves
                        .entry(dst)
                        .and_modify(|m| *m = Err(TooManyElves))
                        .or_insert(Ok(elf_index));
                    break;
                }
            }
        }

        for (&(x, y), proposed_move) in proposed_moves.iter() {
            if let Ok(elf_index) = proposed_move {
                let (elf_x, elf_y) = elves[*elf_index];
                map[elf_y][elf_x] = false;
                map[y][x] = true;
                elves[*elf_index] = (x, y);
            }
        }

        round += 1;

        if round == 10 {
            let (mut min_x, mut min_y) = (usize::MAX, usize::MAX);
            let (mut max_x, mut max_y) = (0, 0);
            for &(x, y) in elves.iter() {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
            let area = (max_x - min_x + 1) * (max_y - min_y + 1);
            empty_tiles_at_round_10 = area - elves.len();
        }

        if proposed_moves.is_empty() {
            break;
        }

        proposed_moves.clear();
    }

    format!("{empty_tiles_at_round_10} {round}")
}

type Map = Vec<Vec<bool>>;
struct TooManyElves;

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Map) {
    let input_size = input.lines().count();
    // Oversize map a good deal so that elves can move around without going into negative indexes.
    // This 5x map size works fine even for inputs where every tile is an elf.
    let map_size = input_size * 5;
    let margin = input_size * 2;
    let mut map = vec![vec![false; map_size]; map_size];

    let elves: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, ch)| *ch == '#')
                .map(move |(x, _v)| (x + margin, y + margin))
        })
        .collect();

    for &(x, y) in elves.iter() {
        map[y][x] = true;
    }

    (elves, map)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        assert_eq!(super::run(SAMPLE), "110 20")
    }
    const SAMPLE: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";
}
