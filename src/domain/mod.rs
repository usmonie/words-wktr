pub mod models;

pub mod use_cases {
    use std::io::BufReader;
    use std::fs::File;
    use std::path::Path;
    use std::sync::Arc;
    use async_trait::async_trait;
    use tokio::sync::Mutex;
    use crate::domain::{models::Word, Error};

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

            let mut repo = repo.lock().await;
            for entry in deserializer {
                let entry = entry.expect("Invalid JSON");
                items.push(entry);

                if items.len() == 100_000 {
                    count += items.len();
                    repo.bulk_insert(std::mem::take(&mut items)).await.unwrap();
                    println!("INSERTED {}", count);
                    items.clear()
                }
            }

            if !items.is_empty() {
                count += items.len();
                repo.bulk_insert(std::mem::take(&mut items)).await.unwrap();
                println!("INSERTED {}", count);
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
