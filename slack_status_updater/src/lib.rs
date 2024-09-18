use js_sys::Promise;
use reqwest::Client;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub fn update_slack_status(token: String, file_name: String) -> Promise {
    future_to_promise(async move {
        let result = send_slack_status(&token, &file_name).await;
        match result {
            Ok(_) => Ok(JsValue::from_str("Slack status updated")),
            Err(e) => Err(JsValue::from_str(&format!(
                "Failed to update Slack status: {}",
                e
            ))),
        }
    })
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

async fn send_slack_status(token: &str, file_name: &str) -> Result<(), String> {
    let client = Client::new();
    let url = "https://slack.com/api/users.profile.set";

    let profile = Profile {
        status_text: format!("Working on {}", file_name),
        status_emoji: ":computer:".to_string(),
    };

    let body = SlackStatus { profile };

    let res = client
        .post(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(format!("Request failed with status code: {}", res.status()))
    }
}
