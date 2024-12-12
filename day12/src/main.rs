use aoch::*;

fn main() {
    let input = Grid::from_str_bytes(include_str!("../../input/day12.txt").trim());
    let types: HashSet<_> = input.positions().into_iter().map(|p| input[p]).collect();
    let p1: usize = types
        .iter()
        .copied()
        .map(|t| {
            let mut sum = 0;
            let mut positions: HashSet<_> = input.positions().filter(|&p| input[p] == t).collect();
            loop {
                let Some(&p) = positions.iter().next() else {
                    break;
                };
                let mut s = HashSet::new();
                positions.remove(&p);
                s.insert(p);
                let mut q = vec![p];
                while let Some(p) = q.pop() {
                    for n in input.neighbor_positions4(p).filter(|&n| input[n] == t) {
                        if positions.remove(&n) {
                            s.insert(n);
                            q.push(n);
                        }
                    }
                }
                let area = s.len();
                let perim: usize = s
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
                sum += area * perim;
                eprintln!("{} {area} x {perim}", t as char);
            }
            sum
        })
        .sum();
    println!("Part 1: {p1}");
    let p2: usize = types
        .iter()
        .copied()
        .map(|t| {
            let mut sum = 0;
            let mut positions: HashSet<_> = input.positions().filter(|&p| input[p] == t).collect();
            loop {
                let Some(&p) = positions.iter().next() else {
                    break;
                };
                let mut s = HashSet::new();
                positions.remove(&p);
                s.insert(p);
                let mut q = vec![p];
                while let Some(p) = q.pop() {
                    for n in input.neighbor_positions4(p).filter(|&n| input[n] == t) {
                        if positions.remove(&n) {
                            s.insert(n);
                            q.push(n);
                        }
                    }
                }
                let area = s.len();
                #[derive(PartialEq, Eq, Hash, Clone, Copy)]
                enum Side {
                    L,
                    R,
                    T,
                    B,
                }
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
                eprintln!("{} {area} x {sides}", t as char);

                sum += area * sides;
            }
            sum
        })
        .sum();
    println!("Part 2: {p2}");
}
