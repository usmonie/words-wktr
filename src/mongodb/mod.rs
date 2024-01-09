use std::fs::File;
use std::io::BufReader;
use actix_web::web;
use bson::doc;
use diesel::QueryDsl;
use futures::TryStreamExt;
use mongodb::{Client, Cursor, Database};
use mongodb::options::{ClientOptions, ResolverConfig};
use crate::domain::models::Word;

pub async fn store_words(mongo: Database, words: Vec<Word>) {
    mongo.collection("words").insert_many(words, None).await.unwrap();
}

pub async fn store() {
    let file = File::open("/Users/usmanakhmedov/Downloads/kaikki.org-dictionary-English.json")
        .expect("Failed to open a file.");
    let reader = BufReader::new(file);

    let mut items: Vec<Word> = Vec::with_capacity(10_000);

    let deserializer = serde_json::Deserializer::from_reader(reader).into_iter::<Word>();

    let mut count = 0;
    let options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(options).unwrap();

    for entry in deserializer {
        let entry = entry.expect("Invalid JSON");

        items.push(entry);

        if items.len() > 50_000 {
            println!("INSERT START");

            count += items.len();
            store_words(client.database("words"), std::mem::take(&mut items)).await;
            println!("INSERTED {}", count);
            items.clear()
        }
    }
}

pub async fn find() {
    let options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(options).unwrap();

    let mut words: Cursor<Word> = client.database("words")
        .collection("words")
        .find(
            doc! {
             "word": "word"
            },
            None
        )
        .await
        .expect("");
    
    let mut items = vec![];
    while let Some(word) = words.try_next().await.expect("") {
        items.push(word)
    }

    println!("found word: {:?}", web::Json(&items));
}