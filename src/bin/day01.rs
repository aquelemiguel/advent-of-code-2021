use std::fs;

fn main() {
    let input = fs::read_to_string("input/day01").expect("Error while reading");

    let depths: Vec<i32> = input
        .lines()
        .map(|line| line.to_string().parse::<i32>().unwrap())
        .collect();

    println!("P1: {}\nP2: {}", p1(&depths), p2(&depths));
}

fn p1(depths: &[i32]) -> usize {
    depths.windows(2).filter(|pair| pair[1] > pair[0]).count()
}

fn p2(depths: &[i32]) -> usize {
    depths
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
}
