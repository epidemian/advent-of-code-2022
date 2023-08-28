use std::collections::HashSet;

use rayon::prelude::*;

pub fn run(input: &str) -> String {
    let mut sensors = parse_sensors(input);
    sensors.sort_by_key(|s| s.position.0);

    let is_sample = sensors.len() < 20;
    let row = if is_sample { 10 } else { 2_000_000 };
    let size = if is_sample { 20 } else { 4_000_000 };

    let excluded_positions_count = excluded_positions_count_at_row(&sensors, row);

    let (beacon_x, beacon_y) =
        find_distress_signal_beacon(&sensors, size).expect("distress signal beacon not found");
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

    Sensor {
        position: (sx, sy),
        beacon_position: (bx, by),
        beacon_distance: (sx - bx).abs() + (sy - by).abs(),
    }
}

fn excluded_positions_count_at_row(sensors: &[Sensor], y: i64) -> i64 {
    // A beacon can be detected by multiple sensors. Avoid double counting.
    let beacon_positions = sensors.iter().map(|s| s.beacon_position);
    let row_beacons: HashSet<_> = beacon_positions.filter(|(_bx, by)| *by == y).collect();

    let segments = get_contiguous_exclusion_row_segments(sensors, y);
    let [(start, end)] = segments[..] else {
        panic!("expected only one exclusion segment at y={y}")
    };

    end - start + 1 - row_beacons.len() as i64
}

fn find_distress_signal_beacon(sensors: &[Sensor], size: i64) -> Option<Point> {
    (0..=size).into_par_iter().find_map_any(|y| {
        let segments = get_contiguous_exclusion_row_segments(sensors, y);
        let &(_start_x, end_x) = segments.first().expect("at least one segment expected");

        if end_x < size {
            return Some((end_x + 1, y));
        }
        None
    })
}

// Assumes sensors are sorted by x coordinate.
fn get_contiguous_exclusion_row_segments(sensors: &[Sensor], y: i64) -> Vec<(i64, i64)> {
    let mut segments = vec![];
    for sensor in sensors {
        let (sx, sy) = sensor.position;
        let d = sensor.beacon_distance - (y - sy).abs();
        if d < 0 {
            continue;
        }
        let mut start = sx - d;
        let mut end = sx + d;

        while let Some(&(last_start, last_end)) = segments.last() {
            if start > last_end + 1 {
                // Doesn't overlap with last segment.
                break;
            }
            // Overlaps. Extend this one and discard last one.
            start = start.min(last_start);
            end = end.max(last_end);
            segments.pop();
        }

        segments.push((start, end))
    }
    segments
}
