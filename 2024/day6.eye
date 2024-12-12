use std.hash.Map

Dir :: enum { Up, Down, Left, Right
    rotate :: fn(self Dir) -> Dir: match self {
        .Up: .Right,
        .Right: .Down,
        .Down: .Left,
        .Left: .Up,
    }

    dir :: fn(self Dir) -> (i32, i32): match self {
        .Up: (0, -1),
        .Down: (0, 1),
        .Left: (-1, 0),
        .Right: (1, 0),
    }

    eq :: fn(self Dir, other Dir) -> bool: match (self, other) {
        (.Up, .Up): true,
        (.Down, .Down): true,
        (.Left, .Left): true,
        (.Right, .Right): true,
        _: false,
    }

    to_string :: fn(self Dir) -> str: match self {
        .Up: "Up",
        .Down: "Down",
        .Left: "Left",
        .Right: "Right",
    }

    ord :: fn(self Dir) -> u8: match self {
        .Up: 0,
        .Down: 1,
        .Left: 2,
        .Right: 3,
    }
}

main :: fn {
    input := std.file.read_to_string("input/day6.txt")
    input: List[str] = collect(input.split("\n"))
    initial_pos := (0, 0)
    dir := Dir.Up
    y := 0
    width := input.get(0).len
    while y < input.len {
        x := 0
        while x < width {
            b := input.get(y).byte(x)
            if b == "^".byte(0) {
                initial_pos = (x as i32, y as i32)
            }
            x += 1
        }
        y += 1
    }

    pos := initial_pos
    positions := Map.new()
    in_bounds := true
    while in_bounds {
        positions.insert(pos, 1)
        o := dir.dir()
        new := (pos.0 + o.0, pos.1 + o.1)
        in_bounds = new.0 >= 0 and new.0 < width as i32 and new.1 >= 0 and new.1 < input.len as i32
        if in_bounds and input.get(new.1 as u64).byte(new.0 as u64)== "#".byte(0) {
            dir = dir.rotate()
        } else {
            pos = new
        }
    }
    print("Part 1: ")
    println(positions.len)

    obstructions: Map[(i32, i32), u8] = Map.new()
    path: Map[(i32, i32, u8), u8] = Map.new()
    pos := initial_pos
    dir := Dir.Up
    in_bounds := true

    while in_bounds {
        path.insert((pos.0, pos.1, dir.ord()), 1)
        o := dir.dir()
        new := (pos.0 + o.0, pos.1 + o.1)
        in_bounds = new.0 >= 0 and new.0 < width as i32 and new.1 >= 0 and new.1 < input.len as i32
        if in_bounds and input.get(new.1 as u64).byte(new.0 as u64)== "#".byte(0) {
            dir = dir.rotate()
        } else {
            if path.get(&(pos.0, pos.1, dir.ord())).is_some() {
                obstructions.insert(new, 1)
            } else {
                println("shit")
                path2 := path.clone()
                tmp := pos
                tmpdir := dir.rotate()
                tmpin_bounds := true
                looped := false

                while tmpin_bounds and !looped {
                    o := tmpdir.dir()
                    tnew := (tmp.0 + o.0, tmp.1 + o.1)
                    tmpin_bounds = tnew.0 >= 0 and tnew.0 < width as i32 and tnew.1 >= 0 and tnew.1 < input.len as i32
                    if tmpin_bounds and (input.get(tnew.1 as u64).byte(tnew.0 as u64)== "#".byte(0) or (tnew.0 == new.0 and tnew.1 == new.1)) {
                        tmpdir = tmpdir.rotate()
                    } else {
                        tmp = tnew
                    }
                    newpos := (tmp.0, tmp.1, tmpdir.ord())
                    if path2.get(&newpos).is_some(): looped = true
                    path2.insert(newpos, 1)
                    print(newpos.0)
                    print(" ")
                    print(newpos.1)
                    print(" ")
                    print(newpos.2)
                    println(looped)
                }
                if looped {
                    obstructions.insert(new, 1)
                }
                std.c.free(path2.table as *_)
            }
            pos = new
        }
    }
    print("Part 2: ")
    println(obstructions.len)
}
