pub fn run(input: &str) -> String {
    let total = input.lines().map(parse_snafu).sum();
    let snafu_total = num_to_snafu(total);
    format!("{snafu_total}")
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

// This function doesn't work for negative numbers.
fn num_to_snafu(n: i64) -> String {
    let mut n = n;
    let mut digits = vec![];
    if n == 0 {
        digits.push('0')
    }
    while n != 0 {
        let rem = (n + 2) % 5;
        n = (n + 2) / 5;
        let snafu_digit = match rem - 2 {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!("i done goofed"),
        };
        digits.push(snafu_digit);
    }
    digits.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        assert_eq!(super::run(SAMPLE), "2=-1=0")
    }
    const SAMPLE: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test]
    fn brochure_sample_numbers() {
        let nums = [
            (1, "1"),
            (2, "2"),
            (3, "1="),
            (4, "1-"),
            (5, "10"),
            (6, "11"),
            (7, "12"),
            (8, "2="),
            (9, "2-"),
            (10, "20"),
            (15, "1=0"),
            (20, "1-0"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ];
        for &(n, snafu) in nums.iter() {
            assert_eq!(n, super::parse_snafu(snafu));
            assert_eq!(snafu, super::num_to_snafu(n));
        }
    }
}
