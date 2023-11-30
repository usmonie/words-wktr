mod data;
mod schema;
mod domain;
mod api;

use futures::{AsyncWriteExt, StreamExt};
use serde::{Deserialize, Deserializer};
use std::io;
use std::io::{BufRead, Write, Seek, Read};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let file_path = "wiktionary.json"; // Replace with your JSON file path
    api::launch_server().await;
//    let file = File::open(file_path)?;
//    let reader = io::BufReader::new(file);
//
//    let mut optional_fields = HashMap::new(); // Tracks fields that might need to be optional
//    let mut missing_fields = HashSet::new();  // Tracks fields not present in any Rust model
//
//    for line in reader.lines() {
//        let json_line = line?;
//        let value: Value = serde_json::from_str(&json_line).unwrap();
//        let object = value.as_object().unwrap();
//
//        // Assuming `Word` is one of your main structures. Include others as needed.
//        match serde_json::from_str::<Word>(&json_line) {
//            Ok(_) => {},
//            Err(_) => {
//                for key in object.keys() {
//                    *optional_fields.entry(key.clone()).or_insert(0) += 1;
//                }
//            }
//        }
//
//        // Check against all your model structures
//        // ... (Similar checks for other structures)
//
//        // Track fields not present in any Rust model
//        for key in object.keys() {
//            if !Word::has_field(key) {
//                missing_fields.insert(key.clone());
//            }
//        }
//    }
//
//    // Output the results
//    println!("Fields to consider as Optional: {:?}", optional_fields);
//    println!("Missing fields in the Rust model: {:?}", missing_fields);
//
//    create_table!();
    Ok(())
}
