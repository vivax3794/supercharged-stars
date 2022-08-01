//! Lets talk to matisse shall we?
//! I.E the fun part!

use serde::Serialize;
use tauri::api::http;

use crate::{ResultExtended, Star};

static BASE_URL: &str = "https://arcade-placement-tool.herokuapp.com";

#[derive(Serialize)]
struct ToStarFieldData {
    jwt: String,
    stars: Vec<Star>,
}

#[tauri::command(async)]
pub async fn send_stars(jwt: String, stars: Vec<Star>) -> Result<(), String> {
    let url = format!("{}/send/toStarField", BASE_URL);

    let stars = stars
        .into_iter()
        .map(|Star { x, y, color }| Star {
            x: x / 1000.0,
            y: y / 500.0,
            color,
        })
        .collect();

    let client = http::ClientBuilder::new()
        .build()
        .prefix_error("Error creating http client")?;
    let request = http::HttpRequestBuilder::new("POST", url)
        .unwrap()
        .body(http::Body::Json(
            serde_json::to_value(ToStarFieldData { jwt, stars })
                .prefix_error("Error creating json")?,
        ))
        .response_type(http::ResponseType::Text);

    let response = client
        .send(request)
        .await
        .prefix_error("Error sending request")?;

    let response = response
        .read()
        .await
        .prefix_error("Error reading response")?;

    if response.status != 200 {
        Err(response
            .data
            .as_str()
            .ok_or("Parsing response")?
            .to_string())
    } else {
        Ok(())
    }
}
