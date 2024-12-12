use std::array;

pub fn run(s: &str) {
    println!("Day 5:");
    let (stacks, cmds) = s.split_once("\n\n").unwrap();
    let rows: Vec<[char; 9]> = stacks
        .lines()
        .map(|l| array::from_fn(|i| l.chars().nth(1 + i * 4).unwrap()))
        .collect();

    let stacks: [Vec<_>; 9] = array::from_fn(|i| rows
        .iter()
        .rev()
        .map(|row| row[i])
        .take_while(|&c| c != ' ')
        .collect()
    );

    let cmds: Vec<[usize; 3]> = cmds.lines().map(|l| {
        let (i, rest) = l.trim_start_matches("move ").split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        [i.parse().unwrap(), from.parse::<usize>().unwrap()-1, to.parse::<usize>().unwrap()-1]
    }).collect();


    let moves = |mut stacks: [Vec<_>; 9], rev: bool| -> String {
        for &[n, from, to] in &cmds {
            let avail = stacks[from].len();
            let mut taken: Vec<_> = stacks[from].drain(avail-n..).collect();
            if rev { taken.reverse() }
            let c = stacks[to].extend(taken);
        }
        stacks.map(|stack| *stack.last().unwrap()).into_iter().collect()
    };

    println!("\tPart 1: {}", moves(stacks.clone(), true));
    println!("\tPart 2: {}", moves(stacks.clone(), false));
}
