use crate::model::quiz::question::Question;
use crate::model::quiz::quiz::Quiz;
use serde::Serialize;

#[derive(Serialize)]
pub struct APIReponse<T: Serialize = ()> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub quiz_id: String,
}

#[derive(Serialize)]
pub struct QuizWithQuestionsResponse {
    pub quiz: Quiz,
    pub questions: Vec<Question>,
}
