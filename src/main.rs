
use crate::bfs::stats;
use crate::bfs::compute_avg_distance;
use crate::bfs::compute_bfs;


mod read_in_graph;
mod random_sample;
mod open_adj_list;
mod bfs;



fn main() {
    let graph = read_in_graph::read_file("twitter.txt");
    let edges = &graph.edges;
    let list_edges = random_sample::sample_edges(edges, 1500);
    let adjacent_edges = open_adj_list::adj_list(&list_edges, graph.n);
    let distances = bfs::compute_bfs(0, &graph, &adjacent_edges);

    let stats_results  = bfs::stats(&distances);
    let avg_dist = bfs::compute_avg_distance(adjacent_edges, distances);
    

}
