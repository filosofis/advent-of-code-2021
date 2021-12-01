use std::fs;

pub fn run() {
    let data = fs::read_to_string("dec_01.txt").expect("unable to read file");
    let numbers: Vec<u32> = data.lines().map(|line| line.parse().unwrap()).collect();

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));

}

fn part1(numbers: &Vec<u32>) -> u32{
    let mut count = 0;
    for i in 1..numbers.len(){
        if numbers[i] > numbers[i-1]{
            count += 1;
        }
    }
    count
}

fn part2(numbers: &Vec<u32>) -> u32{
    let mut sum = u32::MAX;
    let mut count = 0;
    for n in 0..numbers.len() - 2 {
        if (numbers[n] + numbers[n + 1] + numbers[n + 2]) > sum {
            count += 1;
        }
        sum = numbers[n] + numbers[n + 1] + numbers[n + 2];
    }
    count
}
