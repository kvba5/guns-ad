use anyhow::{Ok, Result};
use reqwest::{header::HeaderValue, StatusCode};
use serde::Deserialize;
use serde_json::json;

use crate::helper::cache::Cache;

use super::{message::Message, Discord};

#[derive(Deserialize, Clone)]
pub struct ChannelData {
    pub name: String,
    #[serde(alias = "type")]
    pub channel_type: u8,
    pub rate_limit_per_user: u32
}

pub struct MessageResponseData<'a> {
    pub msg: Option<Message<'a>>,
    pub slowmode: u32
}

pub struct Channel<'a> {
    pub context: &'a Discord,
    pub id: String,
    data: Option<Cache<ChannelData>>
}

impl<'a> Channel<'a> { 
    pub fn from(context: &'a Discord, id: String) -> Self {
        Self {
            context,
            id,
            data: None
        }
    }

    pub fn api(&self, endpoint: &str) -> String {
        Discord::api(&format!(
            "channels/{}{}",
            self.id,
            format!(
                "{}{}",
                if endpoint.len() > 0 { "/" } else { "" },
                endpoint
            )
        ))
    }

    pub async fn get(&mut self) -> Result<ChannelData> {
        if let Some(cached_channel) = &self.data {
            if cached_channel.validate() {
                return Ok(cached_channel.data.clone())
            }
        }

        let url = self.api("");
        let resp = self.context.client.get(url).send().await?;
        Discord::handle_response(&resp)?;

        let json = resp.json::<ChannelData>().await?;
        self.data = Some(
            Cache::new(json.clone(), None)
        );

        Ok(json)
    }

    pub fn message(&self, id: &str) -> Message {
        Message { context: &self, id: String::from(id) }
    }

    pub async fn send_message(&self, content: &str) -> Result<MessageResponseData> {
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
        
        

        if resp.status() == StatusCode::TOO_MANY_REQUESTS {
            return Ok(MessageResponseData {
                msg: None,
                slowmode: resp.headers().get("retry-after").unwrap_or(&HeaderValue::from_static("60"))
                            .to_str()?
                            .parse::<u32>().unwrap_or(60)
            })
        } else { Discord::handle_response(&resp)? }

        #[derive(Deserialize)]
        struct MessageSendResponse {
            id: String
        }

        let data = resp.json::<MessageSendResponse>().await?;


        Ok(MessageResponseData {
            msg: Some(self.message(&data.id)),
            slowmode: 0
        })
    }
}