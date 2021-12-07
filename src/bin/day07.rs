fn main() {
    let input = std::fs::read_to_string("input/d07-full").expect("Error while reading");
    let mut crabs: Vec<i32> = input.split(',').map(|c| c.parse().unwrap()).collect();

    crabs.sort();

    println!("P1: {:?}", get_optimal_position(&crabs, true));
    println!("P2: {:?}", get_optimal_position(&crabs, false));
}

fn get_optimal_position(crabs: &[i32], linear: bool) -> i32 {
    (crabs[0]..crabs[crabs.len() - 1]).fold(i32::MAX, |acc, i| {
        let fuel: i32 = if linear {
            align_to_position(&crabs, i).iter().sum()
        } else {
            align_to_position(&crabs, i)
                .iter()
                .map(|step| get_triangular_number(*step))
                .sum()
        };
        i32::min(acc, fuel)
    })
}

fn align_to_position(crabs: &[i32], position: i32) -> Vec<i32> {
    crabs.iter().map(|crab| (crab - position).abs()).collect()
}

fn get_triangular_number(n: i32) -> i32 {
    n * (n + 1) / 2
}
