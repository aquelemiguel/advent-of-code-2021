use itertools::Itertools;
use std::collections::HashSet;

type Point = (i32, i32);

fn main() {
    let input = std::fs::read_to_string("input/d13-f").expect("Error while reading");
    let (mut points, folds) = parse_input(&input);

    for (i, fold) in folds.iter().enumerate() {
        step(&mut points, fold);

        if i == 0 {
            println!("P1: {}", points.len());
        }
    }

    let mut output: [[char; 40]; 6] = [[' '; 40]; 6];

    points
        .into_iter()
        .for_each(|(i, j)| output[j as usize][i as usize] = '#');

    println!("P2:");

    for line in output {
        println!("{}", line.iter().collect::<String>());
    }
}

fn step(points: &mut HashSet<Point>, fold: &Point) {
    let snapshot = points.clone();

    let inside: HashSet<&Point> = snapshot
        .iter()
        .filter(|point| point.0 >= fold.0 && point.1 >= fold.1)
        .collect();

    for point in inside {
        let folded = ((2 * fold.0 - point.0).abs(), (2 * fold.1 - point.1).abs());
        points.remove(point);
        points.insert(folded);
    }
}

fn parse_input(input: &str) -> (HashSet<Point>, Vec<Point>) {
    let lines: Vec<&str> = input.lines().collect();
    let (points, folds) = lines.split(|line| line.is_empty()).collect_tuple().unwrap();

    let points: HashSet<Point> = points
        .iter()
        .map(|point| {
            point
                .split(',')
                .map(|p| p.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let folds: Vec<Point> = folds
        .iter()
        .map(|fold| {
            let split = fold.split('=').collect_vec();
            let mut point: Point = (split[1].parse().unwrap(), 0);

            if split[0].ends_with('y') {
                std::mem::swap(&mut point.0, &mut point.1);
            }
            point
        })
        .collect();

    (points, folds)
}
