use serde::Deserialize;

use crate::models::{question::post::PostQuestion, quiz::post::PostQuizInfo};

#[derive(Debug, Deserialize)]
pub enum AnswerType {
    ChoicesAnswer(Vec<i32>), // Vec<option_id>
    TextAnswer(String),
}

#[derive(Debug, Deserialize)]
pub struct SubmittedAnswer {
    pub question_id: i32,
    pub answer: AnswerType,
}

#[derive(Debug, Deserialize)]
pub struct Submission {
    pub user_id: i32,
    pub answers: Vec<SubmittedAnswer>,
}

#[derive(Debug, Deserialize)]
pub struct SubmissionResult {}

impl Submission {
    pub fn evaluate(&self) -> SubmissionResult {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
pub struct PostQuiz {
    pub info: PostQuizInfo,
    pub questions: Vec<PostQuestion>,
}
