use std::thread::sleep;

use chrono::Utc;
use redis::Commands;

const VAR_CONNECTION_STRING: &str = "REDIS_CONNECTION_STRING";
const VAR_CHANNEL: &str = "REDIS_CHANNEL";
const SLEEP_FOR: std::time::Duration = std::time::Duration::from_millis(500);
fn main() -> anyhow::Result<()> {
    println!("--------------\nMass Publisher\n--------------\n\nMass Publisher will publish new messages every {:?}.\n\nPress Ctrl+C to terminate this process\n...",
        SLEEP_FOR
    );
    let redis_connection_string = std::env::var(VAR_CONNECTION_STRING)
        .expect("Please set Redis connection string using the `REDIS_CONNECTION_STRING` environment variable");
    let channel = std::env::var(VAR_CHANNEL)
        .expect("Please set Redis channel using the `REDIS_CHANNEL` environment variable");
    let client = redis::Client::open(redis_connection_string)?;
    let mut con = client.get_connection()?;
    loop {
        let time = Utc::now();
        let message = format!(
            "This is a message created by Mass Publisher at {}",
            time.to_rfc3339()
        );
        println!("Publishing message...");
        con.publish(&channel, message)?;
        sleep(SLEEP_FOR);
    }
}
