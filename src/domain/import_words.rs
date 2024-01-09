use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::Error;
use crate::domain::use_cases::{DictionaryRepository};

pub struct ImportWordsDictionary<'a, T: DictionaryRepository> {
    repository: Arc<Mutex<T>>,
    file_path: &'a Path,
}

impl<'a, T: DictionaryRepository> ImportWordsDictionary<'a, T> {
    pub fn new(repository: T, file_path: &'a Path) -> Self {
        Self {
            repository: Arc::new(Mutex::new(repository)),
            file_path,
        }
    }
    pub async fn execute(&mut self) -> Result<(), Error> {
        let file = File::open(self.file_path).unwrap();
        let reader = BufReader::new(file);

        let repo = Arc::clone(&self.repository);

        let mut repo = repo.lock().await;
        let mut count_empty_translations = 0;
        let mut russian_doesnt = 0;
        for line in reader.lines() {
            let word = line.unwrap();

            // Вызовите функцию для слова
            let words = repo.find_exactly(&word).await.unwrap().unwrap();

            let mut has_translation = false;
            for word in &words {
                if !has_translation {
                    has_translation = word.translations.iter().position(|t| t.code == Some("ru".to_string())).is_some();
                }
                // if word.translations.len() == 0 {
                //     count_empty_translations += 1;
                    russian_doesnt += 1;
                //     println!("translations is empty for {}, count of word exist: {}", word.word, &words.len())
                // } else {
                //     if word.translations.iter().position(|t| t.code == Some("ru".to_string())).is_none() {
                //         russian_doesnt += 1;
                //         println!("russian translation is not exist for {}, count of word exist: {}", word.word, &words.len())
                //     }
                // }
            }
            if !has_translation {
                println!("{}", word)
            }
        }

        println!("empty: {}, russian doesnt exist: {}", count_empty_translations, russian_doesnt);

        Ok(())
    }
}