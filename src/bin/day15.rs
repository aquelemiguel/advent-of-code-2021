use itertools::{iproduct, Itertools};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    iter::FromIterator,
};

struct Edge {
    node: usize,
    cost: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = std::fs::read_to_string("input/d15-f").expect("Error while reading");

    let mut cave = generate_small_cave(&input);
    cave[0][0] = 0;

    let adj = get_adjacency_matrix(&cave);

    match dijkstra(&adj) {
        Some(risk) => println!("P1: {}", risk),
        None => unreachable!(),
    }

    let mut cave = generate_big_cave(&input);
    cave[0][0] = 0;

    let adj = get_adjacency_matrix(&cave);

    match dijkstra(&adj) {
        Some(risk) => println!("P2: {}", risk),
        None => unreachable!(),
    }
}

fn get_adjacency_matrix(cave: &[Vec<usize>]) -> Vec<Vec<Edge>> {
    let mapping: HashMap<(usize, usize), usize> = HashMap::from_iter(
        (0..cave.len())
            .cartesian_product(0..cave.len())
            .enumerate()
            .map(|(i, pair)| (pair, i)),
    );

    let mut adj: Vec<Vec<Edge>> = vec![];

    for node in iproduct!(0..cave.len(), 0..cave.len()) {
        let neighbours = get_neighbours(&node, cave);

        let edges = neighbours.into_iter().map(|(x, y)| Edge {
            node: *mapping.get(&(x, y)).unwrap(),
            cost: cave[x][y],
        });

        adj.push(edges.collect_vec());
    }

    adj
}

fn dijkstra(adj: &[Vec<Edge>]) -> Option<usize> {
    let mut dist = (0..adj.len()).map(|_| usize::MAX).collect_vec();
    let mut heap = BinaryHeap::new();

    let goal = adj.len() - 1;

    dist[0] = 0;
    heap.push(State {
        cost: 0,
        position: 0,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for edge in &adj[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

fn generate_small_cave(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec()
}

fn generate_big_cave(input: &str) -> Vec<Vec<usize>> {
    let small_cave = generate_small_cave(input);

    let big_cave = small_cave
        .iter()
        .map(|line| {
            (0..5)
                .map(|i| line.iter().map(|n| (n + i - 1) % 9 + 1).collect_vec())
                .flatten()
                .collect_vec()
        })
        .collect_vec();

    (0..5)
        .map(|i| {
            big_cave
                .iter()
                .map(|line| line.iter().map(|n| (n + i - 1) % 9 + 1).collect_vec())
                .collect_vec()
        })
        .flatten()
        .collect_vec()
}

fn get_neighbours(point: &(usize, usize), cave: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let deltas: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    deltas
        .iter()
        .map(|delta| (delta.0 + point.0 as i32, delta.1 + point.1 as i32))
        .filter(|(x, y)| *x >= 0 && x < &(cave.len() as i32) && *y >= 0 && y < &(cave.len() as i32))
        .map(|(x, y)| (x as usize, y as usize))
        .collect_vec()
}
