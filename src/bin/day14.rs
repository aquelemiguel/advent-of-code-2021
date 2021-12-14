use itertools::Itertools;
use std::{collections::HashMap, iter::FromIterator};

fn main() {
    let input = std::fs::read_to_string("input/d14-f").expect("Error while reading");
    let lines = input.lines().collect_vec();

    let (template, rules) = (lines[0].to_string(), &lines[2..]);

    let rules: HashMap<String, String> = HashMap::from_iter(
        rules
            .iter()
            .map(|rule| rule.split(" -> ").collect_tuple().unwrap())
            .map(|(k, v)| (k.to_string(), v.to_string())),
    );

    let start = template.chars().collect_vec();

    let mut pairs: HashMap<String, usize> =
        HashMap::from_iter(start.windows(2).map(|w| (w.iter().collect::<String>(), 1)));

    let mut letters: HashMap<char, usize> = template.chars().counts();

    for _ in 0..10 {
        pairs = step(pairs, &mut letters, &rules);
    }

    println!("P1: {}", calculate_difference(&letters));

    for _ in 10..40 {
        pairs = step(pairs, &mut letters, &rules);
    }

    println!("P2: {}", calculate_difference(&letters));
}

fn calculate_difference(letters: &HashMap<char, usize>) -> u64 {
    let mut occs = letters.values().collect_vec();
    occs.sort();
    (occs[occs.len() - 1] - occs[0]) as u64
}

fn step(
    pairs: HashMap<String, usize>,
    letters: &mut HashMap<char, usize>,
    rules: &HashMap<String, String>,
) -> HashMap<String, usize> {
    let mut snapshot = HashMap::new();

    for (k, v) in pairs.iter() {
        let middle = rules.get(k).unwrap();
        let left = format!("{}{}", k.chars().next().unwrap(), middle);
        let right = format!("{}{}", middle, k.chars().nth(1).unwrap());

        *snapshot.entry(left.clone()).or_insert(0) += v;
        *snapshot.entry(right.clone()).or_insert(0) += v;

        *letters.entry(middle.chars().next().unwrap()).or_insert(0) += v;
    }

    snapshot
}
