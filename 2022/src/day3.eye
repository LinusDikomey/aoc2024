use std.ptr_add
use std.list.List
use std.c.printf
use std.panic

# code not finished

prio :: fn(c u8) -> i32 {
    match c {
        65..90: c - 65 + 1,
        97..122: c - 97 + 27,
        _: panic("invalid character")
    }
}

contains :: fn(s *i8, len u64, c u8) -> bool {
    i := 0
    while i < len {
        if u8(ptr_add(s, i)^) == c: ret true
        i += 1
    }
    ret false
}

rucksack :: fn(line *i8) -> i32 {
    len := std.string.len(line)

    j := 0
    while j < len/2 {
        k := len/2
        c := u8(ptr_add(line, j)^)
        if contains(ptr_add(line, len/2), len - len/2, c) {
            ret prio(c)
        }
        j += 1
    }
    printf("line: %s\n", line)
    panic("no matching item found in rucksack")
}

main :: fn {
    input := std.file.read_to_string("src/day3.txt")
    lines := std.string.lines(input)

    printf("Part 1: %d\n", a(&lines))
    printf("Part 2: %d\n", b(&lines))
}

a :: fn(lines *List[*i8]) -> i32 {
    total := 0

    i := 0
    while i < lines.len {
        line := lines.get(i)
        if std.string.len(line) > 0 {
            total += rucksack(line)
            i += 1
        }
    }
    ret total
}

b :: fn(lines *List[*i8]) -> i32 {
    total := 0

    i := 0
    while i < lines.len - 2 {
        a := lines.get(i)
        b := lines.get(i+1)
        c := lines.get(i+2)

        i += 3
    }
}