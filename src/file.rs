use std::error::Error;
use csv::Reader;

// Structure to store csv data
#[derive(Debug)]
pub struct Record{
    emmiting_node: String,
    receiving_node: String,
}

impl Record {
    
    // public function to read csv file
    // input : file path &str
    // output : Result Vector of Records || Boxed dynamic error
    pub fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
        
        // Create csv reader - ? propagates error if file not found
        let mut reader = Reader::from_path(file_path)?;

        // Create vector to store records
        let mut records = Vec::new();
    
        // push record to records vector
        for result in reader.records() {
            let record = result?;
            records.push(Record {
                emmiting_node: record[0].to_string(),
                receiving_node: record[1].to_string(),
            });
        }

        // return records vector
        Ok(records)
    }

    pub fn display_csv(records: &Vec<Record>) {
        for record in records {
            println!("Emmiting node : {}", record.emmiting_node);
            println!("Receiving node : {}", record.receiving_node);
        }
    }
}