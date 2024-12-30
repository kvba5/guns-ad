use std::{cmp, io::{stdout, Write}, time::{Duration, Instant}};
use anyhow::Result;

use tokio::time::sleep;

pub fn generate_progressbar(current: u8, length: u8) -> String {
    let current = cmp::min(cmp::max(0, current), length);
    
    format!(
        "[{}{}]",
        "=".repeat(current.into()),
        "-".repeat((length - current).into())
    )
}

pub async fn wait_progressbar(duration: Duration, length: u8, template: Option<&str>) -> Result<()> {
    let template = template.unwrap_or("%p");

    let step_duration = duration / length as u32;
    let start = Instant::now();

    for i in 0..=length {
        let elapsed = start.elapsed();
        let progress_bar = generate_progressbar(i, length);

        print!(
            "  {}\r",
            template.replace("%p", &progress_bar)
        );
        stdout().flush()?;

        if i < length {
            let next_target = step_duration * (i as u32 + 1);
            if next_target > elapsed {
                sleep(next_target - elapsed).await;
            }
        }
    }

    Ok(())
}