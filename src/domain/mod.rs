pub mod models;

pub mod use_cases {
    use std::io::BufReader;
    use std::fs::File;
    use std::path::Path;
    use std::sync::Arc;
    use async_trait::async_trait;
    use futures::future::join_all;
    use tokio::sync::Mutex;
    use tokio::task;
    use crate::domain::{models::Word, Error};

    #[async_trait]
    pub trait DictionaryRepository: Send + Sync + 'static {
        async fn bulk_insert(&mut self, items: Vec<Word>) -> Result<(), Error>;
        async fn find_exactly(&self, word: &str) -> Result<Option<Vec<Word>>, Error>;
        async fn find(&self, word: &str) -> Result<Option<Vec<Word>>, Error>;
        async fn random_word(&self) -> Word;
    }

    pub struct ImportJsonDictionary<'a, T: DictionaryRepository> {
        repository: Arc<Mutex<T>>,
        file_path: &'a Path,
    }

    impl<'a, T: DictionaryRepository> ImportJsonDictionary<'a, T> {
        pub fn new(repository: T, file_path: &'a Path) -> Self {
            Self {
                repository: Arc::new(Mutex::new(repository)),
                file_path,
            }
        }
        pub async fn execute(&mut self) -> Result<(), Error> {
            let file = File::open(self.file_path).expect("Failed to open a file.");
            let reader = BufReader::new(file);

            let deserializer = serde_json::Deserializer::from_reader(reader).into_iter::<Word>();

            let mut bulk_insert_jobs = vec![];

            let mut items: Vec<Word> = Vec::with_capacity(10_000);

            for entry in deserializer {
                let entry = entry.expect("Invalid JSON");
                items.push(entry);

                if items.len() == 50_000 {
                    let repo = Arc::clone(&self.repository);
                    let items = std::mem::replace(&mut items, Vec::with_capacity(10_000));
                    let insert_job = task::spawn(async move {
                        let mut repo = repo.lock().await;
                        repo.bulk_insert(items).await.unwrap_or(());
                    });
                    bulk_insert_jobs.push(insert_job);
                }
            }

            if !items.is_empty() {
                let repo = Arc::clone(&self.repository);
                let insert_job = task::spawn(async move {
                    let mut repo = repo.lock().await;
                    repo.bulk_insert(items).await.unwrap_or(());
                });
                bulk_insert_jobs.push(insert_job);
            }

            join_all(bulk_insert_jobs).await;
            Ok(())
        }
    }

    pub struct SearchWords<T: DictionaryRepository> {
        repository: Arc<Mutex<T>>,
    }

    impl<T: DictionaryRepository> SearchWords<T> {
        pub fn new(repository: Arc<Mutex<T>>) -> Self {
            Self {
                repository,
            }
        }

        pub async fn execute(&self, word: String) -> Result<Option<Vec<Word>>, Error> {
            println!("started finding ");

            let repository = self.repository.lock().await;
            let exact_result = repository.find_exactly(&word).await;
            match exact_result {
                Ok(words) => {
                    match words {
                        None => { repository.find(&word).await }
                        Some(words) => {
                            Ok(Some(words))
                        }
                    }
                }
                Err(_) => {
                    repository.find(&word).await
                }
            }
        }
    }

    pub struct RandomWord<T: DictionaryRepository> {
        repository: Arc<Mutex<T>>,
    }

    impl<'a, T: DictionaryRepository> RandomWord<T> {
        pub fn new(repository: Arc<Mutex<T>>) -> Self {
            Self {
                repository,
            }
        }

        pub async fn execute(&self) -> Word {
            let repository = self.repository.lock().await;

            let exact_result = repository.random_word().await;
            return exact_result
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
