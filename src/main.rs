mod data;
mod schema;
mod domain;
mod api;
mod mongodb;

use std::io;
use crate::mongodb::{index, store};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // store().await;
    // index().await;

    let _ = api::launch_server().await;

    Ok(())
}
