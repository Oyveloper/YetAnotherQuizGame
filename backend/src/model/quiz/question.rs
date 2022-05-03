use serde::Serialize;

#[derive(Serialize)]
pub struct Question {
    pub id: i32,
    pub text: String,
    pub alternatives: Vec<String>,
    pub answer_index: i32,
}
