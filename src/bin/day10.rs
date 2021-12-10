use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/d10-full").expect("Error while reading");

    let token_map = HashMap::from([('}', '{'), (')', '('), (']', '['), ('>', '<')]);
    let corruption_point_map = HashMap::from([('(', 3), ('[', 57), ('{', 1197), ('<', 25137)]);
    let completion_point_map = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let (mut corrupted, mut completed) = (vec![], vec![]);

    'next: for line in input.lines() {
        let mut stack: Vec<char> = vec![];

        for c in line.chars() {
            if let Some(err) = step(&mut stack, &c, &token_map) {
                let points = *corruption_point_map.get(&err).unwrap() as u64;
                corrupted.push(points);
                continue 'next;
            }
        }

        if !stack.is_empty() {
            completed.push(complete(&stack, &completion_point_map));
        }
    }

    println!("P1: {}", corrupted.iter().sum::<u64>());

    completed.sort();
    println!("P2: {}", completed[completed.len() / 2]);
}

fn complete(stack: &Vec<char>, point_map: &HashMap<char, u64>) -> u64 {
    stack
        .iter()
        .rev()
        .fold(0, |acc, c| acc * 5 + point_map.get(c).unwrap())
}

fn step(stack: &mut Vec<char>, c: &char, token_map: &HashMap<char, char>) -> Option<char> {
    match token_map.values().find(|t| *t == c) {
        Some(_) => {
            stack.push(*c);
        }
        None => {
            let converted = token_map.get(&c).unwrap();
            if stack.pop().unwrap() != *converted {
                return Some(*converted);
            }
        }
    }
    None
}
