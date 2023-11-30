mod data;
mod schema;
mod domain;
mod api;

use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let _ = api::launch_server().await;

    Ok(())
}
