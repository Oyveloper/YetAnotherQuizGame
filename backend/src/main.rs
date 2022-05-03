#[macro_use]
extern crate dotenv;

use warp::Filter;

use warp::{http::StatusCode, reject, reply::json, Rejection, Reply};

use dotenv::dotenv;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub mod model;
pub mod shared_state;

use model::api::response_types::APIReponse;
use model::quiz::Quiz;
use shared_state::client::Client;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut client_list: Arc<Mutex<HashMap<String, Client>>> = Arc::new(Mutex::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(health_handler);

    let routes = health_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(json(&APIReponse::<()> {
        status: "OK".to_string(),
        message: "Healthy".to_string(),
        data: None,
    }))
}

async fn list_quizes_handler() -> Result<impl Reply, Rejection> {
    Ok(json(&APIReponse::<Quiz> {
        status: "OK".to_string(),
        message: "Healthy".to_string(),
        data: Some(Quiz {
            id: 1,
            name: "Quiz 1".to_string(),
            description: "This is a quiz".to_string(),
            questions: vec![],
        }),
    }))
}
