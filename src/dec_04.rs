use std::fs;

pub fn run() {
    let data = fs::read_to_string("dec_04.txt").expect("unable to read file");

    let mut cards = vec![vec![vec![]]];
    let mut card = vec![vec![]];
    let mut bingo_balls = vec![0];

    for line in data.lines().rev() {
        match line.len() {
            i if i > 16 => {
                bingo_balls = line.split(",").map(|s| s.parse().unwrap()).collect();
            }
            i if i == 14 => {
                let bingo_line: Vec<i32> = line.split(" ").filter_map(|s| s.parse().ok()).collect();

                card.push(bingo_line);
            }
            0 => {
                cards.push(card.clone());
                card.clear();
            }
            _ => println!("Bad data"),
        }
    }

    let mut high_score = 500;
    let mut winning_card = 0;
    let mut winning_line = vec![];
    for (card_index, card) in cards.iter().enumerate() {
        let mut row = vec![0; 6];
        let mut col = vec![0; 6];
        let mut score_line = vec![];

        for (score, ball) in bingo_balls.iter().enumerate() {
            for (x, line) in card.iter().enumerate() {
                for (y, n) in line.iter().enumerate() {
                    if n == ball {
                        row[x] += 1;
                        col[y] += 1;
                        score_line.push(*ball);
                        if (row[x] == 5) || (col[y] == 5) {
                            if score < high_score {
                                high_score = score;
                                winning_card = card_index;
                                winning_line = score_line.clone();
                            }
                        }
                    }
                }
            }
        }
    }

    let sum1: i32 = winning_line.iter().sum();

    let mut sum2 = 0;
    for card in &cards[winning_card] {
        for n in card {
            sum2 += n;
        }
    }

    let awnser = (sum2 - sum1) * winning_line.last().unwrap();
    println!("Awnser {}", awnser);
}
