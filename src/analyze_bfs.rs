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

    (max_distance, min_distance, mean_distance, max_node, min_node)
}