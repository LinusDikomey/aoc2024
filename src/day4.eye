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
                if check_word_in_dir(&lines, x, y, dir, "XMAS"): part1 += 1
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
                if check_word_in_dir(&lines, x, y, dir, "MAS", offset: -1) and (
                    check_word_in_dir(&lines, x, y, (-dir.0, dir.1), "MAS", offset: -1) 
                    or check_word_in_dir(&lines, x, y, (dir.0, -dir.1), "MAS", offset: -1) 
                ) {
                    part2 += 1
                    d = 5
                } else d += 1
            }
            x += 1
        }
        y += 1
    }
    println(part2)
}

check_word_in_dir :: fn(lines *List[str], x u64, y u64, dir (i32, i32), word str, offset i32 = 0) -> bool {
    i := 0
    while i < word.len {
        xo := x as i32 + dir.0 * (i as i32 + offset)
        yo := y as i32 + dir.1 * (i as i32 + offset)
        if xo >= 0 and xo < lines.get(0).len as i32 and yo >= 0 and yo < lines.len as i32 {
            if word.byte(i) == lines.get(yo as u64).byte(xo as u64): i += 1
            else ret false
        } else ret false
    }
    ret true
}
