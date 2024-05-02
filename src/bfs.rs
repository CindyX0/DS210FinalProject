use std::collections::VecDeque;
use crate::read_in_graph::Graph;

//computes and prints bfs, returns value distance to be processed later in stats analysis

pub fn compute_bfs(start: usize, graph: &Graph, adj_list: &Vec<Vec<usize>>) -> Vec<Option<u32>>{
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
   // println!("Vertex to Distance will be shown below");  //print statement to test that it works up till here, it does print,
    
    for (v, dist) in distance.iter().enumerate() {
        //println!("   {}:{}", v, dist.unwrap_or_default());    //removed this line becuase it took so long to load!!
        
    }
    return distance;
}



pub fn compute_avg_distance(adj_list: &Vec<Vec<usize>>, graph:&Graph) -> f64 {
    let mut total_distance = 0;
    let mut count = 0;
    for start in 0..graph.n {
        let distance = compute_bfs(start, graph, adj_list);
        for dist in distance {
            if let Some(d) = dist {
                total_distance += d;
                count += 1;
                //println!("computing average distance");
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

    for (node, &dist) in distance.iter().enumerate() { 
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

    println!(""); // spacing

    println!("stats finished calculating"); // print statement added

    return(max_distance, min_distance, mean_distance, max_node, min_node);
    

}



//tests//

//test for bfs

#[test]
fn test_compute_bfs() {
    let graph = Graph {
        n: 6,
        edges: vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
            (2, 4),
            (3, 4),
            (4, 5),
        ],
    };

    let adj_list = vec![
        vec![1, 2],
        vec![0, 3],
        vec![0, 3, 4],
        vec![1, 2, 4],
        vec![2, 3, 5],
        vec![4],
    ];

    let start = 0;
    let distance = compute_bfs(start, &graph, &adj_list);

    assert_eq!(distance, vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3)]);
}

//tests for stats
#[test]
fn test_stats() {
    let distance = vec![Some(0), Some(1),Some(1), Some(2),  Some(2), Some(3)];

    let (max_distance, min_distance, mean_distance, max_node, min_node) = stats(&distance);

    assert_eq!(max_distance, 3);
    assert_eq!(min_distance, 0);
    assert_eq!(mean_distance, 1.5);
    assert_eq!(max_node, Some(5));
    assert_eq!(min_node, Some(0));
}

    //test for compute_avg_distance
    #[test]
    fn test_compute_avg_distance() {
        let graph = Graph {
            n: 6,
            edges: vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 5),
            ],
        };
        let adj_list = vec![
            vec![1, 2],
            vec![0, 3],
            vec![0, 3, 4],
            vec![1, 2, 4],
            vec![2, 3, 5],
            vec![4],
        ];
        let avg_distance = compute_avg_distance(&adj_list, &graph);
        assert_eq!(avg_distance, 1.3888888888888888);
    }
