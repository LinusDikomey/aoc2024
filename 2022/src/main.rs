#![feature(iter_array_chunks, array_chunks, array_windows)]
#![allow(unused)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    let funcs = [
		day1::run,
		day2::run,
		day3::run,
		day4::run,
		day5::run,
		day6::run,
		day7::run,
		day8::run,
		day9::run,
		day10::run,
		day11::run,
		day12::run,
		day13::run,
		day14::run,
		day15::run,
		day16::run,
		day17::run,
		day18::run,
		day19::run,
		day20::run,
		day21::run,
		day22::run,
		day23::run,
		day24::run,
		day25::run,

    ];
    let inputs = [
		include_str!("input/day1.txt"),
		include_str!("input/day2.txt"),
		include_str!("input/day3.txt"),
		include_str!("input/day4.txt"),
		include_str!("input/day5.txt"),
		include_str!("input/day6.txt"),
		include_str!("input/day7.txt"),
		include_str!("input/day8.txt"),
		include_str!("input/day9.txt"),
		include_str!("input/day10.txt"),
		include_str!("input/day11.txt"),
		include_str!("input/day12.txt"),
		include_str!("input/day13.txt"),
		include_str!("input/day14.txt"),
		include_str!("input/day15.txt"),
		include_str!("input/day16.txt"),
		include_str!("input/day17.txt"),
		include_str!("input/day18.txt"),
		include_str!("input/day19.txt"),
		include_str!("input/day20.txt"),
		include_str!("input/day21.txt"),
		include_str!("input/day22.txt"),
		include_str!("input/day23.txt"),
		include_str!("input/day24.txt"),
		include_str!("input/day25.txt"),

    ];
    let mut args = std::env::args().skip(1);
    let mut has_args = false;
    while let Some(arg) = args.next() {
        has_args = true;
        match arg.as_str().parse().expect("Could not parse day input") {
            day@1..=25 => funcs[day-1](inputs[day-1]),
            _ => panic!("Invalid day")
        }
    }
    if !has_args {
        for day in 0..25 {
            funcs[day](inputs[day])
        }
    }
}
