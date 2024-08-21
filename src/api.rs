use anyhow::Result;
use reqwest;
use crate::entities::TriviaResponse;

// TODO discuss use of anyhow
pub fn get_questions(category_id: u32) -> Result<TriviaResponse> {
    let url = format!(
        "https://opentdb.com/api.php?amount=5&category={}&difficulty=medium&type=multiple",
        category_id
    );
    let response = reqwest::blocking::get(&url)?.json::<TriviaResponse>()?;
    Ok(response)
}