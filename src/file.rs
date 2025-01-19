use std::error::Error;
use std::collections::HashMap;
use csv::ReaderBuilder;

pub fn read_csv(file_path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    // read the CSV file, delimiter is ';'
    // Initialize the matrix
    // Initialize nodes HashMap
    // Initialize n as node count
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_path(file_path)?;
    let mut matrix = Vec::new();
    let mut nodes = HashMap::new();
    let mut n = 0;

    // First pass to count the number of nodes
    // Iterate over each row of the CSV file
    for result in rdr.records() {
        let record = result?;
        let input_address = record.get(0).unwrap().to_string();
        let output_address = record.get(1).unwrap().to_string();

        // If the node is not in the HashMap, add it
        if !nodes.contains_key(&input_address) {
            nodes.insert(input_address.clone(), n);
            node_count += 1;
        }
        // If the node is not in the HashMap, add it
        if !nodes.contains_key(&output_address) {
            nodes.insert(output_address.clone(), n);
            node_count += 1;
        }
    }

    // Initialize the matrix with zeros
    matrix.resize(node_count, vec![0; n]);

    // Reset the reader
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_path(file_path)?;

    // Second pass to fill the matrix
    for result in rdr.records() {
        let record = result?;
        let input_address = record.get(0).unwrap().to_string();
        let output_address = record.get(1).unwrap().to_string();

        let i = nodes[&input_address];
        let j = nodes[&output_address];
        matrix[i][j] = 1;
    }

    Ok(matrix)
}