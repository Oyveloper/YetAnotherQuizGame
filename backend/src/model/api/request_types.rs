use serde::{Deserialize, Serialize};
pub struct ConnectRequest {
    pub username: String,
    pub game_id: String,
}

pub struct DisConnectRequest {
    pub username: String,
    pub game_id: String,
}

pub struct CreateGameRequest {
    pub username: String,
    pub quiz_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionInput {
    pub content: String,
    pub alternatives: Vec<String>,
    pub answer_index: i32,
}

#[derive(Serialize, Deserialize)]
pub struct QuizInput {
    pub name: String,
    pub description: String,
}
