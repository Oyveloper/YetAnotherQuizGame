use crate::model::api::request_types::QuestionInput;
use crate::model::quiz::quiz::Quiz;
use crate::schema::questions;
use serde::Serialize;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Queryable, Associations, Insertable)]
#[belongs_to(Quiz, foreign_key = "quiz_id")]
#[table_name = "questions"]
pub struct Question {
    pub id: String,
    pub content: String,
    pub alternatives: Vec<String>,
    pub answer_index: i32,
    pub quiz_id: String,
}

impl Question {
    pub fn from(input: QuestionInput, quiz_id: String) -> Self {
        Question {
            id: Uuid::new_v4().to_string(),
            content: input.content,
            alternatives: input.alternatives,
            answer_index: input.answer_index,
            quiz_id: quiz_id,
        }
    }
}
