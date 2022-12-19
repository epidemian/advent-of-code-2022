use std::collections::HashSet;

pub fn run(input: &str) -> String {
    let data = input.as_bytes();
    let start_of_packet_marker =
        find_marker(data, 4).expect("there should be a start-of-packet marker");
    let start_of_message_marker =
        find_marker(data, 14).expect("there should be a start-of-message marker");
    format!("{start_of_packet_marker} {start_of_message_marker}")
}

fn find_marker(data: &[u8], distinct_chars_count: usize) -> Option<usize> {
    (distinct_chars_count..=data.len()).find(|&i| {
        let last_n_chars = &data[i - distinct_chars_count..i];
        let unique_chars: HashSet<_> = last_n_chars.iter().collect();
        unique_chars.len() == distinct_chars_count
    })
}
