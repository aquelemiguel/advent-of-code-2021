use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/d06-full").expect("Error while reading");
    let ages: Vec<u64> = input.split(',').map(|age| age.parse().unwrap()).collect();

    println!("P1: {}", generate(&ages, 80));
    println!("P2: {}", generate(&ages, 256));
}

fn generate(ages: &[u64], days: usize) -> u64 {
    let mut fishes: HashMap<u64, u64> = HashMap::new();

    ages.iter()
        .for_each(|fish| *fishes.entry(*fish).or_insert(0) += 1);

    (0..days).for_each(|_| step(&mut fishes));
    fishes.iter().fold(0, |acc, fish| acc + *fish.1 as u64)
}

fn step(fishes: &mut HashMap<u64, u64>) {
    let snapshot = fishes.clone();
    fishes.remove(&0);

    (1..=8).for_each(|k| {
        if let Some(v) = fishes.clone().get(&k) {
            *fishes.entry(k - 1).or_insert(0) += v;
        }
        fishes.remove(&k);
    });

    if let Some(children) = snapshot.get(&0) {
        *fishes.entry(6).or_insert(0) += children;
        *fishes.entry(8).or_insert(0) += children;
    }
}
