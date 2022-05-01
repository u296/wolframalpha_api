use super::prelude::*;
use super::*;
use image::DynamicImage;

/// Performs a simple api request to wolframalpha, returning you the image
///
/// # Arguments
///
/// * `app_id` - The AppID of your wolframalpha application
/// * `question` - The plaintext question you want to ask wolframalpha
pub async fn api_retrieve_image(
    app_id: &str,
    question: &str,
) -> Result<Result<DynamicImage, WolframalphaError>, Box<dyn Error + Send + Sync>> {
    if question.trim() == "" {
        return Ok(Err(WolframalphaError::InvalidQuestion));
    }
    let encoded_query = encoding::encode_question(question)?;

    let response = reqwest::get(format!(
        "http://api.wolframalpha.com/v1/simple?appid={}&i={}",
        app_id, encoded_query
    ))
    .await?;

    if response.status() == reqwest::StatusCode::NOT_IMPLEMENTED {
        return Ok(Err(WolframalphaError::InvalidQuestion));
    }

    let img = image::load_from_memory(&response.bytes().await?)?;

    Ok(Ok(img))
}
