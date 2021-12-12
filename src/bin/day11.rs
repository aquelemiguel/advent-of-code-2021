use itertools::{iproduct, Itertools};

fn main() {
    let input = std::fs::read_to_string("input/d11-full").expect("Error while reading");

    let mut o1 = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();

    let mut o2 = o1.clone();

    let flashes = (0..100).fold(0, |acc, _| acc + step(&mut o1));
    println!("P1: {}", flashes);

    let mut i = 0;

    while !is_full_flash(&o2) {
        step(&mut o2);
        i += 1;
    }
    println!("P2: {}", i);
}

fn step(octopi: &mut Vec<Vec<i32>>) -> usize {
    let mut flashes = 0;

    for row in octopi.iter_mut() {
        for octopus in row {
            *octopus += 1;
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            if octopi[i][j] > 9 {
                octopi[i][j] = 0;
                flashes += charge_neighbours(octopi, (i, j)) + 1;
            }
        }
    }

    flashes
}

fn is_full_flash(octopi: &[Vec<i32>]) -> bool {
    octopi.iter().all(|row| row.iter().all(|o| *o == 0))
}

fn charge_neighbours(octopi: &mut Vec<Vec<i32>>, octopus: (usize, usize)) -> usize {
    let deltas = iproduct!(-1..=1, -1..=1)
        .filter(|(i, j)| !(*i == 0 && *j == 0))
        .collect_vec();

    let neighbours = deltas
        .iter()
        .map(|(i, j)| (octopus.0 as i32 + i, octopus.1 as i32 + j))
        .filter(|(i, j)| *i >= 0 && *i < 10 && *j >= 0 && *j < 10)
        .map(|(i, j)| (i as usize, j as usize))
        .collect_vec();

    neighbours.into_iter().fold(0, |acc, (i, j)| {
        if octopi[i][j] == 0 {
            return acc;
        }
        octopi[i][j] += 1;

        if octopi[i][j] > 9 {
            octopi[i][j] = 0;
            acc + charge_neighbours(octopi, (i, j)) + 1
        } else {
            acc
        }
    })
}
