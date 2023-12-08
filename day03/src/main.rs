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

    // find the loccations of the symbols and numbers
    let mut coords_symbols:Vec<(usize, usize)> = Vec::new();
    let mut coords_nums:Vec<(usize, usize)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                coords_nums.push((i, j));
            }
            else if c != '.' {
                coords_symbols.push((i, j));
            }
        }
    }

    // reuturn a number at coordinate
    let get_num = |a:usize, b:usize| -> u32 {
        match lines[a].chars().nth(b).unwrap().to_digit(10) {
            Some(x) => x,
            None    => 0
        }
    };

    // return if a coordinate is adjacent to a symbol
    let mut found:bool = false;
    let mut get_is_by_symbol = |a:usize, b:usize| -> bool {
        found = false;
        for (i, j) in coords_symbols.iter() {
            found = ((*i as isize - a as isize).abs() < 2) &&
                ((*j as isize - b as isize).abs() < 2);
            if found { break; }
        }
        found
    };

    // parse the numbers and find ones next to symbols
    let (mut prev_i, mut prev_j) = coords_nums.remove(0);
    let mut num:u32              = get_num(prev_i, prev_j);
    let mut is_by_symbol         = false;
    for (i, j) in coords_nums.iter() {
        is_by_symbol |= get_is_by_symbol(prev_i, prev_j);
        if (prev_i == *i) && ((*j - prev_j) == 1) {
            num = num * 10 + get_num(*i, *j);
        }
        else {
            if is_by_symbol {
                sum += num;
            }
            is_by_symbol = false; 
            num = get_num(*i, *j);
        }
        (prev_i, prev_j) = (*i, *j);
    }
    if is_by_symbol || get_is_by_symbol(prev_i, prev_j){
        sum += num;
    }

    sum
}

fn part_2(lines: Vec<String>) -> u32 {
    let mut sum: u32= 0;

    // find the loccations of the symbols and numbers
    let mut is_parsing = false;
    let mut num:u32 = 0;
    let mut map_nums:Vec<(u32, Vec<(usize, usize)>)> = Vec::new();
    let mut coords_syms:Vec<(usize, usize)>            = Vec::new();
    let mut coords_num = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                num = num * 10 + c.to_digit(10).unwrap();
                is_parsing = true;
                coords_num.push((i, j));
            }
            else {
                if c != '.' {
                    coords_syms.push((i, j));
                }
                if is_parsing {
                    map_nums.push((num, coords_num.clone()));
                    num = 0;
                }
                coords_num = Vec::new();
                is_parsing = false;
            }
        }
    }

    // return if a coordinate is adjacent to anythin in the vec
    fn get_is_adj(a:usize, b:usize, check:&Vec<(usize, usize)>) -> bool {
        let mut found:bool = false;
        for (i, j) in check.iter() {
            found = ((*i as isize - a as isize).abs() < 2) &&
                ((*j as isize - b as isize).abs() < 2);
            if found { break; }
        }
        found
    }

    // parse the numbers and find ones next to symbols
    for (i, j) in coords_syms.iter() {
        let mut ratio = 1;
        let mut ratio_num = 0;
        for (num, v) in map_nums.iter() {
            if get_is_adj(*i, *j, v) {
                ratio_num += 1;
                ratio *= *num;
            }
        }
        if ratio_num > 1 {
            sum += ratio;
        }
    }

    sum
}

fn main() {
    let input = get_text("input.txt").unwrap();

    // part 1
    let test_input = get_text("part1_example.txt").unwrap();
    assert!(part_1(test_input.clone()) == 4361);

    let p1_answer = part_1(input.clone());
    println!("part_1 == {}", p1_answer);

    // part 2
    assert!(part_2(test_input.clone()) == 467835);

    let p2_answer = part_2(input.clone());
    println!("part_2 == {}", p2_answer);
}
