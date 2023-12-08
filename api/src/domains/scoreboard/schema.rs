use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateScoreDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,

    pub longest_streak: i32,
}
