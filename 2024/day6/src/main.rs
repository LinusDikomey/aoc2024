use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn rotate(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn dir(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/day6.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();
    let mut initial_pos = (0, 0);
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            if *c == b'^' {
                initial_pos = (x as i32, y as i32);
            }
        }
    }

    let width = input[0].len();
    let path = {
        let mut path = HashSet::new();
        let mut pos = initial_pos;
        let mut dir = Dir::Up;
        let mut in_bounds = true;

        while in_bounds {
            path.insert((pos.0, pos.1, dir));
            let o = dir.dir();
            let new = (pos.0 + o.0, pos.1 + o.1);
            in_bounds =
                new.0 >= 0 && new.0 < width as i32 && new.1 >= 0 && new.1 < input.len() as i32;
            if in_bounds && input[new.1 as usize].as_bytes()[new.0 as usize] == b'#' {
                dir = dir.rotate();
            } else {
                pos = new;
            }
        }
        path
    };

    let part1 = path
        .iter()
        .map(|&(a, b, _)| (a, b))
        .collect::<HashSet<_>>()
        .len();
    println!("Part 1: {part1}");

    let part2 = path
        .iter()
        .filter_map(|&(a, b, _)| {
            let obstacle = (a, b);
            let mut pos = initial_pos;
            let mut dir = Dir::Up;
            let mut in_bounds = true;
            let mut path = HashSet::new();
            while in_bounds {
                if path.contains(&(pos.0, pos.1, dir)) {
                    return Some(obstacle);
                }
                path.insert((pos.0, pos.1, dir));
                let o = dir.dir();
                let new = (pos.0 + o.0, pos.1 + o.1);
                in_bounds =
                    new.0 >= 0 && new.0 < width as i32 && new.1 >= 0 && new.1 < input.len() as i32;
                if in_bounds
                    && (input[new.1 as usize].as_bytes()[new.0 as usize] == b'#' || new == obstacle)
                {
                    dir = dir.rotate();
                } else {
                    pos = new;
                }
            }
            None
        })
        .collect::<HashSet<(i32, i32)>>()
        .len();
    println!("Part 2: {part2}");
}
