use std::ops::{Add, Mul};

fn main() {
    let input: Vec<(i64, Vec<i64>)> = include_str!("../../input/day7.txt")
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            (
                a.parse().unwrap(),
                b.split_whitespace().map(|s| s.parse().unwrap()).collect(),
            )
        })
        .collect();
    println!("part1: {}", solve(&input, &[Add::add, Mul::mul]));
    println!("part2: {}", solve(&input, &[Add::add, Mul::mul, concat]));
}

fn concat(x: i64, y: i64) -> i64 {
    format!("{x}{y}").parse().unwrap()
}

fn solve(input: &[(i64, Vec<i64>)], fns: &[fn(i64, i64) -> i64]) -> i64 {
    input
        .iter()
        .filter(|(x, ns)| {
            let s = (0..(fns.len().pow(ns.len() as u32 - 1))).any(|mut p| {
                ns.iter()
                    .copied()
                    .reduce(|a, b| {
                        let o = p % fns.len();
                        p /= fns.len();
                        let f = fns[o];
                        f(a, b)
                    })
                    .unwrap()
                    == *x
            });
            s
        })
        .map(|(x, _)| x)
        .sum()
}
