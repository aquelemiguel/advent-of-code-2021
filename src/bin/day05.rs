use regex::Regex;
use std::collections::HashMap;

struct Vent {
    start: (i32, i32),
    end: (i32, i32),
}

type Ocean = HashMap<(i32, i32), i32>;

fn main() {
    let input = std::fs::read_to_string("input/d05-full").expect("Error while reading");
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    let vents = input.lines().map(|line| {
        let capture = re.captures_iter(line).next().unwrap();
        let coords: Vec<i32> = (1..5).map(|i| capture[i].parse().unwrap()).collect();

        Vent {
            start: (coords[0], coords[1]),
            end: (coords[2], coords[3]),
        }
    });

    let (mut straight, mut diagonals): (Vec<Vent>, Vec<Vent>) =
        vents.partition(|vent| is_straight_line(vent));
    println!("P1: {}", generate_ocean_floor(&straight));

    straight.append(&mut diagonals);
    println!("P2: {}", generate_ocean_floor(&straight));
}

fn is_straight_line(vent: &Vent) -> bool {
    vent.start.0 == vent.end.0 || vent.start.1 == vent.end.1
}

fn get_overlaps(ocean: &Ocean) -> usize {
    ocean.values().filter(|vent| *vent >= &2).count()
}

fn draw_line(ocean: &mut Ocean, vent: &Vent) {
    let (dx, dy) = (
        i32::signum(vent.end.0 - vent.start.0),
        i32::signum(vent.end.1 - vent.start.1),
    );

    let mut curr = vent.start;
    *ocean.entry(curr).or_insert(0) += 1;

    while curr.0 != vent.end.0 || curr.1 != vent.end.1 {
        curr = (curr.0 + dx, curr.1 + dy);
        *ocean.entry(curr).or_insert(0) += 1;
    }
}

fn generate_ocean_floor(vents: &[Vent]) -> usize {
    let mut ocean: Ocean = HashMap::new();

    for vent in vents.iter() {
        draw_line(&mut ocean, vent);
    }

    get_overlaps(&ocean)
}
