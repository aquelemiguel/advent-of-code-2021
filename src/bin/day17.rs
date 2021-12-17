use itertools::Itertools;
use regex::Regex;
use std::ops::RangeInclusive;

type TargetArea = (RangeInclusive<i32>, RangeInclusive<i32>);
type Point = (i32, i32);

fn main() {
    let input = std::fs::read_to_string("input/d17-f").expect("Error while reading");
    let area = parse_input(&input);

    let mut peaks: Vec<i32> = vec![];
    let mut valid: Vec<Point> = vec![];

    for vx in -250..250 {
        for vy in -250..250 {
            match fire_probe(&area, &(vx, vy)) {
                Some(steps) => {
                    let highest = get_peak(&steps);
                    peaks.push(highest);
                    valid.push((vx, vy));
                }
                None => {}
            }
        }
    }

    peaks.sort();
    println!("P1: {:?}", peaks.last().unwrap());
    println!("P2: {:?}", valid.len());
}

fn fire_probe(area: &TargetArea, vel: &Point) -> Option<Vec<Point>> {
    let (mut pos, mut vel) = ((0, 0), vel.clone());
    let mut steps: Vec<Point> = vec![(0, 0)];

    for _ in 0..1000 {
        step(&mut pos, &mut vel);
        steps.push(pos);

        if landed(&area, &pos) {
            return Some(steps);
        }
    }
    None
}

fn step(pos: &mut Point, vel: &mut Point) {
    pos.0 += vel.0;
    pos.1 += vel.1;

    if vel.0 > 0 {
        vel.0 -= 1;
    } else if vel.0 < 0 {
        vel.0 += 1;
    }

    vel.1 -= 1;
}

fn get_peak(points: &Vec<Point>) -> i32 {
    let mut points = points.clone();
    points.sort_by(|a, b| b.1.cmp(&a.1));
    points.first().unwrap().1
}

fn landed(area: &TargetArea, pos: &Point) -> bool {
    area.0.contains(&pos.0) && area.1.contains(&pos.1)
}

fn parse_input(input: &str) -> TargetArea {
    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let caps = re.captures(&input).unwrap();

    let coords = [&caps[1], &caps[2], &caps[3], &caps[4]]
        .iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect_vec();

    (coords[0]..=coords[1], coords[2]..=coords[3])
}
