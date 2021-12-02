use std::fs;

pub fn run() {
    let data = fs::read_to_string("dec_02.txt").expect("unable to read file");

    println!("part1 {}", part1(&data));
    println!("part2 {}", part2(&data));
}

fn part1(data: &String) -> u32 {
    let mut distance = 0;
    let mut depth = 0;

    for line in data.lines() {
        let mut chars = line.chars();
        let first = chars.next();
        let num = chars.last().unwrap().to_digit(10).unwrap();

        match first {
            Some('f') => distance += num,
            Some('d') => depth += num,
            Some('u') => depth -= num,
            _ => println!("Bad data"),
        }
    }

    distance * depth
}

fn part2(data: &String) -> u32 {
    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in data.lines() {
        let mut chars = line.chars();
        let first = chars.next();
        let num = chars.last().unwrap().to_digit(10).unwrap();

        match first {
            Some('f') => {
                distance += num;
                depth += num * aim;
            },
            Some('d') => aim += num,
            Some('u') => aim -= num,
            _ => println!("Bad data"),
        }
    }

    distance * depth
}
