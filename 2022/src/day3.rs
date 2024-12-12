use crate::*;

fn prio(i: char) -> i32 {
    (match i {
        'a'..='z' => i as u8 - b'a' + 1,
        'A'..='Z' => i as u8 - b'A' + 27,
        _ => panic!()
    }) as i32
}


pub fn run(s: &str) {
    println!("Day 3:");
    let a: i32 = s.lines().map(|line| {
        let a = &line[0..line.len()/2];
        let b = &line[line.len()/2..];

        for item in a.chars() {
            if b.contains(item) {
                return prio(item)
            }
        }
        panic!()
    }).sum();
    println!("\tPart 1: {a}");

    let b: i32 = s.lines().array_chunks().map(|[a, b, c]| {
        for i in a.chars() {
            if b.contains(i) && c.contains(i) {
                return prio(i);
            }
        }
        panic!()
    }).sum();
    println!("\tPart 2: {b}");
}

