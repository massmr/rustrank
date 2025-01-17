// PageRank algorithm in Rust
// By massmr
// Date: 2025-17-01

mod graph;
use crate::graph::{MatrixGraph, VecGraph};

fn main() {
    let matrix_graph = MatrixGraph::new(vec![
        vec![0, 1, 1, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 0, 1],
        vec![0, 0, 1, 0],
    ]);
    println!("Matrix graph :");
    matrix_graph.display();

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