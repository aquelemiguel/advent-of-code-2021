use std::fs;

type Matrix = Vec<Vec<u32>>;

fn main() {
    let input = fs::read_to_string("input/day03").expect("Error while reading");

    let matrix: Matrix = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!("\nP1: {}\nP2: {}", p1(&matrix), p2(&matrix));
}

fn p1(matrix: &Matrix) -> u32 {
    let gamma: Vec<u32> = (0..matrix[0].len())
        .map(|i| {
            let col: Vec<u32> = matrix.iter().map(|row| row[i]).collect();
            get_most_common_bit(&col, 1, false)
        })
        .collect();

    let epsilon = complement_binary(&gamma);

    let (gamma, epsilon) = (binary_to_decimal(&gamma), binary_to_decimal(&epsilon));
    gamma * epsilon
}

fn p2(matrix: &Matrix) -> u32 {
    let mut oxygen = matrix.to_owned();
    let mut i = 0;

    while i < matrix[0].len() && oxygen.len() != 1 {
        let col: Vec<u32> = oxygen.iter().map(|row| row[i]).collect();
        let mcb = get_most_common_bit(&col, 1, false);

        oxygen = oxygen.into_iter().filter(|o| o[i] == mcb).collect();
        i += 1;
    }

    let mut carbon = matrix.to_owned();
    let mut i = 0;

    while i < matrix[0].len() && carbon.len() != 1 {
        let col: Vec<u32> = carbon.iter().map(|row| row[i]).collect();
        let mcb = get_most_common_bit(&col, 0, true);

        carbon = carbon.into_iter().filter(|o| o[i] == mcb).collect();
        i += 1;
    }

    let (oxygen, carbon) = (binary_to_decimal(&oxygen[0]), binary_to_decimal(&carbon[0]));
    oxygen * carbon
}

fn complement_binary(bin: &[u32]) -> Vec<u32> {
    bin.iter().map(|bit| (*bit == 0) as u32).collect()
}

fn binary_to_decimal(n: &[u32]) -> u32 {
    let tmp: String = n
        .iter()
        .map(|bit| bit.to_string())
        .collect::<Vec<String>>()
        .join("");

    u32::from_str_radix(&tmp, 2).expect("Could not convert binary string to u32")
}

fn get_most_common_bit(n: &[u32], tiebreaker: u32, invert: bool) -> u32 {
    let zeroes = n.iter().filter(|&bit| *bit == 0).count();
    let ones = n.len() - zeroes;

    if zeroes != ones {
        ((zeroes < ones) ^ invert) as u32
    } else {
        tiebreaker
    }
}
