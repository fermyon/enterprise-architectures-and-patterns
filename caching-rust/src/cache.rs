use anyhow::Context;
use spin_sdk::key_value::Store;

use crate::model::CacheKey;

const STORE_NAME: &str = "cache";

pub(crate) fn get_from_cache(key: &CacheKey) -> Option<String> {
    let open_default = Store::open(STORE_NAME);
    let Ok(store) = open_default else {
        return None;
    };
    let Ok(data) = store.get(key.value.as_str()) else {
        return None;
    };
    match data {
        Some(b) => String::from_utf8(b).ok(),
        None => None,
    }
}

pub(crate) fn store_in_cache(key: &CacheKey, data: String) {
    // ignore all errors because we want to return existing data
    // to the callee either way
    let Ok(store) = Store::open(STORE_NAME) else {
        return;
    };
    let _ = store.set(key.value.as_str(), &data.as_bytes());
}

pub(crate) fn invalidate_cache(key: &CacheKey) -> anyhow::Result<()> {
    let store = Store::open(STORE_NAME)?;
    match store.exists(&key.value)? {
        true => store
            .delete(&key.value)
            .with_context(|| "Error while removing data from cache"),
        false => Ok(()),
    }
}

pub(crate) fn invalidate_all() -> anyhow::Result<()> {
    let store = Store::open(STORE_NAME)?;
    let keys = store.get_keys()?;
    for key in &keys {
        store.delete(key)?
    }
    Ok(())
}
