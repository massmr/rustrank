use crate::graph::MatrixGraph;

#[derive(Debug)]
pub struct MatrixRank {
    graph: MatrixGraph,
    damping_factor: f64,
    max_iter: usize,
    tolerance: f64,
}

impl MatrixRank {
    // returns a new instance of MatrixRank
    pub fn new(
        graph: MatrixGraph,
        damping_factor: f64, 
        max_iter: usize, 
        tolerance: f64) -> MatrixRank {
        MatrixRank {
            graph,
            damping_factor,
            max_iter,
            tolerance,
        }
    }

    pub fn page_rank (&self) {

        // Initialize n as the number of nodes
        // Initialize the rank vector as 1/n
        // Initialize the new rank vector as a zero vector
        // Initialize the out degree vector as a zero vector
        // Initialize the matrix as the graph matrix
        // Initialize the random jump factor as (1 - damping factor) / n
        let n = self.graph.get_nodes();
        let mut rank = vec![1.0 / n as f64; n];
        let mut new_rank = vec![0.0; n];
        let mut out_degree = vec![0; n];
        let matrix = self.graph.get_matrix();
        let JUMP_FACTOR: f64 = (1.0 - self.damping_factor) / n as f64;

        // Calculate the out degree of each node
        for i in 0..n {
            for j in 0..n {
                // If there is a link from node i to node j
                // Increase the out degree of node i
                out_degree[i] += matrix[i][j];
            }
        }

        for _ in 0..self.max_iter {

            // Initialize new_rank vector as jump factor
            // Add rank contribution of each linked node
            for i in 0..n {
                new_rank[i] = JUMP_FACTOR;
                for j in 0..n {
                    if matrix[j][i] == 1 {
                        new_rank[i] += self.damping_factor * rank[j] / out_degree[j] as f64;
                    }
                }
            }
            
            // Calculate the difference between the new rank and the rank
            // as the sum of the absolute values of the differences
            // Indeed, the difference is verified for all nodes
            let mut diff = 0.0;
            for i in 0..n {
                diff += (new_rank[i] - rank[i]).abs();
            }
            
            // If the difference is less than the tolerance, break
            if diff < self.tolerance {
                break;
            }

            rank = new_rank.clone();
        }

        // Display the rank of each node
        for i in 0..n {
            println!("Node {} : {}", i + 1, rank[i]);
        }      
    }
}