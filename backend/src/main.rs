#[macro_use]
extern crate diesel;
extern crate dotenv;

use warp::Filter;

use dotenv::dotenv;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod data;
pub mod db;
pub mod endpoints;
pub mod model;
pub mod schema;
pub mod shared_state;

use endpoints::quiz_endpoints::*;

use shared_state::client::Client;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let client_list: Arc<Mutex<HashMap<String, Client>>> = Arc::new(Mutex::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(health_handler);

    let list_route = warp::path!("quiz" / "all").and_then(list_quizes_handler);

    let single_quiz = warp::path!("quiz" / String)
        .and(warp::get())
        .and_then(single_quiz_handler);

    let add_question = warp::path!("quiz" / String / "add")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add_question_handler);

    let add_route = warp::post()
        .and(warp::path!("quiz"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(add_quiz_handler);

    let routes = health_route
        .or(list_route)
        .or(add_route)
        .or(single_quiz)
        .or(add_question)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
