fn main() {
    let input = std::fs::read_to_string("input/d07-full").expect("Error while reading");
    let mut crabs: Vec<i32> = input.split(',').map(|c| c.parse().unwrap()).collect();

    crabs.sort();

    println!("P1: {:?}", get_optimal_position(&crabs, true));
    println!("P2: {:?}", get_optimal_position(&crabs, false));
}

fn get_optimal_position(crabs: &[i32], linear: bool) -> i32 {
    (crabs[0]..crabs[crabs.len() - 1]).fold(i32::MAX, |acc, i| {
        let aligned = align_to_position(&crabs, i);

        if linear {
            i32::min(acc, aligned.iter().sum())
        } else {
            i32::min(acc, aligned.iter().map(|n| n * (n + 1) / 2).sum())
        }
    })
}

fn align_to_position(crabs: &[i32], position: i32) -> Vec<i32> {
    crabs.iter().map(|crab| (crab - position).abs()).collect()
}
