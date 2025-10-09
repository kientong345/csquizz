use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Answer {
    pub question_id: i32,
    pub option_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Submission {
    pub user_id: i32,
    pub answers: Vec<Answer>,
}
