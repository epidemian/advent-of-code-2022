use crate::dijkstra;

pub fn run(input: &str) -> String {
    let (heightmap, start, end) = parse_input(input);

    let distances = {
        let heightmap_height = heightmap.len();
        let heightmap_width = heightmap[0].len();
        // Take a reference so that closure doesn't take ownership of the Vec.
        let heightmap = &heightmap;
        // Note: this function returns the points from we could have come from,
        // because we're calculating distances starting from the *end* point.
        let neighbors = |&(x, y): &Point| {
            let curr_height: u8 = heightmap[y][x];
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .map(move |(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
                .filter(|(x, y)| *y < heightmap_height && *x < heightmap_width)
                .filter(move |&(nx, ny)| {
                    let neighbor_height = heightmap[ny][nx];
                    curr_height <= neighbor_height + 1
                })
        };
        dijkstra::shortest_path_distances(&end, neighbors)
    };

    let shortest_path_from_start = distances[&start];
    let shortest_path_from_any_bottommost_point = distances
        .iter()
        .filter(|((x, y), _dist)| heightmap[*y][*x] == 0)
        .map(|(_point, dist)| dist)
        .min()
        .expect("there should be a point at height 0 that reaches the end");

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
