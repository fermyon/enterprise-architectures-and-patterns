use spin_sdk::http::{Method, Request, RequestBuilder};

use crate::config::Config;

pub fn customer_count(cfg: &Config) -> Request {
    build_request(&cfg.customer_count_uri)
}

pub fn incidents_grouped_by_customer(cfg: &Config) -> Request {
    build_request(&cfg.incidents_grouped_by_customer_uri)
}

pub fn top_customers(cfg: &Config) -> Request {
    build_request(&cfg.top_customers_uri)
}

fn build_request(uri: &str) -> Request {
    RequestBuilder::new(Method::Get, uri)
        .header("Accept", "application/json")
        .build()
}
