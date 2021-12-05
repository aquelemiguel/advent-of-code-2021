use regex::Regex;
use std::{cmp::min, collections::HashMap};

#[derive(Debug)]
struct Vent {
    start: (i32, i32),
    end: (i32, i32),
}

type Ocean = HashMap<(i32, i32), i32>;

fn main() {
    let input = std::fs::read_to_string("input/d05-full").expect("Error while reading");
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    let vents: Vec<Vent> = input
        .lines()
        .map(|line| {
            let capture = re.captures_iter(line).next().unwrap();
            let coords: Vec<i32> = (1..5).map(|i| capture[i].parse().unwrap()).collect();

            Vent {
                start: (coords[0], coords[1]),
                end: (coords[2], coords[3]),
            }
        })
        .collect();

    println!("P1: {}", draw_lines(&vents, false));
    println!("P2: {}", draw_lines(&vents, true));
}

fn is_straight_line(vent: &Vent) -> bool {
    vent.start.0 == vent.end.0 || vent.start.1 == vent.end.1
}

fn get_overlaps(ocean: &Ocean) -> usize {
    ocean.values().filter(|vent| *vent >= &2).count()
}

fn draw_lines(vents: &[Vent], use_diagonals: bool) -> usize {
    let mut ocean: Ocean = HashMap::new();

    for vent in vents.iter() {
        if is_straight_line(vent) {
            let (dx, dy) = (
                i32::abs(vent.start.0 - vent.end.0),
                i32::abs(vent.start.1 - vent.end.1),
            );

            let (sx, sy) = (min(vent.start.0, vent.end.0), min(vent.start.1, vent.end.1));

            (sx..=sx + dx).for_each(|x| {
                (sy..=sy + dy).for_each(|y| {
                    *ocean.entry((x, y)).or_insert(0) += 1;
                })
            });
        } else if use_diagonals {
            let (dx, dy) = (
                i32::signum(vent.end.0 - vent.start.0),
                i32::signum(vent.end.1 - vent.start.1),
            );

            let mut curr = vent.start;
            *ocean.entry(curr).or_insert(0) += 1;

            while curr.0 != vent.end.0 && curr.1 != vent.end.1 {
                curr = (curr.0 + dx, curr.1 + dy);
                *ocean.entry(curr).or_insert(0) += 1;
            }
        }
    }

    get_overlaps(&ocean)
}
