use std::collections::{HashMap, HashSet};

use itertools::iterate;
use vecm::Vec2i;

fn main() {
    let input: Vec<&[u8]> = include_str!("../../input/day8.txt")
        .lines()
        .map(|s| s.as_bytes())
        .collect();
    let antennas_iter = input.iter().zip(0..).flat_map(|(l, y)| {
        l.iter()
            .zip(0..)
            .filter_map(move |(&c, x)| is_antenna(c).then_some((c, x, y)))
    });
    let mut antennas: HashMap<u8, Vec<Vec2i>> = HashMap::new();
    for (a, x, y) in antennas_iter {
        antennas.entry(a).or_default().push(Vec2i::new(x, y));
    }
    let in_bounds =
        |v: Vec2i| (0..input[0].len() as _).contains(&v.x) && (0..input.len() as _).contains(&v.y);

    let part1 = antinode_count(&antennas, in_bounds, move |a1, a2| {
        [a1 - (a2 - a1), a2 + (a2 - a1)]
    });
    println!("part 1: {part1}");
    let part2 = antinode_count(&antennas, in_bounds, move |a1, a2| {
        let step = a2 - a1;
        let mut p = a1;
        while in_bounds(p - step) {
            p -= step;
        }
        iterate(p, move |&p| p + step).take_while(move |&p| in_bounds(p))
    });
    println!("part 2: {part2}");
}

fn is_antenna(x: u8) -> bool {
    x.is_ascii_alphanumeric()
}

fn antinode_count<F: Copy + Fn(Vec2i, Vec2i) -> O, O: IntoIterator<Item = Vec2i>>(
    antennas: &HashMap<u8, Vec<Vec2i>>,
    in_bounds: impl Copy + Fn(Vec2i) -> bool,
    interferences: F,
) -> usize {
    antennas
        .values()
        .flat_map(|antennas| {
            antennas.iter().flat_map(|&a1| {
                antennas
                    .iter()
                    .copied()
                    .filter(move |&a2| a1 != a2)
                    .flat_map(move |a2| interferences(a1, a2))
            })
        })
        .collect::<HashSet<Vec2i>>()
        .into_iter()
        .filter(|&l| in_bounds(l))
        .count()
}
