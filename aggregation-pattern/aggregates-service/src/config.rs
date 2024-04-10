pub(crate) struct Config {
    pub customer_count_uri: String,
    pub incidents_grouped_by_customer_uri: String,
    pub top_customers_uri: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Config> {
        Ok(Config {
            customer_count_uri: String::from(
                "http://customers-service.spin.internal/customers/count",
            ),
            incidents_grouped_by_customer_uri: String::from(
                "http://incidents-service.spin.internal/incidents/grouped-by-customer",
            ),
            top_customers_uri: String::from(
                "http://customers-service.spin.internal/customers/top/5",
            ),
        })
    }
}
