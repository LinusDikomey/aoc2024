#!/usr/bin/python3
import os

mods = ""
funcs = ""
inputs = ""

if input('This will override files in the src directory. Type "override" to proceed at your own risk: ') != 'override':
    print('Cancelling execution')
    exit()

if not os.path.exists("src/input"):
    os.makedirs("src/input")

for i in range(1, 26):
    mods += f"mod day{i};\n"
    funcs += f'\t\tday{i}::run,\n'
    inputs += f'\t\tinclude_str!("input/day{i}.txt"),\n'
    f = open(f"src/day{i}.rs", 'w+')
    f.write(f"""pub fn run(_input: &str) {{
    println!("Day {i}:");
    // ---------- part 1 ----------
    
    println!("\\tPart 1: {{}}", 0);
    // ---------- part 2 ----------
    println!("\\tPart 2: {{}}", 0);
}}""")
    input = open(f"src/input/day{i}.txt", 'w+')

main = open("src/main.rs", 'w+')
main.write(f"""{mods}
fn main() {{
    let funcs = [
{funcs}
    ];
    let inputs = [
{inputs}
    ];
    let mut args = std::env::args().skip(1);
    let mut has_args = false;
    while let Some(arg) = args.next() {{
        has_args = true;
        match arg.as_str().parse().expect("Could not parse day input") {{
            day@1..=25 => funcs[day-1](inputs[day-1]),
            _ => panic!("Invalid day")
        }}
    }}
    if !has_args {{
        for day in 0..25 {{
            funcs[day](inputs[day])
        }}
    }}
}}
""")