use std::collections::HashSet;

type Basin = HashSet<(usize, usize)>;

fn main() {
    let input = std::fs::read_to_string("input/d09-full").expect("Error while reading");

    let mut matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // Pad the matrix with 9's to better handle out-of-bounds
    matrix.iter_mut().for_each(|line| {
        line.append(&mut vec![9, 9]);
        line.rotate_right(1);
    });

    matrix.insert(0, vec![9; matrix[0].len()]);
    matrix.push(vec![9; matrix[0].len()]);

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
    let mut lowest_points: Vec<(usize, usize)> = vec![];

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            if is_low_point(&(i, j), matrix) {
                lowest_points.push((i, j));
            }
        }
    }

    lowest_points
}

fn is_low_point(p: &(usize, usize), matrix: &Vec<Vec<u32>>) -> bool {
    matrix[p.0][p.1] < matrix[p.0 - 1][p.1]
        && matrix[p.0][p.1] < matrix[p.0 + 1][p.1]
        && matrix[p.0][p.1] < matrix[p.0][p.1 - 1]
        && matrix[p.0][p.1] < matrix[p.0][p.1 + 1]
}

fn flood_fill(point: &(usize, usize), matrix: &Vec<Vec<u32>>, basin: &mut HashSet<(usize, usize)>) {
    if !basin.contains(&point) {
        basin.insert(*point);

        if matrix[point.0 - 1][point.1] != 9 {
            flood_fill(&(point.0 - 1, point.1), matrix, basin);
        }
        if matrix[point.0 + 1][point.1] != 9 {
            flood_fill(&(point.0 + 1, point.1), matrix, basin);
        }
        if matrix[point.0][point.1 - 1] != 9 {
            flood_fill(&(point.0, point.1 - 1), matrix, basin);
        }
        if matrix[point.0][point.1 + 1] != 9 {
            flood_fill(&(point.0, point.1 + 1), matrix, basin);
        }
    }
}
