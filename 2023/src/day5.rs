use std::collections::HashMap;

struct Entry {
    src_start: usize,
    dst_start: usize,
    len: usize,
}
impl Entry {
    fn get(&self, index: usize) -> Option<usize> {
        (index >= self.src_start && index < self.src_start + self.len)
            .then(|| self.dst_start + index - self.src_start)
    }
}

pub fn run(input: &str) {
    println!("Day 5:");
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<usize> = seeds
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|seed| seed.parse().unwrap())
        .collect();
    let maps: HashMap<&str, Vec<Entry>> = maps
        .split("\n\n")
        .map(|map| {
            let mut lines = map.lines();
            let map_name = lines.next().unwrap().trim_end_matches(" map:");
            let entries = lines
                .map(|line| {
                    let [dst_start, src_start, len] = line
                        .split(' ')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    Entry {
                        src_start,
                        dst_start,
                        len,
                    }
                })
                .collect();
            (map_name, entries)
        })
        .collect();
    let find_in_map = |map_name: &str, index: usize| {
        for entry in &maps[map_name] {
            if let Some(val) = entry.get(index) {
                return val;
            }
        }
        index
    };
    let categories = [
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ];
    // ---------- part 1 ----------
    let part1 = seeds
        .iter()
        .map(|&seed| {
            categories.array_windows().fold(seed, |item, [from, to]| {
                find_in_map(&format!("{from}-to-{to}"), item)
            })
        })
        .min()
        .unwrap();
    println!("\tPart 1: {}", part1);
    // ---------- part 2 ----------
    // TODO: this solution is naive and entire ranges can be dealt with at once, this will take too long
    let part2 = seeds
        .array_windows()
        .flat_map(|&[start, len]| {
            (start..start+len).map(|seed| {
                categories.array_windows().fold(seed, |item, [from, to]| {
                    find_in_map(&format!("{from}-to-{to}"), item)
                })
            })
        })
        .count();
    println!("\tPart 2: {}", part2);
}

