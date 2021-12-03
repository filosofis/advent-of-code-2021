use std::fs;

pub fn run() {
    let data = fs::read_to_string("dec_03.txt").expect("unable to read file");

    part1(&data);
    //part2(&data);
}

fn part1(data: &String) {
    let mut number = vec![0; 12];

    for line in data.lines() {
        let mut i = 0;
        for char in line.chars() {
            match char {
                '0' => number[i] -= 1,
                '1' => number[i] += 1,
                _ => println!("Bad data"),
            }
            i += 1;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let mut digit = 0;

    for n in number.iter().rev() {
        match n {
            n if n < &0 => {
                epsilon += u32::pow(2, digit);
            }
            n if n > &0 => {
                gamma += u32::pow(2, digit);
            }
            _ => println!("Bad data"),
        }
        digit += 1;
    }
    println!("{}", gamma * epsilon);
}

fn part2(data: &String) {
    //Make boolean vectors from the string inputs
    let mut numbers = vec![vec![false; 12]];
    for line in data.lines() {
        let mut number = vec![false; 12];

        for (x, char) in line.chars().enumerate() {
            match char {
                '0' => number[x] = false,
                '1' => number[x] = true,
                _ => println!("Bad data"),
            }
        }
        numbers.push(number);
        //println!("{:?}", number)
    }

    
    let mut x = 0;
    while numbers.len() > 1 {
        let common = bit_sum(x, &numbers);
        let mut filtered_numbers = vec![vec![false; 12]];
        println!("{} {} {}", x, common, numbers.len());
        println!("{:?}", numbers);

        for number in &numbers {
            if number[x] == common {
                filtered_numbers.push(number.to_vec())
            }
        }
        x+=1;
        numbers = filtered_numbers.to_vec();
    }
}

/* Returns the most common fist bit */
fn bit_sum(index: usize, numbers: &Vec<Vec<bool>>) -> bool {
    let mut common = 0;
    for number in numbers {
        match number[index] {
            false => common -= 1,
            true => common += 1,
        }
    }
    return common >= 0;
}
