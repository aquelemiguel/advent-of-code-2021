use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/d11-full").expect("Error while reading");

    let mut octopi: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut i = 0;
    let mut flashes = 0;

    let x: usize = loop {
        if i == 100 {
            println!("P1: {}", flashes);
        }

        flashes += step(&mut octopi);

        if is_full_flash(&octopi) {
            break i + 1;
        }

        i += 1;
    };

    println!("P2: {}", x);
}

fn step(octopi: &mut Vec<Vec<i32>>) -> usize {
    let mut flashes = 0;

    for i in 0..octopi.len() {
        for j in 0..octopi[i].len() {
            octopi[i][j] += 1;
        }
    }

    for i in 0..octopi.len() {
        for j in 0..octopi[i].len() {
            if octopi[i][j] > 9 {
                octopi[i][j] = 0;
                flashes += 1;

                let mut rec_flashes = 0;
                charge_neighbours(octopi, &(i, j), &mut rec_flashes);
                flashes += rec_flashes;
            }
        }
    }

    flashes
}

fn is_full_flash(octopi: &Vec<Vec<i32>>) -> bool {
    octopi
        .iter()
        .all(|row| row.iter().all(|octopus| *octopus == 0))
}

fn charge_neighbours(octopi: &mut Vec<Vec<i32>>, octopus: &(usize, usize), flashes: &mut usize) {
    let deltas: Vec<(i32, i32)> = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(i, j)| !(*i == 0 && *j == 0))
        .collect();

    let neighbours: Vec<(i32, i32)> = deltas
        .iter()
        .map(|(i, j)| (octopus.0 as i32 + i, octopus.1 as i32 + j))
        .filter(|(i, j)| *i >= 0 && *i < 10 && *j >= 0 && *j < 10)
        .collect();

    neighbours.iter().for_each(|(i, j)| {
        let (i, j) = (*i as usize, *j as usize);

        if octopi[i][j] > 0 {
            octopi[i][j] += 1;

            if octopi[i][j] > 9 {
                octopi[i][j] = 0;
                *flashes += 1;
                charge_neighbours(octopi, &(i, j), flashes);
            }
        }
    });
}
