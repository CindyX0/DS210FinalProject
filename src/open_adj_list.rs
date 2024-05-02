use std::collections::HashSet; 
use std::collections::HashMap;

//fixes the empty space and discrepencies created  as a result of n being graph.n, despite having a sampling of a smaller size,
//basically will save so much time


// renumbering and re-indexing process, looks at all the edges, find all the unique nodes in that, then sort them from least to greatest
//then creates a hash map where the key is the origional node and the value is the index of that node in the sorted unique list
//then goes over the list of edges and repalce all the nodes with the number made in hashmap


pub fn clean_sample(edges: Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    //hash map of unique nodes
    let mut unique_nodes: HashSet<usize> = HashSet::new();
    for (i, j) in edges.iter() {
        unique_nodes.insert(*i);
        unique_nodes.insert(*j);
    }
    let mut sorted_nodes: Vec<usize> = unique_nodes.into_iter().collect();
    sorted_nodes.sort();

    //creating hash map for nodes
    let mut node_map: HashMap<usize, usize> = HashMap::new();
    for (index, node) in sorted_nodes.iter().enumerate() {
        node_map.insert(*node, index);
    }

    let mut cleaned_edges: Vec<(usize, usize)> = Vec::new();
    for (i, j) in edges.iter() {
        let cleaned_i = *node_map.get(i).unwrap();
        let cleaned_j = *node_map.get(j).unwrap();
        cleaned_edges.push((cleaned_i, cleaned_j));
    }

    cleaned_edges
}


//creates adjaceny list

pub fn adj_list(graph: &Vec<(usize, usize)>, num_nodes: usize) -> Vec<Vec<usize>> {
    let mut graph_list: Vec<Vec<usize>> = vec![vec![]; num_nodes];
    for &(v, w) in graph {
        graph_list[v].push(w);
    }
    //println!("completed creating adjacency list");  // print statement to test if the function reached this step! 
    
    return graph_list;
}
