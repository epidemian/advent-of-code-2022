pub fn run(input: &str) -> String {
    let rocks: [&[Point]; 5] = [
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
    let mut rock_iter = rocks.iter().cycle();
    let mut jet_iter = input.chars().cycle();
    let mut chamber: Vec<Row> = vec![];
    let left = (-1, 0);
    let right = (1, 0);
    let down = (0, -1);

    for _ in 0..2022 {
        let mut rock = rock_iter.next().expect("rocks should be infinite").to_vec();
        move_rock(&mut rock, (2, chamber.len() as i32 + 3), &chamber);

        loop {
            let jet_ch = jet_iter.next().expect("hot air jets should be infinite");
            let jet_dir = match jet_ch {
                '<' => left,
                '>' => right,
                _ => unreachable!("unexpected character '{jet_ch}'"),
            };
            move_rock(&mut rock, jet_dir, &chamber);
            let should_stop = !move_rock(&mut rock, down, &chamber);
            if should_stop {
                for (x, y) in rock.iter() {
                    while *y as usize >= chamber.len() {
                        chamber.push(Default::default());
                    }
                    chamber[*y as usize][*x as usize] = ROCK;
                }
                break;
            }
        }
    }

    format!("{}", chamber.len())
}

type Point = (i32, i32);
type Row = [u8; 7];
const ROCK: u8 = 1;

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
