use std::{fs, io::{stdout, Write}, time::Duration};
use anyhow::{Context, Ok, Result, anyhow};
use clap::Parser;
use helper::{config::Config, discord::Discord, progress_bar::wait_progressbar};
use rand::{rngs::ThreadRng, Rng};

mod helper;

const UPVOTE_EMOJI_ID: &str = "upvote:1185979066466181162";
const DEFAULT_DELAY: u8 = 60; // 60 seconds by default

#[derive(Parser, Debug)]
#[command(version, about, long_about = "guns.lol #bio-links advertise bot")]
struct Args {
    /// Custom path to the config file
    #[arg(short = 'c', long = "config", value_name = "PATH", default_value = "./config.toml")]
    config: String,

    /// Custom channel ID, only useful if old channel has been removed and default value is deprecated.
    #[arg(long = "channel", value_name = "CHANNEL_ID", default_value = "1145771692099121206")]
    channel_id: String
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::load(Some(args.config)).context("Error occured when loading config!")?;
    
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

    
    let mut client = Discord::new(&config.token).context("Could not create Discord client!")?;
    let user = client.get_current_user().await.context("Could not get current user!")?;
    let mut channel_instance = client.channel(&args.channel_id);
    let channel = channel_instance.get().await.context("Could not get channel")?;

    // Channel is not Text Channel
    if channel.channel_type != 0 {
        return Err(
            anyhow!("Invalid channel provided! It must be a Text Channel!")
        )
    }

    print!("Welcome, {} ({})!\n", user.global_name, user.username);
    print!("Sending message to #{}...\n\n", channel.name);
    stdout().flush()?;
    

    let base_duration: u64 = if channel.rate_limit_per_user > 10 { channel.rate_limit_per_user as u64 } else { DEFAULT_DELAY as u64 };
    let mut slowmode_remaining: u32 = 0;

    loop {
        let r = channel_instance.send_message(&txt).await?;
        if let Some(message) = r.msg {
            message.react(UPVOTE_EMOJI_ID).await?;
        } else {
            slowmode_remaining = r.slowmode;
        }

        let duration = if slowmode_remaining == 0 { base_duration } else { slowmode_remaining as u64 } + ThreadRng::default().gen_range(1..5);
        wait_progressbar(
            Duration::from_secs(duration),
            20,
            Some(&format!("%p Slowmode Progress ({duration} seconds)"))
        ).await?;
        slowmode_remaining = 0;
    }
}
