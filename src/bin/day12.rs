use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
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
        self.adj
            .entry(a.to_string())
            .or_insert(HashSet::new())
            .insert(b.to_string());

        self.adj
            .entry(b.to_string())
            .or_insert(HashSet::new())
            .insert(a.to_string());
    }

    fn can_visit(&self, path: &Vec<String>, node: &String, can_visit_twice: bool) -> bool {
        // println!("Trying to insert {} into {:?}", node, path);

        if *node == node.to_lowercase() {
            if node == "start" {
                return false;
            }

            if !can_visit_twice {
                return !path.contains(node);
            } else {
                let occs = path.iter().cloned().counts();
                // println!("{:?}", occs);

                return match occs
                    .iter()
                    .find(|&(k, v)| *k == k.to_lowercase() && *v == 2)
                {
                    Some(cave) => cave.0 != node && !occs.contains_key(node),
                    None => true,
                };
            }
        }
        true
    }

    fn dfs(&mut self, node: String, can_visit_twice: bool, path: Vec<String>) {
        let mut curr_path = path.clone();
        curr_path.push(node.clone());

        if node == "end" {
            return self.paths.push(curr_path);
        }

        let edges = self.adj.get(&node).unwrap().clone();

        for edge in edges {
            if self.can_visit(&curr_path, &edge, can_visit_twice) {
                self.dfs(edge.to_string(), can_visit_twice, curr_path.clone());
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
    println!("P1: {}", graph.paths.iter().count());

    graph.paths.clear();

    graph.dfs("start".to_string(), true, vec![]);
    println!("P2: {:?}", graph.paths.iter().count());
}
