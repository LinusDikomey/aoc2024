fn main() {
    let print_disk = std::env::args().any(|arg| arg == "--print");

    let input = include_str!("../../input/day9.txt").trim();
    let input: Vec<u8> = input.as_bytes().iter().map(|b| b - b'0').collect();
    let mut l = 1;
    let mut free = input[l];
    let mut r = input.len() - 1;
    let mut to_move = input[r];
    let mut i = input[0] as usize;
    let mut checksum = 0;
    if print_disk {
        print!("{}", "0".repeat(input[0] as usize));
    }
    let mut append_blocks = |file_id: u64, count: usize| {
        checksum += (i..i + count).map(|i| file_id * i as u64).sum::<u64>();
        if print_disk {
            print!("{}", file_id.to_string().repeat(count));
        }
        i += count;
    };
    while l < r {
        if free == 0 {
            // calculate checksum for the occupied file id
            let file_id = ((l + 1) / 2) as u64;
            let count = if l + 1 == r { to_move } else { input[l + 1] };
            append_blocks(file_id, count as usize);
            l += 2;
            free = input[l];
            continue;
        }
        if to_move == 0 {
            r -= 2;
            to_move = input[r];
            continue;
        }
        let move_count = free.min(to_move);
        to_move -= move_count;
        free -= move_count;
        let file_id = (r / 2) as u64;
        append_blocks(file_id, move_count as usize);
    }
    if print_disk {
        println!();
    }
    println!("Part 1: {checksum}");

    #[derive(Debug, Clone, Copy)]
    struct Slot {
        id: usize,
        size: usize,
        free: usize,
    }
    let mut ids: Vec<_> = (0..=input.len() / 2)
        .map(|i| Slot {
            id: i,
            size: input[2 * i] as usize,
            free: input.get(2 * i + 1).copied().unwrap_or_default() as usize,
        })
        .collect();
    let mut i = ids.len() - 1;
    loop {
        if i == 0 {
            break;
        }
        let file = ids[i];
        for slot in 0..i {
            if print_disk {
                ids.iter().for_each(|&s| {
                    eprint!("{}", s.id.to_string().repeat(s.size));
                    eprint!("{}", ".".repeat(s.free));
                });
                eprintln!("\n{ids:?}");
                eprintln!("checking {i}->{slot} {file:?}->{:?}", ids[slot]);
            }
            if ids[slot].free >= file.size {
                let mut free = ids[slot].free - file.size;
                ids[slot].free = 0;
                if slot + 1 != i {
                    ids[i - 1].free += file.size + file.free;
                } else {
                    free += file.size + file.free;
                }
                ids.remove(i);
                ids.insert(
                    slot + 1,
                    Slot {
                        id: file.id,
                        size: file.size,
                        free,
                    },
                );
                if print_disk {
                    eprintln!("move {i}->{slot}");
                }
                i += 1;
                break;
            } /* else if slot + 1 == i && ids[slot].free > 0 {
                  eprintln!("overlap");
                  // overlapping move into the slot immediately to the left
                  ids[i].free += ids[slot].free;
                  ids[slot].free = 0;
              }*/
        }
        i -= 1;
    }
    let mut i = 0;
    let checksum: usize = ids
        .into_iter()
        .flat_map(|s| {
            if print_disk {
                print!("{}", s.id.to_string().repeat(s.size));
                print!("{}", ".".repeat(s.free));
            }
            let sums = (i..i + s.size).map(move |i| s.id * i);
            i += s.size + s.free;
            sums
        })
        .sum();
    if print_disk {
        println!();
    }
    println!("Part 2: {checksum}");
}
