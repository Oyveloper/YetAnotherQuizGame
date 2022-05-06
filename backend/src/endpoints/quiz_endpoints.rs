use crate::data::quiz_data::*;
use crate::model::api::request_types::QuestionInput;
use crate::model::api::request_types::QuizInput;
use crate::model::api::response_types::APIReponse;
use crate::model::api::response_types::QuizWithQuestionsResponse;
use crate::model::quiz::question::Question;
use crate::model::quiz::quiz::Quiz;
use warp::{reply::json, Rejection, Reply};

pub async fn list_quizes_handler() -> Result<impl Reply, Rejection> {
    let quizes = get_quizes();

    Ok(json(&APIReponse::<Vec<Quiz>> {
        status: "OK".to_string(),
        message: "".to_string(),
        data: Some(quizes),
    }))
}

pub async fn add_quiz_handler(quiz_input: QuizInput) -> Result<impl Reply, Rejection> {
    let quiz = create_quiz(quiz_input);

    match quiz {
        Ok(q) => Ok(json(&APIReponse::<Quiz> {
            status: "Quiz created".to_string(),
            message: "".to_string(),
            data: Some(q),
        })),
        Err(e) => Ok(json(&APIReponse::<Quiz> {
            status: "ERROR".to_string(),
            message: e.to_string(),
            data: None,
        })),
    }
}

pub async fn add_question_handler(
    quiz_id: String,
    question: QuestionInput,
) -> Result<impl Reply, Rejection> {
    let question = create_question(question, quiz_id);

    match question {
        Ok(q) => Ok(json(&APIReponse::<Question> {
            status: "Question created".to_string(),
            message: "".to_string(),
            data: Some(q),
        })),
        Err(e) => Ok(json(&APIReponse::<()> {
            status: "ERROR".to_string(),
            message: e.to_string(),
            data: None,
        })),
    }
}

pub async fn single_quiz_handler(quiz_id: String) -> Result<impl Reply, Rejection> {
    let quiz_response = find_quiz(quiz_id);

    match quiz_response {
        Ok((q, qs)) => Ok(json(&APIReponse::<QuizWithQuestionsResponse> {
            status: "Quiz found".to_string(),
            message: "".to_string(),
            data: Some(QuizWithQuestionsResponse {
                quiz: q,
                questions: qs,
            }),
        })),
        Err(_) => Ok(json(&APIReponse::<Quiz> {
            status: "ERROR".to_string(),
            message: "Quiz not found".to_string(),
            data: None,
        })),
    }
}

pub async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(json(&APIReponse::<()> {
        status: "OK".to_string(),
        message: "Healthy".to_string(),
        data: None,
    }))
}
