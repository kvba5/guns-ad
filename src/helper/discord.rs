use anyhow::{anyhow, Ok, Result};
use channel::Channel;
use reqwest::{header::{HeaderMap, HeaderValue}, Client, ClientBuilder, Response, StatusCode};
use serde::Deserialize;

use super::cache::Cache;


const DEFAULT_PROPERTIES_VALUE: &str = "eyJvcyI6IldpbmRvd3MiLCJicm93c2VyIjoiQ2hyb21lIiwiZGV2aWNlIjoiIiwic3lzdGVtX2xvY2FsZSI6ImVuLVVTIiwiYnJvd3Nlcl91c2VyX2FnZW50IjoiTW96aWxsYS81LjAgKFdpbmRvd3MgTlQgMTAuMDsgV2luNjQ7IHg2NCkgQXBwbGVXZWJLaXQvNTM3LjM2IChLSFRNTCwgbGlrZSBHZWNrbykgQ2hyb21lLzEzMS4wLjAuMCBTYWZhcmkvNTM3LjM2IiwiYnJvd3Nlcl92ZXJzaW9uIjoiMTMxLjAuMC4wIiwib3NfdmVyc2lvbiI6IiIsInJlZmVycmVyIjoiaHR0cHM6Ly9kaXNjb3JkLmNvbS8iLCJyZWZlcnJpbmdfZG9tYWluIjoiZGlzY29yZC5jb20iLCJyZWZlcnJlcl9jdXJyZW50IjoiaHR0cHM6Ly9kaXNjb3JkLmFwcC8iLCJyZWZlcnJpbmdfZG9tYWluX2N1cnJlbnQiOiJkaXNjb3JkLmFwcCIsInJlbGVhc2VfY2hhbm5lbCI6InN0YWJsZSIsImNsaWVudF9ldmVudF9zb3VyY2UiOm51bGwsImhhc19jbGllbnRfbW9kcyI6ZmFsc2V9";
const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
const API_VERSION: u8 = 9;

#[derive(Deserialize, Clone)]
pub struct DiscordUser {
    pub global_name: String,
    pub username: String
}

pub struct Discord {
    client: Client,
    user: Option<Cache<DiscordUser>>
}

impl Discord {
    pub fn handle_response(resp: &Response) -> Result<()> {
        if resp.status() > StatusCode::BAD_REQUEST {
            match resp.status() {
                StatusCode::FORBIDDEN => return Err(anyhow!("Invalid token! Have you provided valid token in config?")),
                StatusCode::TOO_MANY_REQUESTS => return Err(anyhow!("You're sending requests too fast!")),
                _ => return Err(anyhow!("Something went wrong! Please report it to tool creator. ({})", resp.status()))
            }
        }

        Ok(())
    }

    pub fn api(endpoint: &str) -> String {
        format!("https://discord.com/api/v{API_VERSION}/{endpoint}")
    }

    pub fn new(token: &str) -> Result<Self> {
        let mut default_headers = HeaderMap::with_capacity(6);
        default_headers.insert("accept", HeaderValue::from_static("*/*"));
        default_headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.9"));
        default_headers.insert("content-type", HeaderValue::from_static("application/json"));
        default_headers.insert("authorization", HeaderValue::from_str(token)?);
        default_headers.insert("x-discord-locale", HeaderValue::from_static("en-US"));
        default_headers.insert("x-super-properties", HeaderValue::from_static(DEFAULT_PROPERTIES_VALUE));

        let client = ClientBuilder::default()
            .referer(true)
            .user_agent(DEFAULT_USER_AGENT)
            .default_headers(default_headers)
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .use_rustls_tls()
            .build()?;

        Ok(
            Self {
                client,
                user: None
            }
        )
    }

    pub fn channel(&self, id: &str) -> Channel {
        Channel::from(&self, String::from(id))
    }

    pub async fn get_current_user(&mut self) -> Result<DiscordUser> {
        if let Some(cached_user) = &self.user {
            if cached_user.validate() {
                return Ok(cached_user.data.clone())
            }
        }

        let resp = self.client.get(Self::api("users/@me")).send().await?;
        Self::handle_response(&resp)?;

        let json = resp.json::<DiscordUser>().await?;
        self.user = Some(
            Cache::new(json.clone(), None)
        );

        Ok(json)
    }
}

mod channel;
mod message;