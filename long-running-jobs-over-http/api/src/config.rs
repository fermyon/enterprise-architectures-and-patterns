use spin_sdk::variables;

pub(crate) struct Config {
    mqtt_address: String,
    mqtt_client_id: String,
    pub(crate) mqtt_username: String,
    pub(crate) mqtt_password: String,
    pub(crate) mqtt_keep_alive: u64,
    pub(crate) topic_name: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Config> {
        Ok(Config {
            mqtt_address: variables::get("mqtt_address")?,
            mqtt_client_id: variables::get("mqtt_client_id").unwrap_or(String::from("client001")),
            mqtt_username: variables::get("mqtt_username").unwrap_or_default(),
            mqtt_password: variables::get("mqtt_password").unwrap_or_default(),
            mqtt_keep_alive: variables::get("mqtt_keep_alive")?.parse().unwrap_or(30),
            topic_name: variables::get("topic_name").unwrap_or(String::from("jobs/new")),
        })
    }

    pub fn get_connection_string(&self) -> String {
        format!("{}?client_id={}", self.mqtt_address, self.mqtt_client_id)
    }
}
