use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
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

    let straight: Vec<Vent> = vents
        .clone()
        .into_iter()
        .filter(|vent| is_straight_line(vent))
        .collect();

    println!("P1: {}", get_overlaps(&straight));
    println!("P2: {}", get_overlaps(&vents));
}

fn is_straight_line(vent: &Vent) -> bool {
    vent.start.0 == vent.end.0 || vent.start.1 == vent.end.1
}

fn draw_line(ocean: &mut Ocean, vent: &Vent) {
    let (dx, dy) = (
        i32::signum(vent.end.0 - vent.start.0),
        i32::signum(vent.end.1 - vent.start.1),
    );

    let steps = i32::max(
        i32::abs(vent.start.0 - vent.end.0),
        i32::abs(vent.start.1 - vent.end.1),
    );

    let points: Vec<(i32, i32)> = (0..steps)
        .scan(vent.start, |state, _| {
            *state = (state.0 + dx, state.1 + dy);
            Some(*state)
        })
        .collect();

    for point in [points.as_slice(), &[vent.start]].concat() {
        *ocean.entry(point).or_insert(0) += 1;
    }
}

fn get_overlaps(vents: &[Vent]) -> usize {
    let mut ocean: Ocean = HashMap::new();

    for vent in vents.iter() {
        draw_line(&mut ocean, vent);
    }

    ocean.values().filter(|vent| *vent >= &2).count()
}
