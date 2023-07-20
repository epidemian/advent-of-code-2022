pub fn run(input: &str) -> String {
    let numbers: Vec<i64> = input
        .lines()
        .map(|line| line.parse().expect("invalid number"))
        .collect();

    let decrypted_numbers: Vec<i64> = numbers.iter().map(|n| *n * 811589153).collect();

    format!("{} {}", mix(&numbers, 1), mix(&decrypted_numbers, 10))
}

fn mix(numbers: &[i64], mix_count: usize) -> i64 {
    let len = numbers.len();
    let mut ids: Vec<usize> = (0..len).collect();
    for _ in 0..mix_count {
        for id in 0..ids.len() {
            let mut pos = ids
                .iter()
                .position(|i| *i == id)
                .expect("id must be on the list");

            let num = numbers[id];
            if num >= 0 {
                // Move forward.
                let num_mod = num as usize % (len - 1);
                for _ in 0..num_mod {
                    let next_pos = (pos + 1) % len;
                    ids.swap(pos, next_pos);
                    pos = next_pos;
                }
            } else {
                // Move backward.
                let num_mod = (-num) as usize % (len - 1);
                for _ in 0..num_mod {
                    let next_pos = ((pos as isize - 1).rem_euclid(len as isize)) as usize;
                    ids.swap(pos, next_pos);
                    pos = next_pos;
                }
            }
        }
    }

    let zero_pos = ids
        .iter()
        .position(|id| numbers[*id] == 0)
        .expect("0 must be on the list");
    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| numbers[ids[(zero_pos + offset) % len]])
        .sum()
}
