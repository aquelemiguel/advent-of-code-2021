use std::collections::HashSet;

type Basin = HashSet<(usize, usize)>;

fn main() {
    let input = std::fs::read_to_string("input/d09-full").expect("Error while reading");

    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let lowest_points: Vec<(usize, usize)> = compute_lowest_points(&matrix);
    let res: u32 = lowest_points.iter().map(|p| matrix[p.0][p.1] + 1).sum();
    println!("P1: {}", res);

    let basins: Vec<Basin> = lowest_points
        .iter()
        .map(|point| {
            let mut basin: HashSet<(usize, usize)> = HashSet::new();
            flood_fill(&point, &matrix, &mut basin);
            basin
        })
        .collect();

    let mut sizes: Vec<usize> = basins.iter().map(|basin| basin.len()).collect();
    sizes.sort();
    sizes.reverse();

    let res = &sizes[0..3].iter().fold(1, |acc, size| acc * size);
    println!("P2: {}", res);
}

fn compute_lowest_points(matrix: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let lowest_points: Vec<Vec<(usize, usize)>> = (0..matrix.len())
        .map(|i| {
            (0..matrix[i].len())
                .filter_map(|j| {
                    let mut is_lowest_point = true;

                    if i > 0 {
                        is_lowest_point &= matrix[i][j] < matrix[i - 1][j];
                    }

                    if i < matrix.len() - 1 {
                        is_lowest_point &= matrix[i][j] < matrix[i + 1][j];
                    }

                    if j > 0 {
                        is_lowest_point &= matrix[i][j] < matrix[i][j - 1];
                    }

                    if j < matrix[i].len() - 1 {
                        is_lowest_point &= matrix[i][j] < matrix[i][j + 1];
                    }

                    if is_lowest_point {
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    lowest_points.iter().flatten().cloned().collect()
}

fn flood_fill(point: &(usize, usize), matrix: &Vec<Vec<u32>>, basin: &mut HashSet<(usize, usize)>) {
    if !basin.contains(&point) {
        return;
    }

    basin.insert(*point);

    if point.0 > 0 && matrix[point.0 - 1][point.1] != 9 {
        flood_fill(&(point.0 - 1, point.1), matrix, basin);
    }

    if point.0 < matrix.len() - 1 && matrix[point.0 + 1][point.1] != 9 {
        flood_fill(&(point.0 + 1, point.1), matrix, basin);
    }

    if point.1 > 0 && matrix[point.0][point.1 - 1] != 9 {
        flood_fill(&(point.0, point.1 - 1), matrix, basin);
    }

    if point.1 < matrix[point.0].len() - 1 && matrix[point.0][point.1 + 1] != 9 {
        flood_fill(&(point.0, point.1 + 1), matrix, basin);
    }
}
