use std::collections::HashSet;

pub fn run(input: &str) -> String {
    let sensors = parse_sensors(input);

    let excluded_positions_count = excluded_positions_count_at_row(&sensors, 2_000_000);

    let (beacon_x, beacon_y) =
        find_distress_signal_beacon(&sensors).expect("distress signal beacon not found");
    let tuning_frequency = beacon_x * 4_000_000 + beacon_y;

    format!("{excluded_positions_count} {tuning_frequency}",)
}

type Point = (i64, i64);
struct Sensor {
    position: Point,
    beacon_position: Point,
    beacon_distance: i64,
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    input.lines().map(parse_sensor).collect()
}

fn parse_sensor(line: &str) -> Sensor {
    let numbers: Vec<i64> = line
        .split(&['=', ',', ':'])
        .filter_map(|s| s.parse().ok())
        .collect();

    let [sx, sy, bx, by] = numbers[..] else {
        panic!("invalid line '{line}', should have 4 numbers")
    };
    let beacon_distance = distance((sx, sy), (bx, by));

    Sensor {
        position: (sx, sy),
        beacon_position: (bx, by),
        beacon_distance,
    }
}

fn excluded_positions_count_at_row(sensors: &[Sensor], y: i64) -> i64 {
    let segments = sensors_exclusion_row_segments(sensors, y);

    // A beacon can be detected by multiple sensors. We want unique positions.
    let beacon_positions: HashSet<_> = sensors.iter().map(|s| s.beacon_position).collect();
    let beacons_at_row_y_count = beacon_positions.iter().filter(|(_bx, by)| *by == y).count();

    let max_end = segments.iter().map(|(_start, end)| end).max().unwrap();
    let min_start = segments.iter().map(|(start, _end)| start).min().unwrap();

    // Note: assumes no "holes" in this row.
    max_end - min_start + 1 - beacons_at_row_y_count as i64
}

fn find_distress_signal_beacon(sensors: &[Sensor]) -> Option<Point> {
    let size = 4_000_000;
    for y in 0..=size {
        let segments = sensors_exclusion_row_segments(sensors, y);
        let mut x = 0;
        for (start_x, end_x) in segments {
            if start_x == x + 2 {
                // There is a gap of just one position between this exclusion
                // segment and the previous one. Beacon found!
                return Some((x + 1, y));
            }
            x = x.max(end_x);
            if x >= size {
                break;
            }
        }
    }
    None
}

fn sensors_exclusion_row_segments(sensors: &[Sensor], y: i64) -> Vec<(i64, i64)> {
    let mut segments: Vec<_> = sensors
        .iter()
        .filter_map(|sensor| {
            let &Sensor {
                beacon_distance,
                position: (sx, sy),
                ..
            } = sensor;
            let d = (y - sy).abs();
            if d > beacon_distance {
                return None;
            }
            let start_x = sx - (beacon_distance - d);
            let end_x = sx + (beacon_distance - d);
            Some((start_x, end_x))
        })
        .collect();
    segments.sort_by_key(|(start_x, _end_x)| *start_x);
    segments
}

fn distance((x1, y1): Point, (x2, y2): Point) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}
