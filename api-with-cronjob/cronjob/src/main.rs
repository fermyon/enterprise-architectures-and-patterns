use std::{thread, time::Duration};

use anyhow::Result;
use spin_sdk::{key_value::Store, variables};

#[allow(warnings)]
mod bindings;

fn main() -> Result<()> {
    println!("Loading Spin Application Variables");
    let store_name = variables::get("store")?;
    println!("Accessing key-value store: '{}'", store_name);
    let store = Store::open(store_name.as_str())?;
    println!("Key-value store {} opened", store_name);
    let keys = store.get_keys()?;
    let _: Vec<_> = keys
        .into_iter()
        .filter_map(|key| {
            println!("Key-value store has key '{}'", key);
            match store.delete(key.as_str()) {
                Ok(_) => Some(key),
                Err(_) => None,
            }
        })
        .inspect(|key| println!("Removed key '{}' from key-value store", key))
        .collect();
    Ok(())
}
