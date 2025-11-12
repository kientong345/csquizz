use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmittedAnswerDto {
    pub question_id: i32,
    pub question_form: String, // "single-choice" || "multiple-choice" || "text-entry"
    pub single_choice: Option<i32>,
    pub multiple_choices: Option<Vec<i32>>,
    pub entry: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmittedQuizDto {
    pub user_id: Option<i32>,
    pub quiz_id: i32,
    pub answers: Vec<SubmittedAnswerDto>,
}
