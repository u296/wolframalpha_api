//! Simplistic API accessor for wolframalpha. Currently only supports questions and image answer

use image::DynamicImage;
use std::error::Error;
use std::fmt::Write;

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
            let mut buf = [0;4];
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
pub async fn api_retrieve(app_id: &str, question: &str) -> Result<DynamicImage, Box<dyn Error + Send + Sync>> {
    
    let encoded_query = encode_question(question)?;

    let response = reqwest::get(format!(
        "http://api.wolframalpha.com/v1/simple?appid={}&i={}",
        app_id, encoded_query
    ))
    .await?;

    image::load_from_memory(&response.bytes().await?).map_err(|e| e.into())
}
