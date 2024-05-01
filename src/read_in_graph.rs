use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct Graph {
    pub n: usize,
    pub edges: Vec<(usize, usize)>,
}


pub fn read_file(filename: &str) -> Graph {
    let file = File::open(filename).expect("File failed to open 1");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().expect("Failed to read line").parse().expect("Invalid input, n");
    let mut edges = Vec::new();
    for line in lines {
        let line = line.expect("Failed to read line");
        let mut iter = line.split_whitespace();
        let from: usize = iter.next().expect("Invalid input").parse().expect("Invalid input");
        let to: usize = iter.next().expect("Invalid input").parse().expect("Invalid input");
        edges.push((from, to));
    }
    println!("completed reading file into graph <3.");  // opted to write this statement so we know when the program finishes running

    return Graph { n, edges };

}


