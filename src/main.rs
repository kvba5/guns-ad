use std::{fs, time::Duration};

use anyhow::{Context, Ok, Result, anyhow};
use clap::Parser;
use helper::{config::Config, discord::Discord};
use tokio::time::sleep;

mod helper;

const CHANNEL_ID: &str = "1145771692099121206";

const DELAY: u16 = 30 * 60 + 3; // + 3 - occasional jitter

#[derive(Parser, Debug)]
#[command(version, about, long_about = "guns.lol #bio-links advertise bot")]
struct Args {
    /// Custom path to the config file (defaults to ./config.toml)
    #[arg(short, long, value_name = "PATH")]
    config: Option<String>
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::load(args.config).context("Error occured when loading config!")?;
    
    let txt = {
        let exists = fs::exists(&config.msg_path)
            .map_err(|why| anyhow!("{why}\n\nCould not check existance of a message txt file. Please ensure you have all needed permissions!"))?;
        
        if !exists {
            return Err(
                anyhow!("{} doesn't exist!", &config.msg_path)
            )
        }

        Ok(
            fs::read_to_string(&config.msg_path)?
        )
    }.context("Could not read txt file with message!")?;

    
    let client = Discord::new(&config.token).context("Could not create Discord client!")?;
    let user = client.get_current_user().await.context("Could not get current user!")?;
    let channel = client.channel(CHANNEL_ID);

    println!("Welcome, {} ({})!\nSending message...", user.global_name, user.username);

    loop {
        let msg = channel.send_message(&txt).await?;
        msg.react("upvote:1185979066466181162").await?;

        sleep(Duration::from_secs(DELAY as u64)).await;
    }
}
