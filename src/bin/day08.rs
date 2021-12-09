use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

type Display = Vec<HashSet<char>>;

fn main() {
    let input = std::fs::read_to_string("input/d08-full").expect("Error while reading");

    let mut lines: Vec<(Display, Display)> = input
        .lines()
        .map(|line| {
            let parts: Vec<Display> = line
                .split(" | ")
                .map(|part| {
                    part.split(' ')
                        .map(|s| HashSet::from_iter(s.chars()))
                        .collect()
                })
                .collect();

            (parts[0].clone(), parts[1].clone())
        })
        .collect();

    let easy_digits: Vec<i32> = vec![2, 3, 4, 7]
        .iter()
        .map(|i| {
            lines
                .iter()
                .map(|display| display.1.iter().filter(|o| o.len() == *i).count() as i32)
                .sum()
        })
        .collect();

    println!("P1: {}", easy_digits.iter().sum::<i32>());

    let res: Vec<i32> = lines
        .iter_mut()
        .map(|display| {
            let mappings = decode_display(&mut display.0);

            let conversions: Vec<&i32> = display
                .1
                .iter()
                .map(|digit| mappings.iter().find(|(_, v)| digit == **v).unwrap().0)
                .collect();

            conversions.iter().fold(0, |acc, x| acc * 10 + *x)
        })
        .collect();

    println!("P2: {}", res.iter().sum::<i32>());
}

fn decode_display(display: &mut Display) -> HashMap<i32, &HashSet<char>> {
    display.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    let mut mapping: HashMap<i32, &HashSet<char>> = HashMap::new();

    for digit in display {
        if digit.len() == 2 {
            mapping.insert(1, digit);
        } else if digit.len() == 3 {
            mapping.insert(7, digit);
        } else if digit.len() == 4 {
            mapping.insert(4, digit);
        } else if digit.len() == 5 {
            if digit.is_superset(mapping.get(&7).unwrap()) {
                mapping.insert(3, digit);
            } else if mapping.get(&4).unwrap().difference(digit).count() == 1 {
                mapping.insert(5, digit);
            } else {
                mapping.insert(2, digit);
            }
        } else if digit.len() == 6 {
            if digit.is_superset(mapping.get(&4).unwrap()) {
                mapping.insert(9, digit);
            } else if digit.is_superset(mapping.get(&7).unwrap()) {
                mapping.insert(0, digit);
            } else {
                mapping.insert(6, digit);
            }
        } else if digit.len() == 7 {
            mapping.insert(8, digit);
        }
    }

    mapping
}
