
main :: fn {
    input := std.file.read_to_string("input/day2.txt")
    reports: List[List[i64]] = collect(map(input.split("\n"), fn(s str) -> List[i64]: collect(map(s.split(" "), fn(s str) -> u64: s.parse()))))
    
    print("Part 1: ")
    part1 := 0
    it := reports.iter()
    while .Some(report) := Iterator.next(&it) {
        if safe(report): part1 += 1
    }
    println(part1)

    print("Part 2: ")
    part2 := 0
    it := reports.iter()
    while .Some(report) := Iterator.next(&it) {
        if safe(report): part2 += 1
        else {
            i := 0
            while i < report.len {
                if safe(without(report, i)) {
                    part2 += 1
                    i = report.len # don't have break yet
                } else {
                    i += 1
                }
            }
        }
    }
    println(part2)
}

without :: fn[T](list List[T], index u64) -> List[T] {
    i := 0
    out := List.new()
    while i < list.len {
        if i != index: out.push(list.get(i))
        i += 1
    }
    ret out
}

safe :: fn(report List[i64]) -> bool {
    safe := true
    m := if report.get(1) > report.get(0): .Inc else .Dec
    it := report.iter()
    prev := Iterator.next(&it).unwrap()
    while .Some(n) := Iterator.next(&it) {
        ok := match m {
            .Inc: n > prev and n <= prev + 3,
            .Dec: n < prev and n >= prev - 3
        }
        if !ok: safe = false
        prev = n
    }
    ret safe
}
