use anyhow::Result;
use bytes::Bytes;
use spin_sdk::redis_component;
use std::str::from_utf8;

#[redis_component]
fn on_message(message: Bytes) -> Result<()> {
    println!("Received Message via Redis Channel");
    println!("{}", from_utf8(&message)?);
    Ok(())
}
