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

fn parse_ids(input:String) -> Vec<u64> {
    let mut ids = Vec::new();
    for token in input.split(" ") {
        if let Ok(num) = token.parse::<u64>() {
            ids.push(num);
        }
    }
    ids
}

fn parse_map(input: &mut Vec<String>) -> Option<(Vec<String>,Vec<Vec<u64>>)> {
    if input.len() == 0 { return None; }
    let mut line = input.remove(0);
    if line == "" {
        if input.len() == 0 { return None; }
        line = input.remove(0);
    }
    let map_name = line.clone().split("-to-").map(|x| x.to_string()).collect::<Vec<String>>();
    let mut map_nums = Vec::new();

    if input.len() == 0 { return None; }
    line = input.remove(0);
    while line != "" {
        // println!("{}", line);
        map_nums.push(line.clone().split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>());
        if input.len() == 0 { break }
        line = input.remove(0);
    }
    Some((map_name, map_nums))
}



fn part_1(lines: Vec<String>) -> u64 {
    let mut lines: Vec<String> = lines.clone();
    let mut seed_ids = parse_ids(lines.remove(0));
    while {
        if let Some(map) = parse_map(&mut lines) {
            for i in 0..seed_ids.len() {
                for row in map.1.iter() {
                    let diff = (seed_ids[i] as i64) - (row[1] as i64);
                    if (diff >= 0) && ( diff <= (row[2] as i64)) {
                        seed_ids[i] = row[0] + (diff as u64);
                        break;
                    }
                }
            }
            true
        }
        else {
            false
        }
    }{ }
    *seed_ids.iter().min().unwrap()
}

fn part_2(lines: Vec<String>) -> u64 {
    let mut lines: Vec<String> = lines.clone();
    let mut seed_ids = parse_ids(lines.remove(0));
    let mut almanac = Vec::new();
    while {
        if let Some(map) = parse_map(&mut lines) {
            almanac.push(map);
            true
        }
        else {
            false
        }
    }{ }
    for (text,map) in almanac.iter().rev() {
        println!("{:?}", map);
    }


    *seed_ids.iter().min().unwrap()
}

fn main() {
    let input = get_text("input.txt").unwrap();

    // part 1
    let test_input = get_text("part1_example.txt").unwrap();
    // assert!(part_1(test_input.clone()) == 35);

    // let p1_answer = part_1(input.clone());
    // println!("part_1 == {}", p1_answer);

    println!("test_p2 == {}", part_2(test_input.clone()));
    // part 2
    // assert!(part_2(test_input.clone()) == 30);

    // let p2_answer = part_2(input.clone());
    // println!("part_2 == {}", p2_answer);
}

