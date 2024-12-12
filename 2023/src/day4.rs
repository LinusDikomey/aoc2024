pub fn run(input: &str) {
    println!("Day 4:");
    let input: Vec<(u64, Vec<u64>, Vec<u64>)> = input
        .lines()
        .map(|l| {
            let l = l.strip_prefix("Card").unwrap().trim();
            let (game, nums) = l.split_once(": ").unwrap();
            let (a, b) = nums.split_once(" | ").unwrap();
            fn parse_nums(s: &str) -> Vec<u64> {
                s.split(' ')
                    .filter(|s| !s.trim().is_empty())
                    .map(|x| x.trim().parse().unwrap())
                    .collect()
            }
            let a = parse_nums(a);
            let b = parse_nums(b);
            (game.parse().unwrap(), a, b)
        })
        .collect();
    let matches: Vec<_> = input
        .iter()
        .map(|(_, w, my)| my.iter().filter(|x| w.contains(x)).count() as u32)
        .collect();
    eprintln!("{input:?}");
    // ---------- part 1 ----------
    let part1: u64 = matches
        .iter()
        .map(|&n| if n == 0 { 0 } else { 2u64.pow(n - 1) })
        .sum();
    println!("\tPart 1: {}", part1);
    // ---------- part 2 ----------
    let mut total = vec![0u64; matches.len()];
    for (i, matches) in matches.iter().copied().enumerate().rev() {
        total[i] = total[i + 1..i + 1 + matches as usize].iter().sum::<u64>() + 1;
    }
    let part2: u64 = total.iter().sum();
    println!("\tPart 2: {}", part2);
}

