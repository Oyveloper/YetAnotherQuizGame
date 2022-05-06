-- Your SQL goes here
CREATE TABLE IF NOT EXISTS quizs (
    id VARCHAR (255) PRIMARY KEY NOT NULL,
    name VARCHAR (255) NOT NULL,
    description VARCHAR (255) NOT NULL
);
CREATE TABLE IF NOT EXISTS questions (
    id VARCHAR (255) PRIMARY KEY NOT NULL,
    content VARCHAR (255) NOT NULL,
    alternatives text[] NOT NULL DEFAULT '{}',
    answer_index INTEGER NOT NULL DEFAULT 0,
    quiz_id VARCHAR (255) NOT NULL,
    CONSTRAINT fk_quiz_id FOREIGN KEY (quiz_id) REFERENCES quizs (id)
)