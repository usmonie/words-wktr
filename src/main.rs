use std::path::Path;
use std::time::Instant;

use data::DieselDictionaryRepository;
use crate::domain::use_cases::{ImportJsonDictionary, SearchWords};

mod data;
mod schema;
mod domain;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let dictionary_repo = DieselDictionaryRepository::new("postgres://admin:admin@localhost:5433/words");
    let search_words = SearchWords::new(&dictionary_repo);

    // Assuming `execute` method now returns a Future because of async.
    let words_future = search_words.execute(String::from("hello"));

    // You can now use .await in main function because it's async now.
    let words = words_future.await.expect("").unwrap();

    let duration = start.elapsed();
    for word in words {
        println!("{:?}", word);
    }

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
/**
    Parsing wiktionary json
 */
fn start_parsing() {
    let file_path = Path::new("/Users/usmanakhmedov/Downloads/wiktionary.json");
    let mut dictionary_repo = DieselDictionaryRepository::new("postgres://admin:admin@localhost:5433/words");
    let mut importer = ImportJsonDictionary::new(&mut dictionary_repo, file_path);

    match importer.execute() {
        Ok(_) => println!("Successfully parsed the JSON file."),
        Err(e) => eprintln!("Error"), // Replace this with your error handling
    }
}