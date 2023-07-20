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
    // Encode a double-linked circular list as two arrays of previous and next number IDs.
    // Use a smaller integer for number IDs so that the vectors are move cache-friendly.
    let mut prevs: Vec<u16> = (0..len as u16)
        .map(|i| if i == 0 { len as u16 - 1 } else { i - 1 })
        .collect();
    let mut nexts: Vec<u16> = (0..len as u16)
        .map(|i| if i == len as u16 - 1 { 0 } else { i + 1 })
        .collect();

    for _ in 0..mix_count {
        for id in 0..len {
            let num = numbers[id as usize];

            // Skip unnecessary "laps" around the list by taking the reminder.
            let mut moves = num % (len - 1) as i64;
            if moves == 0 {
                continue;
            }

            let half_len = len as i64 / 2;
            // If it's a big move forward, the target is closer by moving backwards, and viceversa.
            if moves > half_len {
                moves = -(len as i64) + moves + 1;
            } else if moves < -half_len {
                moves = len as i64 + moves - 1
            }

            let target_id = if moves >= 0 {
                (0..moves).fold(id, |id, _| nexts[id] as usize)
            } else {
                // When moving backward, Move 1 more so that target_id ends up being the ID *after*
                // which we'll insert `id`.
                (0..-moves + 1).fold(id, |id, _| prevs[id] as usize)
            };

            // Remove num from previous position.
            nexts[prevs[id] as usize] = nexts[id];
            prevs[nexts[id] as usize] = prevs[id];
            // Insert num after target_id.
            prevs[id] = target_id as u16;
            nexts[id] = nexts[target_id];
            prevs[nexts[id] as usize] = id as u16;
            nexts[prevs[id] as usize] = id as u16;
        }
    }

    let zero_id = numbers
        .iter()
        .position(|n| *n == 0)
        .expect("0 must be on the list");

    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| {
            let id = (0..offset).fold(zero_id, |id, _| nexts[id] as usize);
            numbers[id]
        })
        .sum()
}
