use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

use super::{message::Message, Discord};

#[derive(Deserialize)]
pub struct MessageSendResponse {
    id: String
}

pub struct Channel<'a> {
    pub context: &'a Discord,
    pub id: String
}

impl<'a> Channel<'_> {
    pub fn api(&self, endpoint: &str) -> String {
        Discord::api(&format!("channels/{}/{endpoint}", self.id))
    }

    pub fn message(&self, id: &str) -> Message {
        Message { context: &self, id: String::from(id) }
    }

    pub async fn send_message(&self, content: &str) -> Result<Message> {
        let url = self.api("messages");

        let resp = self.context.client.post(url)
            .body(json!({
                "content": content,
                "flags": 0,
                "mobile_network_type": "unknown",
                "nonce": "",
                "tts": false
            }).to_string())
            .send().await?;
        Discord::handle_response(&resp)?;

        let data: MessageSendResponse = serde_json::from_str(&resp.text().await?)?;

        Ok(self.message(&data.id))
    }
}