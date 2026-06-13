#[macro_use]
extern crate rocket;

extern crate pretty_env_logger;
#[macro_use] extern crate log;


// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions

mod cors;
mod handlers;
mod models;

use std::collections::HashMap;
use std::string::String;
use rocket::futures::{StreamExt, TryStreamExt};
use sqlx::postgres::PgPoolOptions;
use cors::*;
use handlers::*;


#[launch]
async fn rocket() -> _ {
    // TODO: Initialize pretty_env_logger
    pretty_env_logger::init();

    // TODO: Initialize dotenv
    let env_vars: HashMap<String, String> = dotenvy::vars().collect();


    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // Use dotenv to get the database url. 
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated. 
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    // let pool = todo!();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(env_vars.get("DATABASE_URL").expect("No DATABASE_URL env value"))
        .await
        .expect("Cannot create a Pg pool");

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.  
    // let recs = todo!();
    let recs = sqlx::query!("select * from questions").fetch_all(&pool).await.unwrap();

    info!("********* Question Records *********");
    // TODO: Log recs with debug formatting using the info! macro
    info!("{:?}", recs);

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}
