use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn get_text(f_name: &str) -> Option<Vec<String>> {
    let mut str_vec= Vec::new();

    let lines = match File::open(f_name) {
        Ok(f)  => Some(BufReader::new(f).lines()),
        Err(_) => None
    };
    if let Some(lines) = lines {
        for l in lines {
            str_vec.push(l.unwrap());
        }
        Some(str_vec)
    }
    else {
        None
    }
}

fn part_1(lines: Vec<String>) -> u32 {
    let mut sum: u32= 0;
    for line in lines.iter() {
        // parse nums into hash sets
        let mut nums_card = HashSet::new();
        let mut nums_wins = HashSet::new();
        let mut is_card = false;
        let mut is_wins = false;
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<usize>() {
                if is_card {
                    nums_card.insert(num);
                }
                else if is_wins {
                    nums_wins.insert(num);
                }
            }
            if token.contains(":") {
                is_card = true;
            }
            else if token == "|"  {
                is_card = false;
                is_wins = true;
            }
        }

        // intersection of hash sets is power + 1
        let pow = nums_wins.intersection(&nums_card).collect::<HashSet<_>>().len();
        sum += match pow {
            0 => 0,
            _ => 2_u32.pow((pow-1) as u32)
        };
    }
    sum
}

fn part_2(lines: Vec<String>) -> u32 {
    let mut sum: u32= 0;
    let mut num_cards = vec![1; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        // parse nums into hash sets
        let mut nums_card = HashSet::new();
        let mut nums_wins = HashSet::new();
        let mut is_card = false;
        let mut is_wins = false;
        for token in line.split_whitespace() {
            if let Ok(num) = token.parse::<usize>() {
                if is_card {
                    nums_card.insert(num);
                }
                else if is_wins {
                    nums_wins.insert(num);
                }
            }
            if token.contains(":") {
                is_card = true;
            }
            else if token == "|"  {
                is_card = false;
                is_wins = true;
            }
        }

        // intersection of hash sets is power + 1
        let score = match nums_wins.intersection(&nums_card).collect::<HashSet<_>>().len() {
            0 => 0,
            x => x
        };
        for n in i..(i  + (score as usize)) {
            if n < num_cards.len() - 1 {
                num_cards[n+1] += num_cards[i];
            }
        }
    }
    for num in num_cards.iter() {
        sum += *num as u32;
    }
    sum
}

fn main() {
    let input = get_text("input.txt").unwrap();

    // part 1
    let test_input = get_text("part1_example.txt").unwrap();
    assert!(part_1(test_input.clone()) == 13);

    let p1_answer = part_1(input.clone());
    println!("part_1 == {}", p1_answer);

    // part 2
    assert!(part_2(test_input.clone()) == 30);

    let p2_answer = part_2(input.clone());
    println!("part_2 == {}", p2_answer);
}

