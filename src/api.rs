use std::{path::Path, time::Instant};
use std::sync::Arc;

use actix_web::{App, error, get, HttpResponse, HttpServer, Responder, web};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{domain::use_cases::{ImportJsonDictionary, SearchWords}};
use crate::data::dictionary_repository::DieselDictionaryRepository;
use crate::domain::use_cases::RandomWord;

#[get("/dictionary/search")]
async fn search_words(data: web::Data<AppState>, info: web::Query<Info>) -> impl Responder {
    let start = Instant::now();
    let search_words_use_case = &data.search_words;

    let words = search_words_use_case.execute(info.query.clone()).await.expect("");

    let words = match words {
        None => { vec![] }
        Some(val) => val
    };
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    let result = /*web::Json(*/serde_json::to_string(&words).unwrap();
    println!("{:?}", &result);
    return result;
}

#[get("/dictionary/random_word")]
async  fn  random_word(data: web::Data<AppState>) -> impl Responder {
    let start = Instant::now();
    let random_word_use_case = &data.random_word;

    let word = random_word_use_case.execute().await;

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    web::Json(serde_json::to_string(&word).unwrap())
}

#[derive(Deserialize)]
pub struct Info {
    query: String,
}

struct AppState {
    search_words: SearchWords<DieselDictionaryRepository>,
    random_word: RandomWord<DieselDictionaryRepository>,
}

pub async fn launch_server() -> std::io::Result<()> {
    HttpServer::new(move || {

        let dictionary_repo = Arc::new(Mutex::new(DieselDictionaryRepository::new("postgres://admin:admin@localhost:5433/words")));
        let app_state = web::Data::new(AppState {
            search_words: SearchWords::new(dictionary_repo.clone()),
            random_word: RandomWord::new(dictionary_repo.clone()),
        });

        let json_cfg = web::JsonConfig::default()
            // limit request payload size
            .limit(4096)
            // only accept text/plain content type
            .content_type(|mime| mime == mime::TEXT_PLAIN)
            // use custom error handler
            .error_handler(|err, req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
            });

        App::new()
            .app_data(app_state.clone())
            .app_data(json_cfg)
            .service(search_words)
            .service(random_word)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
        .expect("Error starting server");
    Ok(())
}

/**
 *
 * Parsing wiktionary json
 *
 **/
async fn start_parsing() {
    println!("parsing started");
    let file_path = Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/wiktionary.json");
    let dictionary_repo = DieselDictionaryRepository::new("postgres://admin:admin@localhost:5433/words");
    let mut importer = ImportJsonDictionary::new(dictionary_repo, file_path);

    match importer.execute().await {
        Ok(_) => println!("Successfully parsed the JSON file."),
        Err(e) => eprintln!("Error {:?}", e),
    }
}
