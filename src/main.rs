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
    // Lire le fichier CSV et créer une matrice d'adjacence
    let file_path = "assets/data/sample1.csv";
    let matrix = read_csv(file_path)?;

    // Créer un graphe à partir de la matrice d'adjacence
    let matrix_graph = MatrixGraph::new(matrix);
    println!("Matrix graph :");
    matrix_graph.display();
    
    // Calculer le PageRank pour le graphe
    let matrix_rank = MatrixRank::new(matrix_graph, 0.85, 100, 0.0001);
    matrix_rank.page_rank();

    Ok(())
}