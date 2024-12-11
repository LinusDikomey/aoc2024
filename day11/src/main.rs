use std::collections::HashMap;

use aoch::{int, ints};
use indicatif::ProgressIterator;

fn main() {
    let input = ints(include_str!("../../input/day11.txt"));
    let mut part1 = input.clone();
    for _ in (0..25).progress() {
        step(&mut part1);
    }
    println!("Part 1: {}", part1.len());
    let mut m: HashMap<i64, u64> = HashMap::new();
    for item in input {
        *m.entry(item).or_default() += 1;
    }
    for _ in (0..75).progress() {
        let mut m2 = HashMap::new();
        for (s, count) in m {
            let (a, b) = stone(s);
            *m2.entry(a).or_default() += count;
            if let Some(b) = b {
                *m2.entry(b).or_default() += count;
            }
        }
        m = m2;
    }
    println!("Part 2: {}", m.values().copied().sum::<u64>());
}

fn step(input: &mut Vec<i64>) {
    let mut i = 0;
    while i < input.len() {
        i += match stone(input[i]) {
            (a, Some(b)) => {
                input[i] = a;
                input.insert(i + 1, b);
                2
            }
            (a, None) => {
                input[i] = a;
                1
            }
        }
    }
}

fn stone(stone: i64) -> (i64, Option<i64>) {
    match stone {
        0 => (1, None),
        n => {
            let s = n.to_string();
            if s.len() % 2 == 0 {
                let (l, r) = s.split_at(s.len() / 2);
                (int(l), Some(int(r)))
            } else {
                (stone * 2024, None)
            }
        }
    }
}
