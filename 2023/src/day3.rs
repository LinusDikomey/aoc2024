use std::iter::repeat;

pub fn run(input: &str) {
    println!("Day 3:");
    // ---------- part 1 ----------
    let lines: Vec<&[u8]> = input
        .lines()
        .map(str::as_bytes)
        .collect();

    let part1: u64 = lines.iter().enumerate().map(|(i, line)| {
        let adj_valid = |x| {
            if i != 0 {
                if !matches!(lines[i-1][x], b'0'..=b'9' | b'.') { return true }
            }
            if let Some(after) = lines.get(i+1) {
                if !matches!(after[x], b'0'..=b'9' | b'.') {
                    return true;
                }
            }
            false
        };
        let res = line.iter().enumerate().fold((0, false, 0), |(n, is_valid, sum), (x, c)| {
            match c {
                b'0'..=b'9' => (n * 10 + (c - b'0') as u64, is_valid || adj_valid(x), sum),
                b'.' => (0, adj_valid(x), sum + if is_valid || adj_valid(x) { n } else { 0 }),
                _ => (0, true, sum + n),

            }
        });
        match res {
            (n, true, sum) => sum + n,
            (_, false, sum) => sum,
        }
    }).sum();
    
    println!("\tPart 1: {}", part1);
    // ---------- part 2 ----------
    let mut numbers = vec![];

    let line_numbers: Vec<Vec<Option<usize>>> = lines
        .iter()
        .map(|line| {
            let mut cur = None;
            let line = line.iter().copied().map(|c| match (cur.take(), c) {
                (prev, c @ b'0'..=b'9') => {
                    cur = Some(prev.unwrap_or(0) * 10 + ((c - b'0') as u64));
                    Some(numbers.len())
                }
                (Some(n), _) => {
                    numbers.push(n);
                    None
                }
                (None, _) => None,
            }).collect();
            if let Some(n) = cur {
                numbers.push(n);
            }
            line
        })
        .collect();
    let part2: u64 = lines
        .iter()
        .enumerate()
        .map(|(y, line)| repeat(y).zip(line.iter().enumerate()))
        .flatten()
        .filter(|(_, (_, &c))| c == b'*')
        .filter_map(|(y, (x, _))| {
            let mut nums: Vec<_> = (y.saturating_sub(1)..(y+2).min(lines.len()))
                .flat_map(|y| (x.saturating_sub(1)..(x+2).min(lines[0].len())).zip(repeat(y)))
                .filter_map(|(x, y)| line_numbers[y][x])
                .collect();
            nums.dedup();
            nums
                .try_into()
                .ok()
                .map(|[a, b]: [usize; 2]| numbers[a] as u64 * numbers[b] as u64)
        })
        .sum();
    println!("\tPart 2: {}", part2);
}
