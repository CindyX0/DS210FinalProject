use std::collections::VecDeque;
use crate::read_in_graph::Graph;


pub fn compute_bfs(start: usize, graph: &Graph, adj_list: &Vec<Vec<usize>>) {
    let mut distance: Vec<Option<u32>> = vec![None; graph.n];
    distance[start] = Some(0);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        for &u in &adj_list[v] {
            if distance[u].is_none() {
                distance[u] = Some(distance[v].unwrap() + 1);
                queue.push_back(u);
            }
        }
    }
    println!("vertex:distance");
    for (v, dist) in distance.iter().enumerate() {
        println!("   {}:{}", v, dist.unwrap_or_default());
    }
}