use reqwest::blocking::Client;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn update_slack_status(token: &str, file_name: &str) -> Result<(), JsValue> {
    let client = Client::new();
    let url = "https://slack.com/api/users.profile.set";

    let profile = Profile {
        status_text: format!("Working on {}", file_name),
        status_emoji: ":computer:".to_string(),
    };

    let body = SlackStatus {
        profile,
    };

    let res = client.post(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(JsValue::from_str("Failed to update Slack status"))
    }
}

#[derive(Serialize)]
struct SlackStatus {
    profile: Profile,
}

#[derive(Serialize)]
struct Profile {
    status_text: String,
    status_emoji: String,
}
