pub fn run(input: &str) {
    let mut start = None;
    let mut end = None;

    let map: Vec<Vec<_>> = input.trim().lines()
        .enumerate()
        .map(|(y, l)| l
            .chars()
            .enumerate()
            .map(|(x, c)| match c {
                'S' => {
                    start = Some((x, y));
                    0
                }
                'E' => {
                    end = Some((x, y));
                    'z' as u8 - b'a'
                }
                'a'..='z' => c as u8 - b'a',
                _ => panic!()
            })
            .collect()
        )
        .collect();

    println!("Day 12:");
    // ---------- part 1 ----------
    
    println!("\tPart 1: {}", find(&map, start.unwrap(), end.unwrap(), &mut vec![[false; 200]; 200]));
    // ---------- part 2 ----------
    println!("\tPart 2: {}", 0);
}   

fn can_move(map: &[Vec<u8>], cur: (usize, usize), dir: (i32, i32)) -> bool {
    map[(cur.1 as i32 + dir.1) as usize][(cur.0 as i32 + dir.0) as usize].saturating_sub(map[cur.1][cur.0]) < 2
}

fn find(map: &[Vec<u8>], cur: (usize, usize), end: (usize, usize), visited: &mut Vec<[bool; 200]>) -> usize {
    if visited[cur.0][cur.1] { return usize::MAX / 8 }
    visited[cur.0][cur.1] = true;
    if cur == end { return 0 }

    let v1 = visited.clone();
    let v2 = visited.clone();
    let v3 = visited.clone();
    let v4 = visited.clone();

    let (new_v, steps) = (cur.0 > 0 && can_move(map, cur, (-1, 0))).then(|| { let mut v = v1; let f = find(map, (cur.0 - 1, cur.1), end, &mut v); (v, f) }).iter()
        .chain((cur.1 > 0 && can_move(map, cur, (0, -1))).then(|| { let mut v = v2; let f = find(map, (cur.0, cur.1 - 1), end, &mut v); (v, f) }).iter())
        .chain((cur.0 + 1 < map[0].len() && can_move(map, cur, (1, 0))).then(|| { let mut v = v3; let f = find(map, (cur.0 + 1, cur.1), end, &mut v); (v, f) }).iter())
        .chain((cur.1 + 1 < map.len()    && can_move(map, cur, (0, 1))).then(|| { let mut v = v4; let f = find(map, (cur.0, cur.1 + 1), end, &mut v); (v, f) }).iter())
        .min_by_key(|(_, n)| n).cloned().unwrap_or((visited.clone(), usize::MAX / 8));
    *visited = new_v;
    if steps < 1000 {
        eprintln!("{}", steps);
    }
    steps + 1
}