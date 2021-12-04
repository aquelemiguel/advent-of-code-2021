use std::fs;

fn main() {
    let input = fs::read_to_string("input/day01").expect("Error while reading");

    let depths: Vec<i32> = input
        .lines()
        .map(|line| line.to_string().parse::<i32>().unwrap())
        .collect();

    println!("P1: {}", p1(&depths));
    println!("P2: {}", p2(&depths));
}

fn p1(depths: &Vec<i32>) -> i32 {
    let mut last_depth = &depths[0];
    let mut count = 0;

    for depth in depths.iter().skip(1) {
        if depth > last_depth {
            count += 1;
        }
        last_depth = depth;
    }

    count
}

// TODO: ARRAYWINDOWS
fn p2(depths: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut last_window: i32 = depths[0..3].iter().sum();

    for i in 1..depths.len() - 2 {
        let curr_window: i32 = depths[i..i + 3].iter().sum();

        if curr_window > last_window {
            count += 1;
        }

        last_window = curr_window;
    }

    count
}
