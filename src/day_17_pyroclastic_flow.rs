use std::collections::HashMap;

pub fn run(input: &str) -> String {
    format!(
        "{}, {}",
        run_rock_simulation(input, 2022),
        run_rock_simulation(input, 1000000000000)
    )
}

fn run_rock_simulation(input: &str, total_rock_falls: u64) -> u64 {
    let rocks_count = ROCKS.len();
    let left = (-1, 0);
    let right = (1, 0);
    let down = (0, -1);
    let mut rock_iter = ROCKS.iter().enumerate().cycle();
    let mut jet_iter = input.chars().enumerate().cycle();
    let mut chamber: Vec<Row> = vec![];
    let mut cyclic_height = None;
    let mut chamber_tops_memo: HashMap<usize, ChamberTopMemo> = HashMap::new();
    let mut remaining_rock_falls = total_rock_falls;
    while remaining_rock_falls > 0 {
        let (rock_idx, rock) = rock_iter.next().expect("rocks should be infinite");
        let mut rock = rock.to_vec();
        move_rock(&mut rock, (2, chamber.len() as i64 + 3), &chamber);

        loop {
            let (jet_idx, jet_ch) = jet_iter.next().expect("hot air jets should be infinite");
            let jet_dir = match jet_ch {
                '<' => left,
                '>' => right,
                _ => unreachable!("unexpected character '{jet_ch}'"),
            };
            move_rock(&mut rock, jet_dir, &chamber);
            let should_stop = !move_rock(&mut rock, down, &chamber);
            if !should_stop {
                continue;
            }

            for (x, y) in rock.iter() {
                while *y as usize >= chamber.len() {
                    chamber.push(Default::default());
                }
                chamber[*y as usize][*x as usize] = ROCK;
            }

            if cyclic_height.is_some() {
                break;
            }

            let chamber_top = &chamber[chamber.len() - 5.min(chamber.len())..];
            let chamber_top_sealed = chamber_top.windows(2).any(|w| {
                let (row_a, row_b) = (w[0], w[1]);
                row_a
                    .iter()
                    .zip(row_b.iter())
                    .all(|(cell_a, cell_b)| *cell_a == ROCK || *cell_b == ROCK)
            });
            if !chamber_top_sealed {
                break;
            }

            let combined_idx = jet_idx * rocks_count + rock_idx;
            if let Some(memo) = &chamber_tops_memo.get(&combined_idx) {
                if chamber_top == memo.chamber_top {
                    let height_diff = chamber.len() - memo.chamber_height;
                    let fallen_rocks_diff = memo.remaining_rock_falls - remaining_rock_falls;
                    let remaining_cycles = (remaining_rock_falls - 1) / fallen_rocks_diff;
                    cyclic_height = Some(remaining_cycles * height_diff as u64);
                    remaining_rock_falls -= remaining_cycles * fallen_rocks_diff;
                    break;
                }
            }
            chamber_tops_memo.insert(
                combined_idx,
                ChamberTopMemo {
                    chamber_top: chamber_top.to_vec(),
                    chamber_height: chamber.len(),
                    remaining_rock_falls,
                },
            );

            break;
        }
        remaining_rock_falls -= 1;
    }

    chamber.len() as u64 + cyclic_height.unwrap_or(0)
}

const ROCKS: [&[Point]; 5] = [
    // ####
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    // .#.
    // ###
    // .#.
    &[(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
    // ..#
    // ..#
    // ###
    &[(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
    // #
    // #
    // #
    // #
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    // ##
    // ##
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
];

const ROCK: u8 = 1;
type Point = (i64, i64);
type Row = [u8; 7];
struct ChamberTopMemo {
    chamber_top: Vec<Row>,
    chamber_height: usize,
    remaining_rock_falls: u64,
}

fn move_rock(rock: &mut [Point], (dx, dy): Point, chamber: &[Row]) -> bool {
    for (x, y) in rock.iter() {
        let x2 = x + dx;
        let y2 = y + dy;
        if x2 < 0 || x2 >= 7 || y2 < 0 {
            return false;
        }
        if (y2 as usize) < chamber.len() && chamber[y2 as usize][x2 as usize] == ROCK {
            return false;
        }
    }

    for (x, y) in rock.iter_mut() {
        *x += dx;
        *y += dy;
    }
    true
}
