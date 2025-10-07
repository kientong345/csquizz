use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Answer {
    question_id: i32,
    option_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Submission {
    user_id: i32,
    answers: Vec<Answer>,
}
