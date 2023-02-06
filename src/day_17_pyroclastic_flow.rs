use std::collections::HashMap;

pub fn run(input: &str) -> String {
    format!(
        "{}, {}",
        run_rock_simulation(input, 2022),
        run_rock_simulation(input, 1_000_000_000_000)
    )
}

fn run_rock_simulation(input: &str, total_rock_falls: u64) -> u64 {
    let mut rock_iter = ROCKS.iter().copied().enumerate().cycle();
    let mut jet_iter = input.chars().enumerate().cycle();
    let mut chamber: Vec<Row> = vec![];
    let mut cyclic_height = None;
    let mut previous_chamber_tops: HashMap<(usize, usize), ChamberTopMemo> = HashMap::new();
    let mut remaining_rock_falls = total_rock_falls;
    while remaining_rock_falls > 0 {
        let (rock_idx, mut rock) = rock_iter.next().expect("rocks should be infinite");
        let mut rock_height = chamber.len() + 3;

        loop {
            let (jet_idx, jet_ch) = jet_iter.next().expect("hot air jets should be infinite");
            if let Some(shifted_rock) = shift_rock(rock, jet_ch) {
                if !rock_collides(shifted_rock, rock_height, &chamber) {
                    rock = shifted_rock;
                }
            };
            let should_stop = rock_height == 0 || rock_collides(rock, rock_height - 1, &chamber);
            if !should_stop {
                rock_height -= 1;
                continue;
            }

            for (i, rock_row) in rock.iter().enumerate() {
                if *rock_row == 0 {
                    continue;
                }
                let chamber_index = rock_height + i;
                while chamber_index >= chamber.len() {
                    chamber.push(0);
                }
                chamber[chamber_index] |= rock_row;
            }

            if cyclic_height.is_some() {
                break;
            }

            let chamber_height = chamber.len();
            if chamber_height < CHAMBER_TOP_HEIGHT {
                break;
            }
            let chamber_top: [Row; CHAMBER_TOP_HEIGHT] = chamber
                [chamber_height - CHAMBER_TOP_HEIGHT..]
                .try_into()
                .expect("chamber should have enough height");

            let chamber_top_sealed = chamber_top.windows(2).any(|w| (w[0] | w[1]) == 0b1111111);
            if !chamber_top_sealed {
                break;
            }

            let combined_idx = (rock_idx, jet_idx);
            if let Some(prev) = &previous_chamber_tops.get(&combined_idx) {
                if chamber_top == prev.chamber_top {
                    let height_diff = chamber_height - prev.chamber_height;
                    let fallen_rocks_diff = prev.remaining_rock_falls - remaining_rock_falls;
                    let remaining_cycles = (remaining_rock_falls - 1) / fallen_rocks_diff;
                    cyclic_height = Some(remaining_cycles * height_diff as u64);
                    remaining_rock_falls -= remaining_cycles * fallen_rocks_diff;
                    break;
                }
            }
            previous_chamber_tops.insert(
                combined_idx,
                ChamberTopMemo {
                    chamber_top,
                    chamber_height,
                    remaining_rock_falls,
                },
            );

            break;
        }
        remaining_rock_falls -= 1;
    }

    chamber.len() as u64 + cyclic_height.unwrap_or(0)
}

type Row = u8;
type Rock = [Row; 4];

// First byte is bottom row of rock.
#[rustfmt::skip]
const ROCKS: [Rock; 5] = [
    [
        0b0011110,
        0b0000000,
        0b0000000,
        0b0000000,
    ],
    [
        0b0001000,
        0b0011100,
        0b0001000,
        0b0000000,
    ],
    [
        0b0011100,
        0b0000100,
        0b0000100,
        0b0000000,
    ],
    [
        0b0010000,
        0b0010000,
        0b0010000,
        0b0010000,
    ],
    [
        0b0011000,
        0b0011000,
        0b0000000,
        0b0000000,
    ],
];

const CHAMBER_TOP_HEIGHT: usize = 4;
struct ChamberTopMemo {
    chamber_top: [Row; CHAMBER_TOP_HEIGHT],
    chamber_height: usize,
    remaining_rock_falls: u64,
}

fn shift_rock(rock: Rock, jet_ch: char) -> Option<Rock> {
    match jet_ch {
        '<' => {
            if rock.iter().all(|row| row & 0b1000000 == 0) {
                return Some(rock.map(|row| row << 1));
            };
        }
        '>' => {
            if rock.iter().all(|row| row & 1 == 0) {
                return Some(rock.map(|row| row >> 1));
            }
        }
        _ => unreachable!("unexpected character '{jet_ch}'"),
    }
    None
}

fn rock_collides(rock: Rock, rock_height: usize, chamber: &[Row]) -> bool {
    rock.iter().enumerate().any(|(i, rock_row)| {
        let chamber_row = chamber.get(rock_height + i).unwrap_or(&0);
        rock_row & chamber_row != 0
    })
}
