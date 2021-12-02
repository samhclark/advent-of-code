// Day 01, part 1: Sonar Sweep

use std::error::Error;

pub fn part1() -> Result<(), Box<dyn Error>> {
    let input: &str = include_str!("../inputs/day01.in");

    let split_input = input.split_whitespace();

    let mut vec = Vec::<u32>::new();
    for num in split_input {
        let parsed: u32 = num.parse::<u32>()?;
        vec.push(parsed);
    }

    let mut previous = 0;
    let mut increases = 0;

    for depth in vec {
        if previous != 0 && depth > previous {
            increases += 1;
        }
        previous = depth;
    }

    println!("Total number of depth increases: {}", increases);

    Ok(())
}

struct Window {
    a: u32,
    b: u32,
    c: u32,
}

pub fn part2() -> Result<(), Box<dyn Error>> {
    let input: &str = include_str!("../inputs/day01.in");

    let split_input = input.split_whitespace();

    let mut vec = Vec::<u32>::new();
    for num in split_input {
        let parsed: u32 = num.parse::<u32>()?;
        vec.push(parsed);
    }

    let mut window = Window { a: 0, b: 0, c: 0 };
    let mut increases = 0;

    for depth in vec {
        if window.a == 0 {
            window.a = depth;
        } else if window.b == 0 {
            window.b = depth;
        } else if window.c == 0 {
            window.c = depth;
        } else {
            let prev = window.a + window.b + window.c;
            let curr = window.b + window.c + depth;
            if curr > prev {
                increases += 1;
            }
            window.a = window.b;
            window.b = window.c;
            window.c = depth;
        }
    }

    println!("Total number of depth increases: {}", increases);

    Ok(())
}
