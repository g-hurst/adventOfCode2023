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

fn parse_game(line: String) -> (u32, Vec<(u32, u32, u32)>) {
        let mut out = (0, Vec::new());
        let id_split = line.split(":").collect::<Vec<&str>>();
        out.0 = id_split[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        for game in id_split[1].split(";") {
            for dice in game.split(",") {
                let dice = dice.trim().split(" ").collect::<Vec<&str>>();
                let mut rgb:(u32,u32,u32) = (0,0,0);
                match dice[1].trim() {
                    "red"   => rgb.0 = dice[0].parse().unwrap(),
                    "green" => rgb.1 = dice[0].parse().unwrap(),
                    "blue"  => rgb.2 = dice[0].parse().unwrap(),
                    &_      => ()

                };
                out.1.push(rgb);
            }
        }
        out
}

fn part_1(lines: Vec<String>) -> u32 {
    let mut sum: u32= 0;
    let max_rgb = (12, 13, 14);
    for line in lines {
        let mut is_good_game = true;
        let (game_num, rgbs) = parse_game(line);
        for (r,g,b) in rgbs.iter() {
            if (*r > max_rgb.0) || (*g > max_rgb.1) || (*b > max_rgb.2) {
                is_good_game = false;
                break;
            }
        }
        if is_good_game {
            sum += game_num;
        }
    }
    sum
}

fn part_2(lines: Vec<String>) -> u32 {
    let mut sum: u32= 0;
    for line in lines {
        let mut min_rgb = (0, 0, 0);
        let (_, rgbs) = parse_game(line);
        for (r,g,b) in rgbs.iter() {
            if *r > min_rgb.0 {
                min_rgb.0 = *r;
            }
            if *g > min_rgb.1 {
                min_rgb.1 = *g;
            }
            if *b > min_rgb.2 {
                min_rgb.2 = *b;
            }
        }
        sum += min_rgb.0 * min_rgb.1 * min_rgb.2;
    }
    sum
}

fn main() {
    let input = get_text("input.txt").unwrap();

    // part 1
    let test_input = get_text("part1_example.txt").unwrap();
    assert!(part_1(test_input.clone()) == 8);

    let p1_answer = part_1(input.clone());
    println!("part_1 == {}", p1_answer);


    // part 2
    assert!(part_2(test_input.clone()) == 2286);

    let p2_answer = part_2(input.clone());
    println!("part_2 == {}", p2_answer);
}
