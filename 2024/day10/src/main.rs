use std::collections::HashSet;

use aoch::{Grid, Vec2i};

fn main() {
    let g = Grid::from_str_bytes(include_str!("../../input/day10.txt").trim()).map(|b| b - b'0');
    println!("Hello, world!");
    let part1: usize = g
        .positions()
        .filter(|&p| g[p] == 0)
        .map(|p| {
            let mut s = HashSet::new();
            traverse(&g, p, 0, &mut |p| {
                s.insert(p);
            });
            s.len()
        })
        .sum();
    println!("Part 1: {part1}");
    let part2: usize = g
        .positions()
        .map(|p| {
            let mut count = 0;
            traverse(&g, p, 0, &mut |_| {
                count += 1;
            });
            count
        })
        .sum();
    println!("Part 2: {part2}");
}

fn traverse<F: FnMut(Vec2i)>(g: &Grid<u8>, p: Vec2i, h: u8, found: &mut F) {
    if g[p] != h {
        return;
    } else if h == 9 {
        found(p);
        return;
    }
    for p in g.neighbor_positions4(p) {
        traverse(g, p, h + 1, found);
    }
}
