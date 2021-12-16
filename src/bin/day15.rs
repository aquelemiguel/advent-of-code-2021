use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/d15-f").expect("Error while reading");

    let mut small_cave = generate_small_cave(&input);
    small_cave[0][0] = 0;

    println!("P1: {}", dijkstra(&small_cave));

    let mut big_cave = generate_big_cave(&input);
    big_cave[0][0] = 0;

    println!("P2: {}", dijkstra(&big_cave));
}

fn dijkstra(cave: &Vec<Vec<usize>>) -> usize {
    let mut unvisited: HashSet<(usize, usize)> = HashSet::from_iter(
        (0..cave.len())
            .cartesian_product(0..cave.len())
            .collect_vec(),
    );

    let mut dist: HashMap<(usize, usize), usize> =
        HashMap::from_iter(unvisited.iter().map(|point| (*point, usize::MAX)));

    dist.insert((0, 0), 0).unwrap();

    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    while !unvisited.is_empty() {
        let next = dist.clone();
        let next = next
            .iter()
            .filter(|entry| unvisited.contains(entry.0))
            .sorted_by(|a, b| Ord::cmp(a.1, b.1))
            .nth(0)
            .unwrap();

        unvisited.remove(next.0);

        let neighbours = get_neighbours(next.0, cave.len());

        let neighbours = neighbours
            .iter()
            .filter(|vertex| unvisited.contains(vertex))
            .collect_vec();

        if next.0 .0 == cave.len() - 1 && next.0 .1 == cave.len() - 1 {
            break;
        }

        for neighbour in neighbours {
            let distance = next.1 + cave[neighbour.0][neighbour.1];

            if distance < *dist.get(neighbour).unwrap() {
                dist.insert(*neighbour, distance);
                prev.insert(*neighbour, *next.0);
            }
        }
    }

    *dist.get(&(cave.len() - 1, cave.len() - 1)).unwrap()
}

fn generate_small_cave(input: &String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec()
}

fn generate_big_cave(input: &String) -> Vec<Vec<usize>> {
    let small_cave = generate_small_cave(input);

    let big_cave = small_cave
        .iter()
        .map(|line| {
            (0..5)
                .map(|i| line.iter().map(|n| (n + i - 1) % 9 + 1).collect_vec())
                .flatten()
                .collect_vec()
        })
        .collect_vec();

    (0..5)
        .map(|i| {
            big_cave
                .iter()
                .map(|line| line.iter().map(|n| (n + i - 1) % 9 + 1).collect_vec())
                .collect_vec()
        })
        .flatten()
        .collect_vec()
}

fn get_neighbours(point: &(usize, usize), cavern_size: usize) -> Vec<(usize, usize)> {
    let deltas: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    deltas
        .iter()
        .map(|delta| (delta.0 + point.0 as i32, delta.1 + point.1 as i32))
        .filter(|(x, y)| {
            *x >= 0 && x < &(cavern_size as i32) && *y >= 0 && y < &(cavern_size as i32)
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect_vec()
}
