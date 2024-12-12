use std::iter::repeat;

pub fn run(input: &str) {
    println!("Day 8:");
    let trees: Vec<Vec<_>> = input.trim().lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect();
    let h = trees.len();
    let w = trees[0].len();
    // ---------- part 1 ----------
    let mut visible = w * 2 + h * 2 - 4;
    for y in 1..h-1 {
        for x in 1..w-1 {
            let t = trees[y][x];
            let v = trees[y][0..x].iter().all(|&t2| t2 < t)
                || trees[y][x+1..w].iter().all(|&t2| t2 < t)
                || trees[0..y].iter().all(|row| row[x] < t)
                || trees[y+1..h].iter().all(|row| row[x] < t);
            if v { visible += 1 }
        }
    }
    println!("\tPart 1: {}", visible);
    // ---------- part 2 ----------
    fn count(t: u8, trees: &[Vec<u8>], it: impl IntoIterator<Item = (usize, usize)>) -> usize {
        let mut it = it.into_iter();
        (&mut it).take_while(|&(x, y)| trees[y][x] < t).count() + it.next().is_some() as usize
    }
    let best = (1..h-1).map(|y| {
        (1..w-1).map(|x| {
            let t = trees[y][x];
            count(t, &trees, (0..x).rev().zip(repeat(y)))
            * count(t, &trees, (x+1..w).zip(repeat(y)))
            * count(t, &trees, repeat(x).zip(0..y))
            * count(t, &trees, repeat(x).zip(y+1..h))
        }).max().unwrap()
    }).max().unwrap();
    println!("\tPart 2: {}", best);
}
