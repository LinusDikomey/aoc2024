use std.max

main :: fn {
    input := std.file.read_to_string("input/day5.txt")
    parts := input.split("\n\n")
    a := parts.next().unwrap()
    b := parts.next().unwrap()
    a: List[(u64, u64)] = collect(map(a.split("\n"), fn(s str) -> (i32, i32) {
        i := s.split("|")  
        ret (i.next().unwrap().parse(), i.next().unwrap().parse())
    }))
    m := 0
    it := a.iter()
    while .Some((x, y)) := Iterator.next(&it) {
        m = max(x, m)
        m = max(y, m)
    }
    order: List[List[u64]] = List.fill(List.new(), m + 1)
    it := a.iter()
    while .Some((x, y)) := Iterator.next(&it) {
        order.get_ptr(x).push(y)
    }
    b: List[List[u64]] = collect(map(b.split("\n"), fn(s str) -> List[u64] {
        ret collect((map(s.split(","), fn(s str) -> u64: s.parse())))
    }))

    print("Part 1: ")
    part1 := 0
    it := b.iter()
    while .Some(rule) := Iterator.next(&it) {
        if ordered(&rule, &order) {
            part1 += rule.get(rule.len / 2)
        }
    }
    println(part1)

    print("Part 2: ")
    part2 := 0
    it := b.iter()
    while .Some(rule) := Iterator.next(&it) {
        if !ordered(&rule, &order) {
            j := 0
            while j < rule.len - 1 {
                k := 0
                while k < rule.len - 1 - j {
                    if !item_ordered(&order, rule.get(k), rule.get(k+1)) {
                        tmp := rule.get(k)
                        rule.put(k, rule.get(k+1))
                        rule.put(k+1, tmp)
                    }                    
                    k += 1
                }
                j += 1
            }
            part2 += rule.get(rule.len / 2)
        }
    }
    println(part2)
}

ordered :: fn(rule *List[u64], order *List[List[u64]]) -> bool {
    i := 0
    while i < rule.len {

        j := i + 1
        after := order.get_ptr(rule.get(i))
        while j < rule.len {
            if !item_ordered(order, rule.get(i), rule.get(j)) {
                ret false
            }
            j += 1
        }
        i += 1
    }
    ret true
}

item_ordered :: fn(order* List[List[u64]], a u64, b u64) -> bool {
    it := order.get_ptr(a).iter()
    found := false
    while .Some(i) := Iterator.next(&it) {
        if b == i: ret true
    }
    ret false
}
