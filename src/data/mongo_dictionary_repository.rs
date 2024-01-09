use crate::domain::use_cases::DictionaryRepository;
use crate::domain::Error;
use std::sync::Arc;
use bson::doc;
use futures::lock::Mutex;
use futures::{StreamExt, TryStreamExt};
use mongodb::{Client, Collection, Cursor, Database};
use mongodb::options::ClientOptions;
use crate::domain::models::Word;

pub struct MongoDictionaryRepository {
    database: Arc<Mutex<Database>>,
}

impl MongoDictionaryRepository {
    pub async fn new(database_url: &str) -> Self {
        let options = ClientOptions::parse(database_url).await.unwrap();
        let client = Client::with_options(options).unwrap();
        let database = client.database("words");
        return MongoDictionaryRepository {
            database: Arc::new(Mutex::new(database))
        };
    }
}

#[async_trait::async_trait]
impl DictionaryRepository for MongoDictionaryRepository {
    async fn bulk_insert(&mut self, items: Vec<crate::domain::models::Word>) -> Result<(), Error> {
        let database = self.database.clone();
        let database = database.lock().await;
        database.create_collection("words", None).await.unwrap();
        database.collection("words").insert_many(items, None).await.unwrap();

        Ok(())
    }

    async fn find_exactly(&self, word: &str) -> Result<Option<Vec<Word>>, Error> {
        let database = self.database.clone();
        let database = database.lock().await;

        let collection = database.collection("words");
        println!("{}", collection.count_documents(doc! {}, None).await.unwrap());

        let mut words: Cursor<Word> = collection
            .find(doc! { "word": word }, None)
            .await
            .expect("");

        let mut items: Vec<Word> = vec![];
        while let Some(word_result) = words.try_next().await.expect("") {
            items.push(word_result)
        }

        return Ok(Some(items));
    }

    async fn find(&self, word: &str) -> Result<Option<Vec<Word>>, Error> {
        let database = self.database.clone();
        let database = database.lock().await;
        let filter = doc! {"word": {"$regex": format!("^{}.*", word)}};
        let cursor = database
            .collection("words").find(filter, None).await.unwrap();
        let result: Vec<Word> = cursor.try_collect().await.unwrap();

        return Ok(Some(result));
    }

    async fn random_word(&self, max_symbols: u32) -> Word {
        let database = self.database.clone();
        let database = database.lock().await;
        let collection: Collection<Word> = database.collection("words");
        println!("{}", collection.count_documents(doc! {}, None).await.unwrap());
        // Query MongoDB for a random word with length less than or equal to the specified maximum symbols
        let reg = format!("^.{{{}, {}}}$", 5, max_symbols);
        let pipeline = vec![
            doc! {
                "$match": {
                  "$expr": {
                    "$and": [
                      { "$gt": [{ "$strLenCP": "$word" }, 5] },  // Filter by word size greater than 5
                      { "$lt": [{ "$strLenCP": "$word" }, max_symbols] },  // Filter by word size less than max_symbols
                      { "$gt": [{ "$size": "$senses" }, 2] }  // Filter by senses_array size greater than 2
                    ]
                  }
                },
            },
            doc! {
                "$sample": {
                    "size": 1
                }
            },
        ];

        let mut cursor = collection.aggregate(pipeline, None).await.unwrap();
        let result = cursor.try_next().await.unwrap().unwrap();
        let word: Word = bson::from_document(result).unwrap();
        return word;
    }
}
