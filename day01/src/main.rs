use std::fs::File;
use std::io::{BufReader, BufRead};

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
        // get the firt digit
        for c in line.chars() {
            if c.is_numeric() {
                sum += 10 * c.to_digit(10).unwrap();
                break;
            }
        }
        // get the last digit
        for c in line.chars().rev() {
            if c.is_numeric() {
                sum += c.to_digit(10).unwrap();
                break;
            }
        }
    }

    sum
}

fn parse_num(line: &str) -> Option<u32> {
    if line.len() == 0 {
        return None;
    }
    if line.chars().next().unwrap().is_numeric() {
        return line.chars().next().unwrap().to_digit(10)
    }
    else if line.starts_with("zero")  { return Some(0) }
    else if line.starts_with("one")   { return Some(1) }
    else if line.starts_with("two")   { return Some(2) }
    else if line.starts_with("three") { return Some(3) }
    else if line.starts_with("four")  { return Some(4) }
    else if line.starts_with("five")  { return Some(5) }
    else if line.starts_with("six")   { return Some(6) }
    else if line.starts_with("seven") { return Some(7) }
    else if line.starts_with("eight") { return Some(8) }
    else if line.starts_with("nine")  { return Some(9) }
    else {return None }
}

fn part_2(lines: Vec<String>) -> u32 {
    let mut sum: u32= 0;
    for line in lines.iter() {
        // get the firt digit
        for i in 0..line.len() {
            let (_, substr) = line.split_at(i);
            if let Some(num) = parse_num(substr) {
                sum += 10 * num;
                break;
            }
        }
        // get the last digit
        for i in (0..line.len()).rev() {
            let (_, substr) = line.split_at(i);
            if let Some(num) = parse_num(substr) {
                sum += num;
                break;
            }
        }
    }

    sum
}


fn main() {
    let input = get_text("input.txt").unwrap();

    // part 1
    let p1_input = get_text("part1_example.txt").unwrap();
    assert!(part_1(p1_input) == 142);

    let p1_answer = part_1(input.clone());
    println!("part_1 == {}", p1_answer);


    // part 2
    let p2_input = get_text("part2_example.txt").unwrap();
    assert!(part_2(p2_input) == 281);

    let p2_answer = part_2(input.clone());
    println!("part_2 == {}", p2_answer);
}
