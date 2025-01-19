use std::error::Error;
use std::collections::HashMap;
use csv::ReaderBuilder;

pub fn read_csv(file_path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_path(file_path)?;
    let mut matrix = Vec::new();
    let mut nodes = HashMap::new();
    let mut node_count = 0;

    // Première passe pour compter les nœuds et créer le mapping
    for result in rdr.records() {
        let record = result?;
        let input_address = record.get(0).unwrap().to_string();
        let output_address = record.get(1).unwrap().to_string();

        if !nodes.contains_key(&input_address) {
            nodes.insert(input_address.clone(), node_count);
            node_count += 1;
        }
        if !nodes.contains_key(&output_address) {
            nodes.insert(output_address.clone(), node_count);
            node_count += 1;
        }
    }

    matrix.resize(node_count, vec![0; node_count]);

    // Réinitialiser le lecteur CSV pour la deuxième passe
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_path(file_path)?;

    // Deuxième passe pour remplir la matrice d'adjacence
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