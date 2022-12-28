use std::cmp::Ordering;

use serde_json;

pub fn run(input: &str) -> String {
    let pairs: Vec<(Value, Value)> = input
        .split("\n\n")
        .map(|s| {
            let (left, right) = s.split_once("\n").expect("expected a pair of values");
            (parse_value(left), parse_value(right))
        })
        .collect();

    let ordered_pairs_indices_sum: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_index, (left, right))| left < right)
        .map(|(index, _pair)| index + 1)
        .sum();

    let divider_packets = [parse_value("[[2]]"), parse_value("[[6]]")];

    let mut all_packets: Vec<_> = pairs
        .iter()
        .flat_map(|(left, right)| [left, right])
        .chain(divider_packets.iter())
        .collect();
    all_packets.sort();

    let divider_packets_indices_product: usize = all_packets
        .iter()
        .enumerate()
        .filter(|(_index, value)| divider_packets.contains(value))
        .map(|(index, _value)| index + 1)
        .product();

    format!("{ordered_pairs_indices_sum} {divider_packets_indices_product}")
}

#[derive(Eq, PartialEq)]
enum Value {
    Num(u64),
    List(Vec<Value>),
}
use Value::*;

fn parse_value(s: &str) -> Value {
    // TODO: Remove serde_json dependency.
    let json: serde_json::Value = serde_json::from_str(s).expect("expected a JSON value");
    json_to_value(&json)
}

fn json_to_value(json: &serde_json::Value) -> Value {
    match json {
        serde_json::Value::Number(n) => Num(n.as_u64().unwrap()),
        serde_json::Value::Array(json_values) => {
            List(json_values.iter().map(json_to_value).collect())
        }
        _ => unreachable!("unexpected JSON value {}", json),
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Num(l_num), Num(r_num)) => l_num.cmp(r_num),
            (List(l_values), List(r_values)) => cmp_lists(&l_values, &r_values),
            (Num(l_num), List(r_values)) => cmp_lists(&[Num(*l_num)], &r_values),
            (List(l_values), Num(r_num)) => cmp_lists(&l_values, &[Num(*r_num)]),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cmp_lists(l_values: &[Value], r_values: &[Value]) -> Ordering {
    match (l_values, r_values) {
        ([], []) => Ordering::Equal,
        ([], _) => Ordering::Less,
        (_, []) => Ordering::Greater,
        ([l_head, l_tail @ ..], [r_head, r_tail @ ..]) => {
            l_head.cmp(r_head).then_with(|| cmp_lists(l_tail, r_tail))
        }
    }
}
