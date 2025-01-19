// PageRank algorithm in Rust
// By massmr
// Date: 2025-17-01

mod graph;
mod pagerank;
use crate::graph::{MatrixGraph, VecGraph};
use crate::pagerank::MatrixRank;

fn main() {

    // Create a matrix graph
    // i as column and j as row
    let matrix_graph = MatrixGraph::new(vec![
        vec![0, 1, 1, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 0, 1],
        vec![0, 0, 1, 0],
    ]);
    println!("Matrix graph :");
    matrix_graph.display();
    
    // Calculate MatrixGraph page rank
    let matrix_rank = MatrixRank::new(matrix_graph, 0.85, 100, 0.0001);
    matrix_rank.page_rank();

    let vec_graph = VecGraph::new(
        vec![
         0, 1, 1, 0,
         1, 0, 1, 0,
         0, 1, 0, 1,
         0, 0, 1, 0
         ], 4);
    println!("Vector graph :");
    vec_graph.display();
}