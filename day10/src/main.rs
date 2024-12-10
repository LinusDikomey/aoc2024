use std::collections::HashSet;

fn main() {
    let input: Vec<Vec<u8>> = include_str!("../../input/day10.txt")
        .trim()
        .lines()
        .map(|line| line.as_bytes().iter().map(|&b| b - b'0').collect())
        .collect();
    println!("Hello, world!");
    let part1: usize = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &h)| (x, y, h)))
        .filter(|&(_, _, h)| h == 0)
        .map(|(x, y, h)| {
            let mut s = HashSet::new();
            traverse(&input, x, y, h, &mut |p| {
                s.insert(p);
            });
            s.len()
        })
        .sum();
    println!("Part 1: {part1}");
    let part2: usize = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &h)| (x, y, h)))
        .filter(|&(_, _, h)| h == 0)
        .map(|(x, y, h)| {
            let mut count = 0;
            traverse(&input, x, y, h, &mut |_| {
                count += 1;
            });
            count
        })
        .sum();
    println!("Part 2: {part2}");
}

fn traverse<F: FnMut((usize, usize))>(map: &[Vec<u8>], x: usize, y: usize, h: u8, found: &mut F) {
    if map[y][x] != h {
        return;
    } else if h == 9 {
        found((x, y));
        return;
    }
    if x > 0 {
        traverse(map, x - 1, y, h + 1, found);
    }
    if y > 0 {
        traverse(map, x, y - 1, h + 1, found);
    }
    if x < map[0].len() - 1 {
        traverse(map, x + 1, y, h + 1, found);
    }
    if y < map.len() - 1 {
        traverse(map, x, y + 1, h + 1, found);
    }
}
