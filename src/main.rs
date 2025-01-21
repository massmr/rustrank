// PageRank algorithm in Rust
// By massmr
// Date: 2025-17-01

mod graph;
mod pagerank;
mod file;
use std::env;
use crate::graph::MatrixGraph;
use crate::pagerank::MatrixRank;
use crate::file::read_csv;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let args : Vec<String> = env::args().collect();
    let file_path = &args[1] || "./assets/data/easysample.csv";
    
    // Create matrix and node names from csv file
    let (matrix, node_names) = read_csv(file_path)?;

    // Create MatrixGraph from matrix
    let matrix_graph = MatrixGraph::new(matrix);
   
    // Apply PageRank algorithm
    let matrix_rank = MatrixRank::new(matrix_graph, node_names, 0.85, 100, 0.0001);
    matrix_rank.page_rank();

    Ok(())
}