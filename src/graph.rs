//graph.rs

// Adjency matrix structure : 
// Slow page rank computation
// High abstraction level
pub struct MatrixGraph {
    matrix_graph: Vec<Vec<i32>>,
}

impl MatrixGraph {
    // returns a new instance of MatrixGraph
    pub fn new(matrix_graph: Vec<Vec<i32>>) -> MatrixGraph {
        MatrixGraph { matrix_graph }
    }

    // returns the number of nodes in the graph
    pub fn get_nodes(&self) -> usize {
        self.matrix_graph.len()
    }

    // get reference to the matrix
    pub fn get_matrix(&self) -> &Vec<Vec<i32>> {
        &self.matrix_graph
    }

    // displays the graph
    pub fn display(&self) {
        for i in 0..self.matrix_graph.len() {
            for j in 0..self.matrix_graph[i].len() {
                print!("{} ", self.matrix_graph[i][j]);
            }
            println!();
        }
    }
}

// 1D vector structure : 
// Faster page rank computation
// Low abstraction level
pub struct VecGraph {
    vec_graph: Vec<i32>,
    col_num: usize,
}

impl VecGraph {
    // returns a new instance of OneDGraph
    pub fn new(vec_graph: Vec<i32>, col_num: usize) -> VecGraph {
        VecGraph {vec_graph, col_num}
    }

    // returns the number of nodes in the graph
    pub fn get_nodes(&self) -> usize {
        self.vec_graph.len() / self.col_num
    }

    // get reference to the vector
    pub fn get_vec(&self) -> &Vec<i32> {
        &self.vec_graph
    }

    // displays the graph
    pub fn display(&self) {
        let rows = self.vec_graph.len() / self.col_num;
        for i in 0..rows {
            for j in 0..self.col_num {
                print!("{}", self.vec_graph[i * self.col_num + j]);
            }
            println!();
        }
    }
}
