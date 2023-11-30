mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use crate::domain::Error;
use crate::domain::models::Word;
use crate::domain::use_cases::DictionaryRepository;

pub struct DieselDictionaryRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselDictionaryRepository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to crate database pool");

        Self { pool }
    }
}

#[async_trait::async_trait]
impl DictionaryRepository for DieselDictionaryRepository {
    async fn bulk_insert(&mut self, items: Vec<Word>) -> Result<(), Error> {
        Ok(())
    }

    async fn find(&self, query: &str) -> Result<Option<Vec<Word>>, Error> {
        Ok(Some(vec![]))
    }
}