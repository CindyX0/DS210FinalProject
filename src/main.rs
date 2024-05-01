
mod analyze_bfs;
use crate::analyze_bfs::stats;

mod read_in_graph;

mod random_sample;

mod open_adj_list;

mod bfs;

mod bfs_distance;

fn main() {
    let graph = read_in_graph::read_file("twitter.txt");
    let edges = &graph.edges;
    let list_edges = random_sample::sample_edges(edges, 1500);
    let adjacent_edges = open_adj_list::adj_list(&list_edges, graph.n);
    bfs::compute_bfs(0, &graph, &adjacent_edges);
    let distance = bfs_distance::return_distance(0, &graph, &adjacent_edges);
    let results = bfs_distance::stats(&distance);
    println!("{:?}", results);
}