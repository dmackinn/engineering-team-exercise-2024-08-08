use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TriviaResponse {
    pub results: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub category: String,
    #[serde(rename = "type")]
    pub question_type: String,
    pub difficulty: String,
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
}