use std.string
use std.ptr_add
use std.panic
use std.list.List
use std.c.printf

Shape :: enum { Rock Paper Scissors }
shape_score:: fn(shape Shape) -> u32: match shape {
    .Rock: 1,
    .Paper: 2,
    .Scissors: 3
}
Goal :: enum { Lose Draw Win }

main :: fn {
    input := std.file.read_to_string("src/input/day2.txt")
    lines := string.lines(input.ptr)
    
    printf("%d".ptr, lines.len)

    shapes := List.new()

    i := 0
    points := 0
    while i < lines.len {
        line := lines.get(i)

        if (std.string.len(line) > 0) {
            a := match u8(line^) {
                65: .Rock,
                66: .Paper,
                67: .Scissors,
                _: panic("invalid input")
            }

            b := match u8(ptr_add(line, 2)^) {
                88: .Rock,
                89: .Paper,
                90: .Scissors,
                _: panic("invalid input")
            }

            shapes.push((a, b))
        }
        i += 1

    }
    printf("Part a: %d\n".ptr, a(&shapes))
    printf("Part b: %d\n".ptr, b(&shapes as _))
}

winning :: fn(s Shape) -> Shape: match s {
    .Rock: .Paper,
    .Paper: .Scissors,
    .Scissors: .Rock,
}

losing :: fn(s Shape) -> Shape: match s {
    .Rock: .Scissors,
    .Paper: .Rock,
    .Scissors: .Paper,
}

round_score :: fn(a Shape, b Shape) -> u32 {
    score := shape_score(b)

    if a == b: score += 3
    else if winning(a) == b: score += 6
    ret score
}

a :: fn(shapes *List[(Shape, Shape)]) -> u32 {
    score := 0

    i := 0
    while i < shapes.len {
        line := shapes.get(i)
        a := line.0
        b := line.1

        score += round_score(a, b)

        i += 1
    }

    ret score
}

b :: fn(shapes *List[(Shape, Goal)]) -> u32 {
    score := 0

    i := 0
    while i < shapes.len {
        line := shapes.get(i)
        shape := line.0

        my_shape := match line.1 {
            .Lose: losing(shape),
            .Draw: shape,
            .Win: winning(shape)
        }
        score += round_score(shape, my_shape)

        i += 1
    }   
    ret score
}