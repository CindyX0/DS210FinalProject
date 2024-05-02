

mod read_in_graph;
mod random_sample;
mod open_adj_list;
mod bfs;
use crate::open_adj_list::clean_sample;



fn main() {

    //read graph into file
    
    let graph = read_in_graph::read_file("twitter.txt");

    // store edges of graph in variable
    let edges = &graph.edges;

    // sampling 50,000 edges, minmum amount was 1,000, wanted a sample that could still visualize connections in wich all of them werent broken

    let list_edges = random_sample::sample_edges(edges, 50_000);

    println!("{}", "list edges shown being made"); // print statement to see where the program is 

    println!("");
    //println!("{:?}", list_edges);

  //utilizing clean_sample
    let clean_edges = clean_sample(list_edges);

    let adjacent_edges = open_adj_list::adj_list(&clean_edges, graph.n);//used to be graph.n // put highest node found in data set
    println!("{}", "adjacent edges in process");
    println!("");
    //println!("{:?}", adjacent_edges);
    
    let distances = bfs::compute_bfs(0, &graph, &adjacent_edges);
    println!("{}", "computing in process");
    let stats_results: (u32, u32, f64, Option<usize>, Option<usize>) = bfs::stats(&distances);

    let avg_dist = bfs::compute_avg_distance(&adjacent_edges, &graph);
    
    // print statements to visualize all the data

    
    //print statements for statistical analysis
    println!("");
    println!("");
    println!("Nodes Statistics:");
    println!("");
    println!("As Analyzed From All Nodes",);
    println!("Maximum distance value is {} and it is from this node: {}", stats_results.0, stats_results.3.unwrap_or_default());
    println!("Minimum distance value is {} and it is from this node: {}", stats_results.1, stats_results.4.unwrap_or_default());
    println!("Mean value: {}", stats_results.2);
    println!("Average distance: {}", avg_dist);


}


