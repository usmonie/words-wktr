mod data;
mod schema;
mod domain;
mod api;
mod mongodb;

use std::io;
use crate::mongodb::{find};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // find().await;
    // store().await;
    let _ = api::launch_server().await;

    Ok(())
}
