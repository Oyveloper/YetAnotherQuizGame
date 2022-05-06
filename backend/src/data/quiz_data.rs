use crate::db::establish_connection;
use crate::model::api::request_types::QuestionInput;
use crate::model::api::request_types::QuizInput;
use crate::model::quiz::question::Question;
use crate::model::quiz::quiz::Quiz;
use crate::schema::questions::dsl::*;
use crate::schema::quizs::dsl::*;
use diesel::prelude::*;

pub fn get_quizes() -> Vec<Quiz> {
    let connection = establish_connection();

    let quizes = quizs.load::<Quiz>(&connection);

    match quizes {
        Ok(q) => q,
        Err(_) => vec![],
    }
}

pub fn create_quiz(quiz_input: QuizInput) -> Result<Quiz, &'static str> {
    let connection = establish_connection();

    let new_quiz = Quiz::from(quiz_input);

    let result = diesel::insert_into(quizs)
        .values(&new_quiz)
        .execute(&connection);

    match result {
        Ok(_) => Ok(new_quiz),
        Err(_) => Err("Error creating quiz"),
    }
}

pub fn find_quiz(q_id: String) -> Result<(Quiz, Vec<Question>), &'static str> {
    let connection = establish_connection();

    let quiz = quizs.find(q_id).first::<Quiz>(&connection);

    match quiz {
        Ok(q) => {
            let qs = Question::belonging_to(&q).load::<Question>(&connection);

            match qs {
                Ok(qs) => Ok((q, qs)),
                Err(_) => Ok((q, vec![])),
            }
        }

        Err(_) => Err("Error finding quiz"),
    }
}

pub fn create_question(question: QuestionInput, q_id: String) -> Result<Question, &'static str> {
    let connection = establish_connection();

    let quiz = quizs.find(q_id.clone()).first::<Quiz>(&connection);

    if let Err(_) = quiz {
        return Err("Error finding quiz");
    }

    let new_question = Question::from(question, q_id);

    let result = diesel::insert_into(questions)
        .values(&new_question)
        .execute(&connection);

    match result {
        Ok(_) => Ok(new_question),
        Err(_) => Err("Could not create the question"),
    }
}
