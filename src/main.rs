use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use anyhow::{Result, Ok};
use std::env;
use dotenv::dotenv;

mod constants;
mod config;
mod api;

use config::store::Store;
use config::routes::config_services;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let redis_url = env::var("REDIS_URL").expect("You need to set up the REDIS_URL env var");
    let psql_url = env::var("URI_PSQL_CONNECTION").expect("You need to set up the URI_PSQL_CONNECTION env var");
    let app_url = format!("{}:{}", &app_host, &app_port);

    let stores = Store::database_setup(psql_url, redis_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(stores.clone()))
            .wrap(Logger::new("%{r}a | Route: %r | Status: %s | Time: %T s"))
            .configure(config_services)
    })
    .bind(&app_url)?
    .run()
    .await?;

    Ok(())
}
