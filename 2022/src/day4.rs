use std::ops::RangeInclusive;

use crate::*;

fn r(range: &str) -> RangeInclusive<i32> {
    let (l, r) = range.split_once('-').unwrap();

    l.parse().unwrap() ..= r.parse().unwrap()
}

pub fn run(s: &str) {
    println!("Day 4:");
    let i: Vec<_> = s.trim().lines().map(|l| {
        let (a, b) = l.split_once(',').unwrap();
        (r(a), r(b))
    }).collect();

    let a = i.iter().filter(|(a, b)|
        (a.start() >= b.start() && a.end() <= b.end())
        || (b.start() >= a.start() && b.end() <= a.end())
    ).count();
    println!("\tPart 1: {a}");

    let b = i.iter().filter(|(a, b)|
        if a.start() < b.start() {
            a.end() >= b.start()
        } else {
            b.end() >= a.start()
        }
    ).count();
    println!("\tPart 2: {b}");
}

