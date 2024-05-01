use rand::thread_rng;
use rand::seq::SliceRandom;


pub fn sample_edges(graph: &Vec<(usize, usize)>, sample_size: usize) -> Vec<(usize, usize)> {
    let mut rng = thread_rng();
    let mut sampled_edges = Vec::new();
    let mut shuffled_edges = graph.clone();
    shuffled_edges.shuffle(&mut rng);
    sampled_edges.extend(shuffled_edges.into_iter().take(sample_size));
    sampled_edges
}