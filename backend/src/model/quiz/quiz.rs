use crate::model::api::request_types::QuizInput;
use crate::schema::quizs;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name = "quizs"]
pub struct Quiz {
    id: String,
    name: String,
    description: String,
    // questions: Vec<Question>,
}

impl Quiz {
    pub fn new(name: &str, description: &str) -> Quiz {
        let id = Uuid::new_v4().to_string();
        Quiz {
            id: id,
            name: name.to_owned(),
            description: description.to_owned(),
            // questions: vec![],
        }
    }

    pub fn from(input: QuizInput) -> Self {
        Quiz::new(&input.name, &input.description)
    }
}
