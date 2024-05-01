use std::collections::VecDeque;
use crate::read_in_graph::Graph;
//computes and prints bfs, returns value distance to be processed later in stats analysis

pub fn compute_bfs(start: usize, graph: &Graph, adj_list: &Vec<Vec<usize>>) -> Vec<Option<u32>>{
    let mut distance: Vec<Option<u32>> = vec![None; graph.n];
    distance[start] = Some(0);                       //might be something wrong w this since 0 is all thats being printed for dist
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
    println!("vertex:distance will be shown below");  //print statement to test that it works up till here, it does print,
    //since it takes forever to  print, even with the sampled portion
    for (v, dist) in distance.iter().enumerate() {
       // println!("   {}:{}", v, dist.unwrap_or_default());    //will remove the dashes after cuz it take so long to load!!
        
    }
    return distance;
}


pub fn compute_avg_distance(graph: &Graph, adj_list: &Vec<Vec<usize>>) -> f64 {
    let mut total_distance = 0;
    let mut count = 0;
    for start in 0..graph.n {
        let distance = compute_bfs(start, graph, adj_list);
        for dist in distance {
            if let Some(d) = dist {
                total_distance += d;
                count += 1;
            }
        }
    }
    if count > 0 {
        return total_distance as f64 / count as f64;
    } else {
        return 0.0;
    }
}

// creates analysis based off of bfs results, max, min, mean


pub fn stats(distance: &[Option<u32>]) -> (u32, u32, f64, Option<usize>, Option<usize>) {
    let mut max_distance = 0;
    let mut min_distance = u32::MAX;
    let mut total_distance = 0;
    let mut count = 0;
    let mut max_node = None;
    let mut min_node = None;

    for (node, &dist) in distance.iter().enumerate() { //help
        if let Some(d) = dist {
            if d > max_distance {
                max_distance = d;
                max_node = Some(node);
            }
            if d < min_distance {
                min_distance = d;
                min_node = Some(node);
            }
            total_distance += d;
            count += 1;
        }
    }

    let mean_distance = total_distance as f64 / count as f64;

    (max_distance, min_distance, mean_distance, max_node, min_node)
}