use std::collections::HashMap;

pub fn run(input: &str) -> String {
    let monkeys: HashMap<_, _> = input.lines().map(parse_monkey).collect();

    let ans_1 = eval(parse_id("root"), &monkeys);

    format!("{ans_1}")
}

type MonkeyId = [u8; 4];

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum MonkeyJob {
    Num(u64),
    Op(Op, MonkeyId, MonkeyId),
}

fn parse_monkey(line: &str) -> (MonkeyId, MonkeyJob) {
    let (id, job) = line.split_once(": ").expect("line should have a colon");
    (parse_id(id), parse_job(job))
}

fn parse_id(s: &str) -> MonkeyId {
    s.as_bytes()
        .try_into()
        .expect("monkey ID should be 4 letters")
}

fn parse_job(s: &str) -> MonkeyJob {
    if let Ok(num) = s.parse() {
        return MonkeyJob::Num(num);
    }
    let op_parts: Vec<_> = s.split(' ').collect();
    let &[lhs, op, rhs] = &op_parts[..] else {
        panic!("invalid math operation format")
    };
    let lhs = parse_id(lhs);
    let rhs = parse_id(rhs);
    let op = match op {
        "+" => Op::Add,
        "-" => Op::Sub,
        "*" => Op::Mul,
        "/" => Op::Div,
        _ => unreachable!("invalid operator {op}"),
    };
    MonkeyJob::Op(op, lhs, rhs)
}

fn eval(id: MonkeyId, monkeys: &HashMap<MonkeyId, MonkeyJob>) -> u64 {
    let Some(job) = monkeys.get(&id) else {
        panic!("id {id:?} not found");
    };

    match job {
        MonkeyJob::Num(n) => *n,
        MonkeyJob::Op(op, lhs, rhs) => {
            let lhs = eval(*lhs, monkeys);
            let rhs = eval(*rhs, monkeys);
            match op {
                Op::Add => lhs + rhs,
                Op::Sub => lhs - rhs,
                Op::Mul => lhs * rhs,
                Op::Div => lhs / rhs,
            }
        }
    }
}
