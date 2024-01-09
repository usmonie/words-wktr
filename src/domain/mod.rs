pub mod models;
pub mod import_dictionary;
pub mod import_words;

pub mod use_cases {
    use std::sync::Arc;
    use async_trait::async_trait;
    use tokio::sync::Mutex;
    use crate::domain::{models::Word, Error};

    #[async_trait]
    pub trait DictionaryRepository: Send + Sync + 'static {
        async fn bulk_insert(&mut self, items: Vec<Word>) -> Result<(), Error>;
        async fn find_exactly(&self, word: &str) -> Result<Option<Vec<Word>>, Error>;
        async fn find(&self, word: &str) -> Result<Option<Vec<Word>>, Error>;
        async fn random_word(&self, max_symbols: u32) -> Word;
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

        pub async fn execute(&self, max_symbols: u32) -> Word {
            let repository = self.repository.lock().await;

            let exact_result = repository.random_word(max_symbols).await;
            return exact_result
        }
    }

    pub struct WordOfTheDay<T: DictionaryRepository> {
        repository: Arc<Mutex<T>>,
    }

    impl<'a, T: DictionaryRepository> WordOfTheDay<T> {
        pub fn new(repository: Arc<Mutex<T>>) -> Self {
            Self {
                repository,
            }
        }

        pub async fn execute(&self, max_symbols: u32) -> Word {
            let repository = self.repository.lock().await;

            let exact_result = repository.random_word(max_symbols).await;
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
