use std.c.printf as print
use std.string
use std.ptr_add

main :: fn {
    input := std.file.read_to_string("src/input/day1.txt")
    lines := string.lines(input.ptr)
    

    print("Part a: %d\n".ptr, a(lines))
    print("Part b: %d\n".ptr, b(lines))
}

a :: fn(lines std.list.List[*i8]) -> i32 {
    current := 0
    max := 0
    i := 0
    while i < lines.len {
        line := ptr_add(lines.buf, i)^
        if string.len(line) == 0 {
            # print("empty line\n")
            if current > max: max = current
            current = 0
        } else {
            val := string.parse_int(line)
            # print("Line %d: %s -> %d\n", i, line, val)
            current += val
        }
        i += 1
    }
    if current > max: max = current
    ret max
}

b :: fn(lines std.list.List[*i8]) -> i32 {
    # largest is at last index
    top := [0, 0, 0]

    insert :: fn(top *[i32; 3], val i32) {
        if top^[0] > val: ret
        i := 0
        while i < 2 and top^[i+1] < val {
            i += 1
        }
        prev := top^[i]
        top^[i] = val
        i -= 1
        while i >= 0 {
            tmp := top^[i]
            top^[i] = prev
            prev = tmp
            i -= 1
        }
    }

    current := 0
    max := 0
    i := 0
    while i < lines.len {
        line := ptr_add(lines.buf, i)^
        if string.len(line) == 0 {
            insert(&top, current)
            current = 0
        } else {
            val := string.parse_int(line)
            current += val
        }
        i += 1
    }
    insert(&top, current)

    ret top[0] + top[1] + top[2]
}
