use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> String {
    let mut elves = parse_elves(input);

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
    loop {
        let mut proposed_moves = HashMap::new();
        for elf in elves.iter() {
            let (x, y) = elf;

            let neighbors =
                directions.map(|dirs| dirs.map(|(dx, dy)| !elves.contains(&(x + dx, y + dy))));

            let all_empty = neighbors.iter().all(|ns| ns.iter().all(|empty| *empty));
            if all_empty {
                continue;
            }

            for i in 0..4 {
                let dir_index = (round + i) % 4;
                let dir_empty = neighbors[dir_index].iter().all(|empty| *empty);
                if dir_empty {
                    let (dx, dy) = directions[dir_index][0];
                    let dst = (x + dx, y + dy);
                    proposed_moves
                        .entry(dst)
                        // If there's already another elf wanting to move to this destination, don't
                        // consider either of those moves.
                        .and_modify(|m| *m = Err(()))
                        .or_insert(Ok(*elf));
                    break;
                }
            }
        }

        for (dst, proposed_move) in proposed_moves.iter() {
            if let Ok(elf) = proposed_move {
                elves.remove(elf);
                elves.insert(*dst);
            }
        }

        round += 1;

        if round == 10 {
            let (mut min_x, mut min_y) = (0, 0);
            let (mut max_x, mut max_y) = (0, 0);
            for &(x, y) in elves.iter() {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
            let area = (max_x - min_x + 1) * (max_y - min_y + 1);
            empty_tiles_at_round_10 = area - elves.len() as i32;
        }

        if proposed_moves.is_empty() {
            break;
        }
    }

    format!("{empty_tiles_at_round_10} {round}")
}

fn parse_elves(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, ch)| *ch == '#')
                .map(move |(x, _ch)| (x as i32, y as i32))
        })
        .collect()
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
.#..#..";
}
