use crate::domain::use_cases::DictionaryRepository;
use crate::domain::Error;
use std::sync::Arc;
use bson::doc;
use futures::lock::Mutex;
use futures::TryStreamExt;
use mongodb::{Client, Cursor, Database, IndexModel};
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

    async fn find_exactly(&self, word: &str) -> Result<Option<Vec<crate::domain::models::Word>>, Error> {
        let database = self.database.clone();
        let database = database.lock().await;


        let mut words: Cursor<crate::domain::models::Word> = database
            .collection("words")
            .find(doc! { "word": word }, None)
            .await
            .expect("");

        let mut items: Vec<crate::domain::models::Word> = vec![];
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
        let collection = database.collection("words");
        println!("{}", collection.count_documents(doc! {}, None).await.unwrap());
        // Query MongoDB for a random word with length less than or equal to the specified maximum symbols
        let filter = doc! {"$where": format!("this.word.length <= {}", max_symbols)};
        let cursor = collection.find(filter, None).await.unwrap();
        let result: Vec<Word> = cursor.try_collect().await.unwrap();

        // Return a random word if there are matching words, otherwise return None
        let random_index = rand::random::<usize>() % result.len();
        result[random_index].clone()
    }
}
