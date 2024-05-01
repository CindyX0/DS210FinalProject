

pub fn adj_list(graph: &Vec<(usize, usize)>, num_nodes: usize) -> Vec<Vec<usize>> {
    let mut graph_list: Vec<Vec<usize>> = vec![vec![]; num_nodes];
    for &(v, w) in graph {
        graph_list[v].push(w);
    }
    graph_list
}