use std::{collections::HashMap, fmt};

pub fn run(input: &str) -> String {
    let monkeys: HashMap<_, _> = input.lines().map(parse_monkey).collect();

    let ans_1 = eval(parse_id("root"), &monkeys);
    let ans_2 = solve_for(parse_id("humn"), &monkeys);

    format!("{ans_1} {ans_2}")
}

// Use a 4-letter array for monkey IDs instead of &str to avoid having explicit lifetime annotations
// everywhere. And use a opaque struct instead of a type alias so that we can implement Debug for it
// and interpolate IDs in error messages.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct MonkeyId([u8; 4]);

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum MonkeyJob {
    Num(i64),
    Op(Op, MonkeyId, MonkeyId),
}

impl fmt::Debug for MonkeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = std::str::from_utf8(&self.0).unwrap();
        write!(f, "\"{s}\"")
    }
}

fn parse_monkey(line: &str) -> (MonkeyId, MonkeyJob) {
    let (id, job) = line.split_once(": ").expect("line should have a colon");
    (parse_id(id), parse_job(job))
}

fn parse_id(s: &str) -> MonkeyId {
    let id = s
        .as_bytes()
        .try_into()
        .expect("monkey ID should be 4 letters");
    MonkeyId(id)
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

fn eval(id: MonkeyId, monkeys: &HashMap<MonkeyId, MonkeyJob>) -> i64 {
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

fn solve_for(var_id: MonkeyId, monkeys: &HashMap<MonkeyId, MonkeyJob>) -> i64 {
    let mut var_uses = monkeys.iter().filter_map(|(id, job)| match job {
        MonkeyJob::Op(op, lhs, rhs) if *lhs == var_id || *rhs == var_id => Some((id, op, lhs, rhs)),
        _ => None,
    });

    let Some((use_id, op, lhs, rhs)) = var_uses.next() else {
        panic!("found no uses of {var_id:?}")
    };

    if let Some((other_use_id, ..)) = var_uses.next() {
        panic!("found more than one use of {var_id:?}: used in {use_id:?} and {other_use_id:?}")
    }

    assert!(lhs != rhs, "variable {var_id:?} used twice in {use_id:?}");

    if *use_id == parse_id("root") {
        // When we reach the root equation our var_id must be equal to the other operand.
        let other = if *lhs == var_id { rhs } else { lhs };
        return eval(*other, monkeys);
    }

    let use_id_res = solve_for(*use_id, monkeys);

    // Solve the math equation depending on whether our "variable" is the LHS or the RHS
    if var_id == *lhs {
        // use_id = var_id <op> rhs
        let rhs_res = eval(*rhs, monkeys);
        match *op {
            Op::Add => use_id_res - rhs_res,
            Op::Sub => use_id_res + rhs_res,
            Op::Mul => use_id_res / rhs_res,
            Op::Div => use_id_res * rhs_res,
        }
    } else {
        // use_id = lhs <op> var_id
        let lhs_res = eval(*lhs, monkeys);
        match *op {
            Op::Add => use_id_res - lhs_res,
            Op::Sub => lhs_res - use_id_res,
            Op::Mul => use_id_res / lhs_res,
            Op::Div => lhs_res / use_id_res,
        }
    }
}
