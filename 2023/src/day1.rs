
pub fn run(input: &str) {
    let x: u64 = input.lines().map(|l| {
        let mut c = l.chars().filter(|c| c.is_digit(10));
        ((c.clone().next().unwrap() as u8 - b'0') * 10 + (c.next_back().unwrap() as u8 - b'0')) as u64
    }).sum();

    println!("Part 1: {x}");

    let x: u64 = input.trim().lines().map(|l| {
        letter_digit(l, false) * 10 + letter_digit(l, true)
    }).sum();

    println!("Part 2: {x}");
}

const DIGITS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn letter_digit(mut s: &str, back: bool) -> u64 {
    loop {
        let c = (if back { s.chars().next_back() } else { s.chars().next() }).unwrap();
        match c {
            c @ '0'..='9' => return c as u64 - b'0' as u64,
            _ => {
                for (i, d) in DIGITS.iter().copied().enumerate() {
                    if !back && s.starts_with(d) { return i as u64 }
                    if back && s.ends_with(d) { return i as u64 }
                }
            }
        }
        if back {
            s = &s[..s.len() - 1];
        } else {
            s = &s[1..]
        }
    }
}

