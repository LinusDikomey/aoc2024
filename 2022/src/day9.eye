use std.list.List
use std.panic
use std.print
use std.println

Dir :: enum { L R U D }

main :: fn {
    println("Day 9:")
    input := std.file.read_to_string("src/input/day9.txt")
    lines := input.trim().lines()
    cmds := List.new()
    i := 0
    while i < lines.len {
        l := lines.get(i)
        dir := match l.slice(0, 1) {
            "L": .L,
            "R": .R,
            "U": .U,
            "D": .D,
            _: panic("Invalid input")
        }
        amount := l.slice(2, l.len).parse() as i32
        cmds.push((dir, amount))
        i += 1
    }
    std.c.printf("\tPart 1: %d\n".ptr, a(cmds))
    std.c.printf("\tPart 2: %d\n".ptr, b(cmds))
}

abs :: fn(x i32) -> i32: if x < 0: -x else x
sign :: fn(x i32) -> i32: if x > 0: 1 else if x < 0: -1 else 0

follow :: fn(hx i32, hy i32, tx i32, ty i32, visited *[[bool; 2000]; 2000]) -> (i32, i32) {
     while abs(hx - tx) > 1 or abs(hy - ty) > 1 {
        t := follow_once(hx, hy, tx, ty)
        tx = t.0
        ty = t.1
        visited^[tx + 1000][ty + 1000] = true
    }
    ret (tx, ty)
}

follow_once :: fn(hx i32, hy i32, tx i32, ty i32) -> (i32, i32) {
    if abs(hx - tx) > 1 or abs(hy - ty) > 1 {
        tx += sign(hx - tx)
        ty += sign(hy - ty)
    }
    ret (tx, ty)
}

count_visited :: fn(v *[[bool; 2000]; 2000]) -> u32 {
    c := 0
    x := 0
    y := 0
    while x < 2000 {
        # nested while was broken
        if v^[x][y]: c += 1
        x += 1
        if x == 2000 and y < 1999 {
            x = 0
            y += 1
        }
    }
    ret c
}

move :: fn(cmd (Dir, i32), x *i32, y *i32) {
    (d, a) := cmd
    match d {
        .L: x^ -= a,
        .R: x^ += a,
        .U: y^ += a,
        .D: y^ -= a,
    }
}

a :: fn(cmds List[(Dir, i32)]) -> u32 {
    visited: *[[bool; 2000]; 2000] = std.c.malloc(2000*2000) as _
    # tuple here was broken
    (head_x, head_y) := (0, 0)
    tail := (0, 0)
    i := 0
    while i < cmds.len {
        visited^[tail.0 + 1000][tail.1 + 1000] = true
        move(cmds.get(i), &head_x, &head_y)
        tail = follow(head_x, head_y, tail.0, tail.1, visited)
        i += 1
    }
    ret count_visited(visited)
}

b :: fn(cmds List[(Dir, i32)]) -> u32 {
    visited: *[[bool; 2000]; 2000] = std.c.malloc(2000*2000) as _
    ignore_visited: *[[bool; 2000]; 2000] = std.c.malloc(2000*2000) as _

    rope := std.c.malloc(10*4*2) as *[(i32, i32); 10]
    i := 0
    while i < cmds.len {
        visited^[rope^[9].0 + 1000][rope^[9].1 + 1000] = true
        (d, c) := cmds.get(i)
        m := 0
        while m < c {
            match d {
                .L: rope^[0].0 -= 1,
                .R: rope^[0].0 += 1,
                .U: rope^[0].1 += 1,
                .D: rope^[0].1 -= 1,
            }
            m += 1
            t := 1
            while t < 10 {
                rope^[t] = follow_once(rope^[t-1].0, rope^[t-1].1, rope^[t].0, rope^[t].1)
                t += 1
            }
            visited^[rope^[9].0 + 1000][rope^[9].1 + 1000] = true
        }
        i += 1
    }
    ret count_visited(visited)
}
