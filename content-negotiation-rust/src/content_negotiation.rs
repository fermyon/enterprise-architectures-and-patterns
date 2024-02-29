use std::fmt::Display;

use anyhow::Context;
use serde::Serialize;
use spin_sdk::http::{conversions::IntoBody, Request, ResponseBuilder};

pub enum SupportedContentType {
    Xml,
    Json,
    Yaml,
    PlainText,
}

impl From<&str> for SupportedContentType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "application/yaml" => SupportedContentType::Yaml,
            "application/xml" => SupportedContentType::Xml,
            "text/plain" => SupportedContentType::PlainText,
            _ => SupportedContentType::Json,
        }
    }
}

impl SupportedContentType {
    fn as_header_value(self) -> String {
        match self {
            SupportedContentType::Xml => String::from("application/xml"),
            SupportedContentType::Yaml => String::from("application/yaml"),
            SupportedContentType::Json => String::from("application/json"),
            SupportedContentType::PlainText => String::from("text/plain"),
        }
    }
}

pub trait Negotiate {
    fn negotiate<T>(&mut self, req: &Request, data: &T) -> &mut Self
    where
        T: Serialize + Display;
}

impl Negotiate for ResponseBuilder {
    fn negotiate<T>(&mut self, req: &Request, data: &T) -> &mut Self
    where
        T: Serialize + Display,
    {
        let ct = detect_content_type(req);
        match negotiate_content(data, &ct) {
            Ok(payload) => self
                .header("Content-Type", &ct.as_header_value())
                .body(payload),
            Err(e) => self.status(500).body(format!("{}", e)),
        };
        self
    }
}

fn detect_content_type(req: &Request) -> SupportedContentType {
    let Some(accept_header) = req.header("Accept") else {
        return SupportedContentType::Json;
    };
    let Some(accept_header_value) = accept_header.as_str() else {
        return SupportedContentType::Json;
    };
    SupportedContentType::from(accept_header_value)
}

fn negotiate_content<T>(
    data: &T,
    content_type: &SupportedContentType,
) -> anyhow::Result<impl IntoBody>
where
    T: Serialize + Display,
{
    match content_type {
        SupportedContentType::PlainText => Ok(format!("{}", data)),
        SupportedContentType::Json => {
            serde_json::to_string(data).with_context(|| "Error while producing JSON")
        }
        SupportedContentType::Yaml => {
            serde_yaml::to_string(data).with_context(|| "Error while producing YAML")
        }
        SupportedContentType::Xml => match quick_xml::se::to_string(data) {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e);
                Ok(String::from(""))
            }
        },
    }
}
