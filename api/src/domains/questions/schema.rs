use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuestionDto {
    #[validate(length(min = 1, max = 255))]
    pub statement: String,

    pub answer: bool,

    #[validate(length(min = 1, max = 255))]
    pub explanation: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct IncrementAnswerCountDto {
    pub is_correct: bool,
}
