pub fn run(input: &str) -> String {
    let sensors = parse_sensors(input);

    // let min_x = -10;
    // let max_x = 30;
    // let y = 10;

    let min_x = -1_000_000;
    let max_x = 10_000_000;
    let y = 2_000_000;

    let excluded_positions_count = (min_x..max_x)
        .filter(|&x| {
            sensors.iter().any(|sensor| {
                let dist = distance(sensor.position, (x, y));
                let in_sensor_exclusion_zone = dist <= sensor.beacon_distance;
                in_sensor_exclusion_zone && !(sensor.beacon_position == (x, y))
            })
        })
        .count();

    format!("{excluded_positions_count}")
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    input.lines().map(parse_sensor).collect()
}

fn parse_sensor(line: &str) -> Sensor {
    let numbers: Vec<i32> = line
        .split(&['=', ',', ':'])
        .filter_map(|s| s.parse().ok())
        .collect();

    let [sx, sy, bx, by] = numbers[..] else {
        panic!("invalid line {line}, should have 4 numbers")
    };
    let beacon_distance = distance((sx, sy), (bx, by));

    Sensor {
        position: (sx, sy),
        beacon_position: (bx, by),
        beacon_distance,
    }
}

fn distance((x1, y1): Point, (x2, y2): Point) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

type Point = (i32, i32);
#[derive(Debug)]
struct Sensor {
    position: Point,
    beacon_position: Point,
    beacon_distance: i32,
}
