#[derive(Default, Clone, Copy)]
struct RGB {
    red: u64,
    green: u64,
    blue: u64,
}
impl RGB {
    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }

    fn accum(self, b: RGB) -> Self {
        Self {
            red: self.red.max(b.red),
            green: self.green.max(b.green),
            blue: self.blue.max(b.blue),
        }
    }
}

pub fn run(input: &str) {
    println!("Day 2:");
    let input: Vec<(u64, Vec<RGB>)> = input
        .lines()
        .map(|l| {
            let (g, cubes) = l.strip_prefix("Game ").unwrap().split_once(": ").unwrap();
            let sets = cubes
                .split("; ")
                .map(|colors| {
                    let mut rgb = RGB::default();
                    for color in colors.split(", ") {
                        let (n, color) = color.split_once(" ").unwrap();
                        let n: u64 = n.parse().unwrap();
                        match color {
                            "red" => rgb.red += n,
                            "green" => rgb.green += n,
                            "blue" => rgb.blue += n,
                            _ => unreachable!(),
                        }
                    }
                    rgb
                })
                .collect();
            (g.parse().unwrap(), sets)
        })
        .collect();
    // ---------- part 1 ----------
    let part1: u64 = input
        .iter()
        .filter_map(|(game, sets)| {
            sets.iter()
                .all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14)
                .then_some(*game)
        })
        .sum();
    println!("\tPart 1: {}", part1);
    // ---------- part 2 ----------
    let part2: u64 = input
        .iter()
        .map(|(_, sets)| {
            sets.iter()
                .copied()
                .fold(RGB::default(), RGB::accum)
                .power()
        })
        .sum();
    println!("\tPart 2: {}", part2);
}
