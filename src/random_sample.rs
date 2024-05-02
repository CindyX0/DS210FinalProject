use rand::thread_rng;
use rand::seq::SliceRandom;

//takes sample size of data because its too big to process
pub fn sample_edges(graph: &Vec<(usize, usize)>, sample_size: usize) -> Vec<(usize, usize)> {
    let mut rng = thread_rng();
    let mut sampled_edges = Vec::new();
    let mut shuffled_edges = graph.clone();
    shuffled_edges.shuffle(&mut rng);
    sampled_edges.extend(shuffled_edges.into_iter().take(sample_size));

    println!("sampling in process"); //see if it works here, aslso opted to keep this print statement in to see where program is.
    println!("");
   return sampled_edges;
}