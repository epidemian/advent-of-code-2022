use std::collections::VecDeque;

pub fn run(input: &str) -> String {
    format!(
        "{} {}",
        run_monkey_in_the_middle_rounds(input, 20, true),
        run_monkey_in_the_middle_rounds(input, 10_000, false)
    )
}

fn run_monkey_in_the_middle_rounds(
    input: &str,
    rounds_count: usize,
    relief_after_inspection: bool,
) -> u64 {
    let mut monkeys: Vec<_> = input.split("\n\n").map(parse_monkey).collect();
    // Trick: keep track of the product of all divisibility tests' divisors so
    // that each time the worry level for an item is increased we can mod that
    // number with this and keep it from ballooning out of control.
    let divisors_product: u64 = monkeys.iter().map(|m| m.div_test_divisor).product();

    for _ in 0..rounds_count {
        for monkey_index in 0..monkeys.len() {
            while let Some(throw) = monkeys[monkey_index]
                .inspect_and_throw_item(divisors_product, relief_after_inspection)
            {
                let (item, receiver_monkey_index) = throw;
                monkeys[receiver_monkey_index].items.push_back(item);
            }
        }
    }

    let mut inspection_counts: Vec<_> = monkeys.iter().map(|m| m.inspections_count).collect();
    inspection_counts.sort();
    let monkey_business: u64 = inspection_counts[inspection_counts.len() - 2..]
        .iter()
        .product();

    monkey_business
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    div_test_divisor: u64,
    if_true_receiver: usize,
    if_false_receiver: usize,
    inspections_count: u64,
}

impl Monkey {
    fn inspect_and_throw_item(
        &mut self,
        mod_divisor: u64,
        relief_after_inspection: bool,
    ) -> Option<(u64, usize)> {
        let Some(item) = self.items.pop_front() else {
            return None
        };
        self.inspections_count += 1;
        let mut new_worry_level = (self.operation)(item) % mod_divisor;
        if relief_after_inspection {
            new_worry_level /= 3
        };
        let receiver = if new_worry_level % self.div_test_divisor == 0 {
            self.if_true_receiver
        } else {
            self.if_false_receiver
        };
        Some((new_worry_level, receiver))
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let lines_data: Vec<_> = input
        .lines()
        .skip(1)
        .map(|line| line.split_once(": ").expect("line should have a colon").1)
        .collect();
    let items: VecDeque<u64> = lines_data[0]
        .split(", ")
        .map(|s| s.parse().expect("item should be a valid number"))
        .collect();
    let operation_words: Vec<_> = lines_data[1].split(' ').collect();
    let operation: Box<dyn Fn(u64) -> u64> = match operation_words[..] {
        ["new", "=", "old", "+", "old"] => Box::new(|old: u64| old + old),
        ["new", "=", "old", "+", rhs] => {
            let rhs: u64 = rhs.parse().unwrap();
            Box::new(move |old: u64| old + rhs)
        }
        ["new", "=", "old", "*", "old"] => Box::new(|old: u64| old * old),
        ["new", "=", "old", "*", rhs] => {
            let rhs: u64 = rhs.parse().unwrap();
            Box::new(move |old: u64| old * rhs)
        }
        _ => unreachable!("invalid operation {}", lines_data[1]),
    };
    let div_test_divisor: u64 = lines_data[2]
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .expect("invalid number");
    let if_true_receiver: usize = lines_data[3]
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .expect("invalid number");
    let if_false_receiver: usize = lines_data[4]
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .expect("invalid number");

    Monkey {
        items,
        operation,
        div_test_divisor,
        if_true_receiver,
        if_false_receiver,
        inspections_count: 0,
    }
}
