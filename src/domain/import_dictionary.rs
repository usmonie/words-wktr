use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use futures::future::join_all;
use tokio::sync::Mutex;
use crate::domain::Error;
use crate::domain::models::Word;
use crate::domain::use_cases::{DictionaryRepository};

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
        self.store().await;
        Ok(())
    }

    pub async fn store(&mut self) {
        let file = File::open(self.file_path).expect("Failed to open a file.");
        let reader = BufReader::new(file);

        let mut items: Vec<Word> = Vec::with_capacity(50_000);

        let deserializer = serde_json::Deserializer::from_reader(reader).into_iter::<Word>();

        let mut count = 0;

        let repo = Arc::clone(&self.repository);

        let mut repo = repo.lock().await;
        for entry in deserializer {
            let entry = entry.expect("Invalid JSON");

            if entry.word == "abortion" {
                println!("abortion: {:?}", entry)
            }
            items.push(entry);

            if items.len() > 50_000 {
                println!("INSERT START");

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
    }

    pub async fn store_parallel(&mut self) {
        let file = File::open(self.file_path).expect("Failed to open a file.");
        let reader = BufReader::new(file);

        let mut items: Vec<Word> = Vec::with_capacity(20_000);

        let deserializer = serde_json::Deserializer::from_reader(reader).into_iter::<Word>();

        let mut count = 0;

        let repo = Arc::clone(&self.repository);

        let mut handles = vec![];
        for entry in deserializer {
            let entry = entry.expect("Invalid JSON");
            items.push(entry);

            count += 1;

            if count % 20_000 == 0 {
                println!("PARSED {}", &count);
            }
            let mut items_clone = items.clone();
            if items_clone.len() == 20_000 {
                let repo = Arc::clone(&repo);
                let items = std::mem::take(&mut items_clone);
                let handle = tokio::task::spawn(async move {
                    let mut repo = repo.lock().await;
                    let count = items.len();
                    println!("INSERT STARTED");
                    repo.bulk_insert(items).await.unwrap();
                    println!("INSERTED {}", count);
                    count
                });
                handles.push(handle);
            }
        }

        if !&items.is_empty() {
            let repo = Arc::clone(&repo);
            let handle = tokio::task::spawn(async move {
                let mut repo = repo.lock().await;
                let count = items.len();
                println!("INSERTED {}", count);
                repo.bulk_insert(items).await.unwrap();
                count
            });
            handles.push(handle);
        }

        join_all(handles).await;
        // for handle in handles {
        //     let mut count = handle.await.unwrap();
        //     count += items.len();
        //     println!("INSERTED {}", count);
        // }
    }
}