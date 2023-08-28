pub fn run(input: &str) -> String {
    let total = input.lines().map(parse_snafu).sum();
    num_to_snafu(total)
}

fn parse_snafu(s: &str) -> i64 {
    let mut res = 0;
    let mut mul = 1;
    for ch in s.chars().rev() {
        let digit = match ch {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!("invalid SNAFU digit '{ch}'"),
        };
        res += mul * digit;
        mul *= 5;
    }
    res
}

fn num_to_snafu(n: i64) -> String {
    assert!(n >= 0, "This function doesn't work for negative numbers");
    let mut n = n;
    let mut digits = vec![];
    if n == 0 {
        digits.push('0')
    }
    while n != 0 {
        let rem = (n + 2) % 5;
        n = (n + 2) / 5;
        digits.push(match rem - 2 {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!("i done goofed"),
        });
    }
    digits.into_iter().rev().collect()
}
