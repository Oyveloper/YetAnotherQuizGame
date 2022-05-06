table! {
    questions (id) {
        id -> Varchar,
        content -> Varchar,
        alternatives -> Array<Text>,
        answer_index -> Int4,
        quiz_id -> Varchar,
    }
}

table! {
    quizs (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
    }
}

joinable!(questions -> quizs (quiz_id));

allow_tables_to_appear_in_same_query!(
    questions,
    quizs,
);
