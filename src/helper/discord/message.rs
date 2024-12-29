use anyhow::Result;

use super::{channel::Channel, Discord};

pub struct Message<'a> {
    pub context: &'a Channel<'a>,
    pub id: String
}

impl<'a> Message<'_> {
    pub fn api(&self, endpoint: &str) -> String {
        self.context.api(&format!("messages/{}/{endpoint}", self.id))
    }

    pub async fn react(&self, reaction: &str) -> Result<()> {
        let url = self.api(&format!("reactions/{}/%40me", urlencoding::encode(reaction)));

        let resp = self.context.context.client.put(url)
            .send().await?;
        Discord::handle_response(&resp)?;
        
        Ok(())
    }
}