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






fn part_1(lines: Vec<String>) -> u64 {
    let mut times = Vec::new();
    let mut dists = Vec::new();

    let parse_nums = |i:usize, arr:&mut Vec<_>| {
        for num in lines[i].split_whitespace() {
            if let Ok(num) = num.parse::<u32>() {
                arr.push(num);
            }
        }
    };
    parse_nums(0, &mut times);
    parse_nums(1, &mut dists);

    // d = distance traveled
    // t = total time
    // h = time held
    //
    // d = (t - h)*(h)
    // h = t/2 +- sqrt(t^2/4-d)

    let mut sum = 1;
    for i in 0..times.len() {
        let d = dists[i] ;
        let t = times[i];
        let min = (t as f64)/2.0 - f64::sqrt(((t*t) as f64)/4.0 - (d as f64));
        let max = (t as f64)/2.0 + f64::sqrt(((t*t) as f64)/4.0 - (d as f64 + 1.0));

        let diff = (max.floor() as u64) - (min.floor() as u64);
        sum *= diff;
    }

    sum
}



fn part_2(lines: Vec<String>) -> u64 {
    let mut time = Vec::new();
    let mut dist = Vec::new();

    let parse_nums = |i:usize, arr:&mut Vec<_>| {
        for num in lines[i].split(":") {
            let num = num.replace(" ", "");
            if let Ok(num) = num.parse::<u64>() {
                arr.push(num);
            }
        }
    };
    parse_nums(0, &mut time);
    parse_nums(1, &mut dist);

    // d = distance traveled
    // t = total time
    // h = time held
    //
    // d = (t - h)*(h)
    // h = t/2 +- sqrt(t^2/4-d)

    let d = dist[0];
    let t = time[0];
    let min = (t as f64)/2.0 - f64::sqrt(((t*t) as f64)/4.0 - (d as f64));
    let max = (t as f64)/2.0 + f64::sqrt(((t*t) as f64)/4.0 - (d as f64 + 1.0));

    (max.floor() as u64) - (min.floor() as u64)
}

fn main() {
    let input = get_text("input.txt").unwrap();

    // part 1
    let test_input = get_text("part1_example.txt").unwrap();
    assert!(part_1(test_input.clone()) == 288);

    let p1_answer = part_1(input.clone());
    println!("part_1 == {}", p1_answer);

    // part 2
    assert!(part_2(test_input.clone()) == 71503);

    let p2_answer = part_2(input.clone());
    println!("part_2 == {}", p2_answer);
}

