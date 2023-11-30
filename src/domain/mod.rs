pub mod models;
mod import_json_use_case;

pub mod use_cases {
    use std::io::BufReader;
    use std::fs::File;
    use std::path::Path;
    use std::sync::Arc;
    use async_trait::async_trait;
    use futures::{stream, StreamExt};
    use serde_json::Value;
    use tokio::sync::Mutex;
    use crate::domain::Error;
    use crate::domain::models::Word;

    #[async_trait]
    pub trait DictionaryRepository {
        async fn bulk_insert(&mut self, items: Vec<Word>) -> Result<(), Error>;
        async fn find(&self, word: &str) -> Result<Option<Vec<Word>>, Error>;
    }

    pub struct ImportJsonDictionary<'a, T: DictionaryRepository> {
        repository: Arc<Mutex<&'a mut T>>,
        file_path: &'a Path,
    }

    impl<'a, T: DictionaryRepository> ImportJsonDictionary<'a, T> {
        pub fn new(repository: &'a mut T, file_path: &'a Path) -> Self {
            Self {
                repository: Arc::new(Mutex::new(repository)),
                file_path,
            }
        }
        pub async fn execute(&mut self) -> Result<(), Error> {
            let file = File::open(self.file_path).expect("Failed to open a file.");
            let reader = BufReader::new(file);

            let mut items: Vec<Word> = Vec::new();

            let deserializer = serde_json::Deserializer::from_reader(reader).into_iter::<Word>();

            let mut count = 0;

            let repo = Arc::clone(&self.repository);
            // stream::iter(deserializer)
            //     .chunks(10_000)
            //     .for_each_concurrent(20, |words_chunk| {
            //         let mut repo = Arc::clone(&repo);
            //         // async move {
            //         //     match repo.lock().await.bulk_insert(words_chunk).await {
            //         //         Ok(_) => println!("INSERTED"),
            //         //         Err(e) => eprintln!("Failed to insert chunks: {:?}", e),
            //         //     }
            //         // }
            //         count += 10_000;
            //         println!("INSERTED {}", count);
            //
            //         async move {}
            //     })
            //     .await;
            for entry in deserializer {
                let entry = entry.expect("Invalid JSON");
                items.push(entry);

                if items.len() == 1000 {
                    // self.repository.bulk_insert(std::mem::take(&mut items))?;
                    count += 1;
                    println!("INSERTED {}", count * 1000);
                    items.clear()
                }
            }

            if !items.is_empty() {
                // self.repository.bulk_insert(items)?;
            }

            Ok(())
        }
    }

    pub struct SearchWords<'a, T: DictionaryRepository> {
        repository: &'a T,
    }

    impl<'a, T: DictionaryRepository> SearchWords<'a, T> {
        pub fn new(repository: &'a T) -> Self {
            Self {
                repository,
            }
        }

        pub async fn execute(&self, word: String) -> Result<Option<Vec<Word>>, Error> {
            self.repository.find(&word).await
        }
    }
}


#[derive(Debug)]
pub enum Error {
    NotFound,
    InvalidInput,
    DatabaseError,
}


impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::InvalidInput
    }
}
