use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Graph {
    adj: HashMap<String, HashSet<String>>,
    paths: Vec<Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj: HashMap::new(),
            paths: vec![],
        }
    }

    fn add_edge(&mut self, a: &str, b: &str) {
        [(a, b), (b, a)].iter().for_each(|(start, end)| {
            self.adj
                .entry(start.to_string())
                .or_insert_with(HashSet::new)
                .insert(end.to_string());
        });
    }

    fn can_visit(&self, path: &[String], node: &str, can_visit_twice: bool) -> bool {
        if node == "start" {
            return false;
        }

        if *node == node.to_uppercase() {
            return true;
        }

        if can_visit_twice {
            let occs = path.iter().cloned().counts();

            match occs
                .iter()
                .find(|&(k, v)| *k == k.to_lowercase() && *v == 2)
            {
                Some(cave) => cave.0 != node && !occs.contains_key(node),
                None => true,
            }
        } else {
            !path.contains(&node.to_string())
        }
    }

    fn dfs(&mut self, node: String, can_visit_twice: bool, mut path: Vec<String>) {
        path.push(node.clone());

        if node == "end" {
            return self.paths.push(path);
        }

        let edges = self.adj.get(&node).unwrap().clone();

        for edge in edges {
            if self.can_visit(&path, &edge, can_visit_twice) {
                self.dfs(edge.to_string(), can_visit_twice, path.clone());
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/d12-full").expect("Error while reading");
    let mut graph = Graph::new();

    input
        .lines()
        .map(|line| line.split('-').collect_vec())
        .for_each(|pair| graph.add_edge(pair[0], pair[1]));

    graph.dfs("start".to_string(), false, vec![]);
    println!("P1: {}", graph.paths.len());

    graph.paths.clear();

    graph.dfs("start".to_string(), true, vec![]);
    println!("P2: {:?}", graph.paths.len());
}
