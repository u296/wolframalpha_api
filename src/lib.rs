//! Simplistic API accessor for wolframalpha. Currently only supports questions and image answer

use bytes::Bytes;
use image::DynamicImage;
use std::error::Error;
use std::fmt::{self, Write};

fn encode_char(c: char) -> bool {
    if c.is_ascii() {
        if c.is_alphanumeric() {
            false
        } else if "-_.~".contains(c) {
            false
        } else {
            true
        }
    } else {
        true
    }
}

fn encode_question(s: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut res = String::with_capacity(s.len());

    for c in s.chars() {
        if !encode_char(c) {
            res.push(c);
        } else if c == ' ' {
            res.push('+');
        } else {
            let mut buf = [0; 4];
            let n = c.encode_utf8(&mut buf).len();

            let mut tmp = String::with_capacity(3 * n);

            for i in 0..n {
                write!(tmp, "%{:02x}", buf[i])?;
            }

            res.push_str(&tmp);
        }
    }

    Ok(res)
}

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
        return Ok(Err(WolframalphaError::InvalidQuestion))
    }
    let encoded_query = encode_question(question)?;

    let response = reqwest::get(format!(
        "http://api.wolframalpha.com/v1/simple?appid={}&i={}",
        app_id, encoded_query
    ))
    .await?;

    if response.status() == reqwest::StatusCode::NOT_IMPLEMENTED {
        return Ok(Err(WolframalphaError::InvalidQuestion))
    }

    let img = image::load_from_memory(&response.bytes().await?)?;

    Ok(Ok(img))
}

/// Does the same thing as `api_retrieve_image` but instead of retrieving
/// the image it just gives you the raw bytes of the image instead
pub async fn api_retrieve_bytes(
    app_id: &str,
    question: &str,
) -> Result<Result<Bytes, WolframalphaError>, Box<dyn Error + Send + Sync>> {
    if question.trim() == "" {
        return Ok(Err(WolframalphaError::InvalidQuestion))
    }

    let encoded_query = encode_question(question)?;

    let response = reqwest::get(format!(
        "http://api.wolframalpha.com/v1/simple?appid={}&i={}",
        app_id, encoded_query
    ))
    .await?;

    if response.status() == reqwest::StatusCode::NOT_IMPLEMENTED {
        return Ok(Err(WolframalphaError::InvalidQuestion))
    }

    Ok(Ok(response.bytes()
    .await?))
}

#[derive(Debug, Clone, Copy)]
pub enum WolframalphaError {
    InvalidQuestion
}

impl fmt::Display for WolframalphaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::InvalidQuestion => "invalid question"
        })
    }
}

/// Errors specific to wolframalpha
impl Error for WolframalphaError {

}