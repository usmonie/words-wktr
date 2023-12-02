use std::{path::Path, time::Instant};

use actix_web::{App, get, HttpServer, Responder, web};
use serde::Deserialize;

use crate::{domain::use_cases::{ImportJsonDictionary, SearchWords}, };
use crate::data::dictionary_repository::DieselDictionaryRepository;

#[get("/dictionary/search")]
async fn search_word(info: web::Query<Info>) -> impl Responder {
    let start = Instant::now();
    let dictionary_repo = DieselDictionaryRepository::new("postgres://admin:admin@localhost:5433/words");
    let search_words = SearchWords::new(&dictionary_repo);

    let words_future = search_words.execute(info.query.clone());

    let words_res = words_future.await.expect("");
    let words = match words_res {
        None => { vec![] }
        Some(val) => val
    };

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    web::Json(words)
}


#[derive(Deserialize)]
pub struct Info {
    query: String,
}

pub async fn launch_server() -> std::io::Result<()> {
    start_parsing().await;
    HttpServer::new(|| {
        App::new()
            .service(search_word)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
        .expect("Error starting server");
    Ok(())
}

/**
 * Parsing wiktionary json
 */
async fn start_parsing() {
    println!("parsing started");
    let file_path = Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/wiktionary.json");
    let mut dictionary_repo = DieselDictionaryRepository::new("postgres://admin:admin@localhost:5433/words");
    let mut importer = ImportJsonDictionary::new(&mut dictionary_repo, file_path);

    match importer.execute().await {
        Ok(_) => println!("Successfully parsed the JSON file."),
        Err(e) => eprintln!("Error {:?}", e),
    }
}
