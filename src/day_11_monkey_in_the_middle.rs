use std::collections::{BinaryHeap, HashMap, VecDeque};

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
    let mut monkeys: Vec<_> = input.split("\n\n").map(Monkey::parse).collect();
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

    let inspection_counts: BinaryHeap<_> = monkeys.iter().map(|m| m.inspections_count).collect();
    // BinaryHeap is max-sorted so the first two values are the biggest.
    let monkey_business: u64 = inspection_counts.iter().take(2).product();

    monkey_business
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    div_test_divisor: u64,
    if_true_receiver: usize,
    if_false_receiver: usize,
    inspections_count: u64,
}

struct Operation {
    lhs: Operand,
    rhs: Operand,
    operator: Operator,
}

enum Operand {
    Old,
    Num(u64),
}

enum Operator {
    Add,
    Mult,
}

impl Monkey {
    fn parse(input: &str) -> Monkey {
        let data: HashMap<_, _> = input
            .lines()
            .skip(1)
            .map(|line| line.trim_start().split_once(": ").expect("invalid line"))
            .collect();

        let items: VecDeque<u64> = data["Starting items"]
            .split(", ")
            .map(|s| s.parse().expect("item should be a valid number"))
            .collect();

        let operation_words: Vec<_> = data["Operation"].split(' ').collect();
        let operation = match operation_words[..] {
            ["new", "=", lhs, op, rhs] if op == "+" || op == "*" => Operation::parse(lhs, rhs, op),
            _ => unreachable!("invalid operation {}", data["Operation"]),
        };

        let div_test_divisor = parse_last_number(data["Test"]) as u64;
        let if_true_receiver = parse_last_number(data["If true"]);
        let if_false_receiver = parse_last_number(data["If false"]);

        Monkey {
            items,
            operation,
            div_test_divisor,
            if_true_receiver,
            if_false_receiver,
            inspections_count: 0,
        }
    }

    fn inspect_and_throw_item(
        &mut self,
        mod_divisor: u64,
        relief_after_inspection: bool,
    ) -> Option<(u64, usize)> {
        let Some(item) = self.items.pop_front() else {
            return None
        };
        self.inspections_count += 1;
        let mut new_worry_level = self.operation.call(item) % mod_divisor;
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

impl Operation {
    fn parse(lhs: &str, rhs: &str, operator: &str) -> Operation {
        Operation {
            lhs: Operand::parse(lhs),
            rhs: Operand::parse(rhs),
            operator: Operator::parse(operator),
        }
    }

    fn call(&self, old: u64) -> u64 {
        let lhs = self.lhs.value(old);
        let rhs = self.rhs.value(old);
        self.operator.call(lhs, rhs)
    }
}

impl Operand {
    fn parse(s: &str) -> Operand {
        if s == "old" {
            Operand::Old
        } else {
            let num: u64 = s.parse().expect("invalid number");
            Operand::Num(num)
        }
    }

    fn value(&self, old: u64) -> u64 {
        match self {
            Operand::Old => old,
            Operand::Num(num) => *num,
        }
    }
}

impl Operator {
    fn parse(operator: &str) -> Operator {
        match operator {
            "+" => Operator::Add,
            "*" => Operator::Mult,
            _ => unreachable!("invalid operator {operator}"),
        }
    }

    fn call(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mult => lhs * rhs,
        }
    }
}

fn parse_last_number(s: &str) -> usize {
    s.split(' ')
        .last()
        .expect("string should have at least one word")
        .parse()
        .expect("invalid number")
}
