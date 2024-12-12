use std.list.List
use std.panic
use std.print
use std.println
use std.string.str

Cpu :: struct {
    cycle u64
    x i64
    signal_strength i64

    new :: fn -> Cpu: Cpu(0, 1, 0)

    next_cycle :: fn(this *Cpu) {
        this.cycle += 1
        if (i64(this.cycle) - 20) % 40 == 0 {
            this.signal_strength += this.x * i64(this.cycle)
        }
        h_pos := i64((this.cycle - 1) % 40)
        c := if this.x - h_pos >= -1 and this.x - h_pos <= 1: "#" else "."
        print(c)
        if h_pos == 39: print("\n")
    }

    run :: fn(this *Cpu, cmds List[str]) {
        i := 0
        while i < cmds.len {
            s := cmds.get(i).split(" ")
            (_, op) := s.next()
            match op {
                "noop": this.next_cycle(),
                "addx" {
                    this.next_cycle()
                    this.next_cycle()
                    (_, n) := s.next()
                    this.x += n.parse()
                }
                _ {
                    print(op)
                    panic(" is an invalid op")
                }
            }
            i += 1
        }
    }
}

main :: fn {
    println("Day 10:")
    input := std.file.read_to_string("src/input/day10.txt")
    lines := input.trim().lines()

    cpu := Cpu.new()
    cpu.run(lines)
    std.c.printf("\tPart 1: %d\n".ptr, cpu.signal_strength)

}
