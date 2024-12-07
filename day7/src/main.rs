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
    let part1: i64 = input
        .iter()
        .filter(|(x, ns)| {
            let s = (0..(2usize.pow(ns.len() as u32 - 1))).any(|p| {
                let mut i = 0;
                ns.iter()
                    .copied()
                    .reduce(|a, b| {
                        let f = [Add::add, Mul::mul][(p >> i) & 1];
                        i += 1;
                        f(a, b)
                    })
                    .unwrap()
                    == *x
            });
            s
        })
        .map(|(x, _)| x)
        .sum();
    println!("part1: {part1}");

    let part2: i64 = input
        .iter()
        .filter(|(x, ns)| {
            let s = (0..(3usize.pow(ns.len() as u32 - 1))).any(|mut p| {
                ns.iter()
                    .copied()
                    .reduce(|a, b| {
                        let o = p % 3;
                        p /= 3;
                        let f = [Add::add, Mul::mul, |a: i64, b: i64| {
                            (a.to_string() + &b.to_string()).parse().unwrap()
                        }][o];
                        f(a, b)
                    })
                    .unwrap()
                    == *x
            });
            s
        })
        .map(|(x, _)| x)
        .sum();
    println!("part2: {part2}");
}

enum Op {
    Add,
    Mul,
}
