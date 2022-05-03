use crate::model::quiz::question::Question;
use serde::Serialize;

#[derive(Serialize)]
pub struct Quiz {
    id: i32,
    name: String,
    description: String,
    questions: Vec<Question>,
}

impl Quiz {
    pub fn new(id: i32, name: String, description: String) -> Quiz {
        Quiz {
            id: id,
            name: name,
            description: description,
            questions: vec![],
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_questions(&self) -> &Vec<Question> {
        &self.questions
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }

    pub fn add_questions(&mut self, questions: Vec<Question>) {
        self.questions.extend(questions);
    }
}
