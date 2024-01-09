use std::{path::Path, time::Instant};
use std::sync::Arc;

use actix_web::{App, error, get, HttpResponse, HttpServer, Responder, web};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::domain::use_cases::{SearchWords};
use crate::data::mongo_dictionary_repository::MongoDictionaryRepository;
use crate::domain::import_words::ImportWordsDictionary;
use crate::domain::use_cases::RandomWord;

#[get("/dictionary/search")]
async fn search_words(data: web::Data<AppState>, info: web::Query<QueryParams>) -> impl Responder {
    let start = Instant::now();
    let search_words_use_case = &data.search_words;

    let words = search_words_use_case.execute(info.query.clone()).await.expect("");

    let words = match words {
        None => { vec![] }
        Some(val) => val
    };
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    let result = web::Json(words);
    println!("{:?}", &result);
    return result;
}

#[get("/dictionary/random_word")]
async fn random_word(data: web::Data<AppState>, info: web::Query<RandomParams>) -> impl Responder {
    println!("random_word");
    let start = Instant::now();
    let random_word_use_case = &data.random_word;

    let word = random_word_use_case.execute(info.max_symbols).await;

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    web::Json(vec![word])
}

#[derive(Deserialize)]
pub struct QueryParams {
    query: String,
}

#[derive(Deserialize)]
pub struct RandomParams {
    max_symbols: u32,
}

struct AppState {
    search_words: SearchWords<MongoDictionaryRepository>,
    random_word: RandomWord<MongoDictionaryRepository>,
}

pub async fn launch_server() -> std::io::Result<()> {
    // find_words().await;
    println!("launch server");
    let dictionary = Arc::new(Mutex::new(MongoDictionaryRepository::new("mongodb://localhost:27017").await));
    HttpServer::new(
        move || {
            let dictionary_repo = dictionary.clone();
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
        }
    )
        .bind("127.0.0.1:8000")?
        .run()
        .await
        .expect("Error starting server");
    println!("start server");
    Ok(())
}

// /**
//  *
//  * Parsing wiktionary json
//  *
//  **/
// async fn start_parsing() {
//     println!("parsing started");
//     let file_paths = vec![
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.aa"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ab"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ac"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ad"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ae"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.af"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ag"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ah"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ai"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.aj"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ak"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.al"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.am"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.an"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ao"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ap"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.aq"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ar"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.as"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.at"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.au"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.av"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.aw"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ax"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.ay"),
//         Path::new("/Users/usmanakhmedov/IdeaProjects/words-wktr/file.az"),
//     ];
//
//     // for file_path in file_paths {
//     //     thread::spawn(move || async move {
//     //          tokio::spawn(async move {
//     //             println!("path {:?}", &file_path);
//     //             let dictionary_repo = DieselDictionaryRepository::new("postgres://postgres:admin@localhost:5433/word");
//     //             let mut importer = ImportJsonDictionary::new(dictionary_repo, file_path);
//     //
//     //             match importer.execute().await {
//     //                 Ok(_) => println!("Successfully parsed the JSON file."),
//     //                 Err(e) => eprintln!("Error {:?}", e),
//     //             }
//     //         }).await
//     //     });
//     // }
//     let join_handles = file_paths
//         .into_iter()
//         .map(|file_path| {
//             tokio::spawn(async move {
//                 println!("path {:?}", &file_path);
//                 let dictionary_repo = DieselDictionaryRepository::new("postgres://postgres:admin@localhost:5433/word");
//                 let mut importer = ImportJsonDictionary::new(dictionary_repo, file_path);
//
//                 match importer.execute().await {
//                     Ok(_) => println!("Successfully parsed the JSON file."),
//                     Err(e) => eprintln!("Error {:?}", e),
//                 }
//             })
//         })
//         .collect::<Vec<_>>();
//
//     join_all(join_handles).await;
// }

/**
 *
 * Parsing wiktionary json
 *
 **/
async fn find_words() {
    println!("find words started");
    let file_path = Path::new("/Users/usmanakhmedov/Downloads/Oxford 5000.txt");
    let dictionary_repo = MongoDictionaryRepository::new("mongodb://localhost:27017").await;
    let mut importer = ImportWordsDictionary::new(dictionary_repo, file_path);

    match importer.execute().await {
        Ok(_) => println!("Successfully parsed the TXT file."),
        Err(e) => eprintln!("Error {:?}", e),
    }
}
