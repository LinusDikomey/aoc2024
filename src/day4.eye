use std.int.Int

main :: fn {
    input := std.file.read_to_string("input/day4.txt")
    lines: List[str] = collect(input.split("\n"))
    print("Part 1: ")
    y := 0
    directions: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
    m := "XMAS"
    part1: u64 = 0
    while y < lines.len {
        x := 0
        while x < lines.get(0).len {
            d := 0
            while d < 8 {
                dir := directions[d]
                i := 0
                while i < m.len {
                    xo := x as i32 + dir.0 * i as i32
                    yo := y as i32 + dir.1 * i as i32
                    if xo >= 0 and xo < lines.get(0).len as i32 and yo >= 0 and yo < lines.len as i32 {
                        if m.byte(i) == lines.get(yo as u64).byte(xo as u64): i += 1
                        else i = m.len + 1
                    } else i = m.len + 1
                }
                if i == m.len {
                    part1 += 1
                }
                d += 1
            }
            i := 0
            
            x += 1
        }
        y += 1
    }
    println(part1)

    print("Part 2: ")
    directions: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)]
    m := "MAS"
    y := 0
    part2 := 0
    while y < lines.len {
        x := 0
        while x < lines.get(0).len {
            d := 0
            while d < 4 {
                dir := directions[d]
                i := 0
                while i < m.len {
                    xo := x as i32 + dir.0 * (i as i32 - 1)
                    yo := y as i32 + dir.1 * (i as i32 - 1)
                    if xo >= 0 and xo < lines.get(0).len as i32 and yo >= 0 and yo < lines.len as i32 {
                        if m.byte(i) == lines.get(yo as u64).byte(xo as u64): i += 1
                        else i = m.len + 1
                    } else i = m.len + 1
                }
                if i == m.len {
                    d = 5
                    i := 0
                    {
                        dir: (i32, i32) = (dir.0, if dir.1 == 1: -1 else 1)
                        while i < m.len {
                            xo := x as i32 + dir.0 * (i as i32 - 1)
                            yo := y as i32 + dir.1 * (i as i32 - 1)
                            if xo >= 0 and xo < lines.get(0).len as i32 and yo >= 0 and yo < lines.len as i32 {
                                if m.byte(i) == lines.get(yo as u64).byte(xo as u64): i += 1
                                else i = m.len + 1
                            } else i = m.len + 1
                        }
                    }
                    if i == m.len {
                        part2 += 1
                    } else {
                        dir: (i32, i32) = (if dir.0 == 1: -1 else 1, dir.1)
                        i := 0
                        while i < m.len {
                            xo := x as i32 + dir.0 * (i as i32 - 1)
                            yo := y as i32 + dir.1 * (i as i32 - 1)
                            if xo >= 0 and xo < lines.get(0).len as i32 and yo >= 0 and yo < lines.len as i32 {
                                if m.byte(i) == lines.get(yo as u64).byte(xo as u64): i += 1
                                else i = m.len + 1
                            } else i = m.len + 1
                        }
                        if i == m.len: part2 += 1
                    }
                }
                d += 1
            }
            i := 0
            
            x += 1
        }
        y += 1
    }
    println(part2)
}
