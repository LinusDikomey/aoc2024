pub fn run(input: &str) {
    println!("Day 6:");
    let chars: Vec<_> = input.chars().collect();
    // ---------- part 1 ----------
    println!("\tPart 1: {}", find::<4>(&chars));
    // ---------- part 2 ----------
    println!("\tPart 2: {}", find::<14>(&chars));
}

fn find<const N: usize>(chars: &[char]) -> usize {
    chars
        .array_windows::<N>()
        .enumerate()
        .find(|(_, elems)|
            elems
                .iter()
                .enumerate()
                .all(|(i, c)| !elems[0..i].contains(c))
        ).unwrap().0 + N
}
