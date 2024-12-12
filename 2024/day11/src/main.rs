use aoch::ints;
use std::{collections::HashMap, time::Instant};

fn main() {
    let input = ints(include_str!("../../input/day11.txt"));
    let start = Instant::now();
    let part1 = solve(&input, 25);
    let part2 = solve(&input, 75);
    dbg!(start.elapsed());
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn solve(input: &[i64], steps: usize) -> u64 {
    let mut m: HashMap<i64, u64> = HashMap::new();
    let mut m2 = HashMap::new();
    for &item in input {
        *m.entry(item).or_default() += 1;
    }
    for _ in 0..steps {
        for (s, count) in m.drain() {
            let (a, b) = stone(s);
            *m2.entry(a).or_default() += count;
            if let Some(b) = b {
                *m2.entry(b).or_default() += count;
            }
        }
        std::mem::swap(&mut m, &mut m2);
        m2.clear();
    }
    m.values().copied().sum::<u64>()
}

fn stone(stone: i64) -> (i64, Option<i64>) {
    match stone {
        0 => (1, None),
        n => {
            let digits = n.ilog10() + 1;
            if digits % 2 == 0 {
                let divider = 10i64.pow(digits / 2);
                (n / divider, Some(n % divider))
            } else {
                (stone * 2024, None)
            }
        }
    }
}
