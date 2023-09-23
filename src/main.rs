mod routes;
mod models;

use actix_cors::Cors;
use actix_web::{dev::Server, http::header, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use routes::config::config;
use routes::health_route::health_checker_handler;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let data_url: String = std::env::var("DATABASE_URL").expect("DATABASE URL must be set");
    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&data_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the db is successful");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database {:?}", err);
            std::process::exit(1);
        }
    };

    println!("Server started successfully!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(health_checker_handler)
            .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/*
    setup the db

    declare routes
    declare cors middleware

    start listening for requests in port x
*/
