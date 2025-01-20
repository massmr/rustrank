// PageRank algorithm in Rust
// By massmr
// Date: 2025-17-01

mod graph;
mod pagerank;
mod file;
use crate::graph::MatrixGraph;
use crate::pagerank::MatrixRank;
use crate::file::read_csv;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Read csv file
    let file_path = "assets/data/fullsample.csv";
    let matrix = read_csv(file_path)?;

    // Create matrix graph from csv file
    let matrix_graph = MatrixGraph::new(matrix);
    
    // Apply PageRank algorithm
    let matrix_rank = MatrixRank::new(matrix_graph, 0.85, 100, 0.0001);
    matrix_rank.page_rank();

    Ok(())
}