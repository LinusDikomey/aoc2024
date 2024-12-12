use aoch::*;

fn main() {
    let input = Grid::from_str_bytes(include_str!("../../input/day12.txt").trim());
    let regions = transitive_closure(input.positions(), |&a, &b| {
        a.x.abs_diff(b.x) + a.y.abs_diff(b.y) < 2 && input[a] == input[b]
    });
    let p1: usize = regions
        .iter()
        .map(|r| {
            let t = input[*r.first().unwrap()];
            let area = r.len();
            let perim: usize = r
                .iter()
                .map(|&p| {
                    (p.x == 0 || p.x == input.width() as i32 - 1) as usize
                        + (p.y == 0 || p.y == input.height() as i32 - 1) as usize
                        + input
                            .neighbor_positions4(p)
                            .filter(|&n| input[n] != t)
                            .count()
                })
                .sum();
            area * perim
        })
        .sum();
    println!("Part 1: {p1}");
    let p2: usize = regions
        .iter()
        .map(|s| {
            let t = input[s[0]];
            let area = s.len();
            let mut sides: HashMap<_, Vec<_>> = HashMap::new();
            s.iter().for_each(|&p| {
                if p.x == 0 {
                    sides.entry((Side::L, p.x)).or_default().push(p.y);
                } else if p.x == input.width() as i32 - 1 {
                    sides.entry((Side::R, p.x)).or_default().push(p.y);
                }
                if p.y == 0 {
                    sides.entry((Side::T, p.y)).or_default().push(p.x);
                } else if p.y == input.height() as i32 - 1 {
                    sides.entry((Side::B, p.y)).or_default().push(p.x);
                }
                for n in input.neighbor_positions4(p).filter(|&n| input[n] != t) {
                    let (v, o) = match n {
                        n if n.x < p.x => ((Side::L, p.x), p.y),
                        n if n.y < p.y => ((Side::T, p.y), p.x),
                        n if n.x > p.x => ((Side::R, p.x), p.y),
                        n if n.y > p.y => ((Side::B, p.y), p.x),
                        _ => unreachable!(),
                    };
                    sides.entry(v).or_default().push(o);
                }
            });

            let sides: usize = sides
                .into_values()
                .map(|mut s| {
                    s.sort();
                    s.windows(2).filter(|a| a[1] > a[0] + 1).count() + 1
                })
                .sum();
            area * sides
        })
        .sum();
    println!("Part 2: {p2}");
}
