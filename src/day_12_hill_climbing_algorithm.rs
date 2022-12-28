use crate::dijkstra;

pub fn run(input: &str) -> String {
    let (heightmap, start, end) = parse_input(input);

    let shortest_path_from_start =
        shortest_path(&heightmap, &start, &end).expect("there should be a path from start to end");

    let mut shortest_path_from_any_bottommost_point = shortest_path_from_start;
    for (y, row) in heightmap.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                if let Some(shortest_path) = shortest_path(&heightmap, &(x, y), &end) {
                    shortest_path_from_any_bottommost_point =
                        shortest_path_from_any_bottommost_point.min(shortest_path);
                };
            }
        }
    }

    format!("{shortest_path_from_start} {shortest_path_from_any_bottommost_point}")
}

type Point = (usize, usize);

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Point, Point) {
    let char_map: Vec<_> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let height = char_map.len();
    let width = char_map[0].len();

    let start = map_points_iter(width, height)
        .find(|&(x, y)| char_map[y][x] == b'S')
        .expect("start position not found");
    let end = map_points_iter(width, height)
        .find(|&(x, y)| char_map[y][x] == b'E')
        .expect("end position not found");

    let mut heightmap = vec![vec![0; width]; height];
    for (x, y) in map_points_iter(width, height) {
        heightmap[y][x] = match char_map[y][x] {
            b'S' => 0,
            b'E' => 25,
            ch => ch - b'a',
        };
    }

    (heightmap, start, end)
}

fn map_points_iter(width: usize, height: usize) -> impl Iterator<Item = Point> {
    (0..height).flat_map(move |y| (0..width).map(move |x| (x, y)))
}

fn shortest_path(heightmap: &Vec<Vec<u8>>, start: &Point, end: &Point) -> Option<usize> {
    let heightmap_height = heightmap.len();
    let heightmap_width = heightmap[0].len();

    let neighbors = |&(x, y): &Point| {
        let curr_height: u8 = heightmap[y][x];
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(move |(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
            .filter(|(x, y)| *y < heightmap_height && *x < heightmap_width)
            .filter(move |&(x, y)| {
                let neighbor_height = heightmap[y][x];
                neighbor_height <= curr_height + 1
            })
            .map(|neighbor_point| (neighbor_point, 1))
    };

    dijkstra::shortest_path(start, end, neighbors)
}
