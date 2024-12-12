use std::{str::FromStr, mem};
use lstd::*;

pub fn run(input: &str) {
    println!("Day 11:");
    let monkeys: Vec<_> = input.lines()
        .array_chunks()
        .map(Monkey::parse)
        .collect();
    let div = monkeys.iter().map(|m| m.test).product();
    // ---------- part 1 ----------
    println!("\tPart 1: {}", calc(monkeys.clone(), 20, 3, div));
    // ---------- part 2 ----------
    println!("\tPart 2: {}", calc(monkeys, 10_000, 1, div));
}

type Item = u64;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    op: Op,
    test: Item,
    on_true: usize,
    on_false: usize,
    inspect_count: u64,
}
impl Monkey {
    fn parse(s: [&str; 7]) -> Monkey {
        assert!(s[0].starts_with("Monkey "), "Expected 'Monkey' but found {}", s[0]);
        let items = s[1]
            .trim_start_matches("  Starting items: ")
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        let op = s[2]
            .trim_start_matches("  Operation: new = old ");
        let op = match op.chars().next().unwrap() {
            _ if op == "* old" => Op::Square,
            '+' => Op::Add(op[1..].trim().parse().unwrap()),
            '*' => Op::Mul(op[1..].trim().parse().unwrap()),
            _ => panic!("unknown op")
        };
        let test = s[3]
            .trim()
            .trim_start_matches("Test: divisible by ")
            .parse().unwrap();

        let on_true = s[4]
            .trim()
            .trim_start_matches("If true: throw to monkey ")
            .parse().unwrap();

        let on_false = s[5]
            .trim()
            .trim_start_matches("If false: throw to monkey ")
            .parse().unwrap();

        Monkey {
            items,
            op,
            test,
            on_true,
            on_false,
            inspect_count: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(Item),
    Mul(Item),
    Square,
}
impl Op {
    fn apply(self, x: Item) -> Item {
        match self {
            Op::Add(n) => x + n,
            Op::Mul(n) => x * n,
            Op::Square => x * x,
        }
    }
}

fn calc(mut monkeys: Vec<Monkey>, rounds: usize, worry_divide: Item, div: Item) -> u64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            m.inspect_count += m.items.len() as u64;
            let (t, f) = m.items
                .drain(..)
                .map(|i| (m.op.apply(i) % div) / worry_divide)
                .partition::<Vec<_>, _>(|item| item % m.test == 0);
            let on_true = m.on_true;
            let on_false = m.on_false;
            monkeys[on_true].items.extend(t);
            monkeys[on_false].items.extend(f);
        }
    }
    monkeys 
        .iter()
        .map(|m| m.inspect_count)
        .max_n::<2>()
        .into_iter()
        .map(|n| n.unwrap())
        .product()
}
