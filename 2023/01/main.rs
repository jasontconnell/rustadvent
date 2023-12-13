fn get_input() -> Vec<String> {
    let filename = match std::env::args().nth(1) {
        Some(s) => s,
        None => String::from("input.txt")
    };
    let contents = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<String> = contents.lines().map(str::to_string).collect();
    return lines.clone();
}

fn main(){
    let lines: Vec<String> = get_input();
    let p1: i32 = part1(lines.clone());
    let p2: i32 = part2(lines.clone());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(lines: Vec<String>) -> i32 {
    let mut ret: i32 = 0;
    for line in lines.iter() {
        let mut nums = Vec::new();
        for (_i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                let n = match c.to_string().parse::<i32>() {
                    Ok(n) => n,
                    Err(_e) => 0
                };
                nums.push(n);
            }
        }
        let f: i32 = match nums.get(0) {
            Some(n) => *n,
            None => 0
        };
        let lst: i32 = match nums.last() {
            Some(n) => *n,
            None => 0
        };
        ret += (f*10) + lst;
    }
    return ret;
}

fn part2(lines: Vec<String>) -> i32 {
    let numlookup: [&str; 10] = ["zero","one","two","three","four","five","six","seven","eight","nine"];
    let mut ret: i32 = 0;
    for line in lines.iter() {
        let mut nums = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                let n = match c.to_string().parse::<i32>() {
                    Ok(n) => n,
                    Err(_e) => 0
                };
                nums.push(n);
            } else {
                for x in 1..10 {
                    let s = numlookup[x];
                    if i + s.len() <= line.len() && line[i..i+s.len()] == *s {
                        nums.push(x as i32);
                        break;
                    }
                }
            }
        }
        let f: i32 = match nums.get(0) {
            Some(n) => *n,
            None => 0
        };
        let lst: i32 = match nums.last() {
            Some(n) => *n,
            None => 0
        };
        ret += (f*10) + lst;
    }

    return ret;
}
