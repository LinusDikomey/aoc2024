use std.sort

main :: fn {
    input := std.file.read_to_string("input/day1.txt")
    input: List[(u64, u64)] = collect(map(input.split("\n"), fn(line str) -> (u64, u64) {
        s := line.split("   ")
        ret (s.next().unwrap().parse(), s.next().unwrap().parse())
    }))
    first: List[u64] = collect(map(input.iter(), fn(t (u64, u64)) -> u64: t.0))
    second: List[u64] = collect(map(input.iter(), fn(t (u64, u64)) -> u64: t.1))
    sort(&first)
    sort(&second)
    i := 0
    x := 0
    while i < first.len {
        x += abs(first.get(i) as i64 - second.get(i) as i64)
        i += 1
    }
    print("Part 1: ")
    println(x)

    x := 0
    it := first.iter()
    while .Some(i) := Iterator.next(&it) {
        sim := 0
        r := second.iter()
        while .Some(j) := Iterator.next(&r) {
            if i == j: sim += 1
        }
        x += sim * i
    }
    print("Part 2: ")
    println(x)
}
