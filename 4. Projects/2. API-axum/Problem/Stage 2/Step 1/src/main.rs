// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions
use log::info;
use pretty_env_logger::env_logger;
use dotenvy::dotenv;

use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;

use handlers::*;

#[tokio::main]
async fn main() {
    // TODO: Initialize pretty_env_logger
    env_logger::init();
    // TODO: Initialize dotenv
    dotenv().ok();

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // Use dotenv to get the database url.
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated.
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    // let pool = todo!();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.
    // let recs = todo!();
    let recs = sqlx::query!("select * from questions").fetch_all(&pool).await.unwrap();
    info!("********* Question Records *********");
    // TODO: Log recs with debug formatting using the info! macro
    info!("Records: {:?}", recs);

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
