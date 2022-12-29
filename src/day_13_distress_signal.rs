use std::cmp::Ordering;

pub fn run(input: &str) -> String {
    let pairs: Vec<(Value, Value)> = input
        .split("\n\n")
        .map(|s| {
            let (left, right) = s.split_once("\n").expect("expected a pair of values");
            (parse_packet(left), parse_packet(right))
        })
        .collect();

    let ordered_pairs_indices_sum: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_index, (left, right))| left < right)
        .map(|(index, _pair)| index + 1)
        .sum();

    let divider_packets = [parse_packet("[[2]]"), parse_packet("[[6]]")];

    let mut all_packets: Vec<_> = pairs
        .iter()
        .flat_map(|(left, right)| [left, right])
        .chain(divider_packets.iter())
        .collect();
    all_packets.sort();

    let decoder_key: usize = all_packets
        .iter()
        .enumerate()
        .filter(|(_index, value)| divider_packets.contains(value))
        .map(|(index, _value)| index + 1)
        .product();

    format!("{ordered_pairs_indices_sum} {decoder_key}")
}

#[derive(Eq, PartialEq)]
enum Value {
    Num(u32),
    List(Vec<Value>),
}
use Value::*;

fn parse_packet(s: &str) -> Value {
    let mut bytes = s.bytes().peekable();
    let mut list_stack: Vec<Vec<Value>> = Vec::new();
    while let Some(ch) = bytes.next() {
        match ch {
            b'[' => {
                list_stack.push(Vec::new());
            }
            b']' => {
                let Some(last_list) = list_stack.pop() else {
                    panic!("unmatched ]")
                };
                let value = Value::List(last_list);
                if let Some(parent_list) = list_stack.last_mut() {
                    parent_list.push(value);
                } else {
                    // List is topmost list.
                    return value;
                }
            }
            b',' => {}
            b'0'..=b'9' => {
                let mut n = (ch - b'0') as u32;
                while let Some(next_ch) = bytes.next_if(|ch| matches!(ch, b'0'..=b'9')) {
                    n = n * 10 + (next_ch - b'0') as u32;
                }
                let Some(list) = list_stack.last_mut() else {
                    panic!("number must appear inside a list")
                };
                list.push(Value::Num(n))
            }
            _ => panic!("unexpected character '{}'", ch as char),
        }
    }
    panic!("unclosed list")
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
