use std.int.Int

Inst :: enum {
    Do
    Dont
    Mul(u64, u64)
}

main :: fn {
    input := std.file.read_to_string("input/day3.txt")
    insts: List[Inst] = List.new()
    i := 0
    while i < input.len - 7 {
        if input.slice(i, i+4) == "do()" {
            insts.push(.Do)
            i += 4
        } else if input.slice(i, i+7) == "don't()" {
            insts.push(.Dont)
            i += 7
        } else if input.slice(i,i+4) == "mul("  {
            i += 4
            l := 0
            ok := false
            while .Some(d) := digit(input.byte(i)) {
                ok = true
                l *= 10
                l += d
                i += 1
            }
            if ok and input.slice(i, i+1) == "," {
                i += 1
                ok = false
                r := 0
                
                while .Some(d) := digit(input.byte(i)) {
                    ok = true
                    r *= 10
                    r += d
                    i += 1
                }
                if ok and input.slice(i, i+1) == ")" {
                    insts.push(.Mul(l, r))
                    i += 1
                }
            }
        } else i += 1
    }
    
    print("Part 1: ")
    part1: u64 = sum(map(insts.iter(), fn(inst Inst) -> u64 {
        ret if .Mul(x, y) := inst: x * y
        else 1
    }))
    println(part1)

    print("Part 2: ")
    part2 := 0
    it := insts.iter()
    do := true
    while .Some(inst) := Iterator.next(&it) {
        match inst {
            .Do { do = true }
            .Dont { do = false }
            .Mul(a, b): if do { part2 += a * b }
        }
    }
    println(part2)
}

digit :: fn(x u8) -> Option[u64]: if 48 < x and x <= 57: .Some(x as u64 -48) else .None
