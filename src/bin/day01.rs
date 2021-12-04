use std::fs;

fn main() {
    let depths = read_input("input/01-example");
    println!("P1: {}\nP2: {}", p1(&depths), p2(&depths));
}

fn read_input(file_name: &str) -> Vec<i32> {
    fs::read_to_string(file_name)
        .expect("Error while reading")
        .lines()
        .map(|line| line.to_string().parse::<i32>().unwrap())
        .collect()
}

fn p1(depths: &[i32]) -> usize {
    depths.windows(2).filter(|pair| pair[1] > pair[0]).count()
}

fn p2(depths: &[i32]) -> usize {
    depths
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&read_input("input/01-example")), 7);
        assert_eq!(p1(&read_input("input/01-full")), 1400);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&read_input("input/01-example")), 5);
        assert_eq!(p2(&read_input("input/01-full")), 1429);
    }
}
