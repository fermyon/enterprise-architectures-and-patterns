use anyhow::Context;

use crate::models::{
    CustomerCountResponseModel, CustomerListModel, IncidentsGroupedByCustomerResponseModel,
};

pub fn customer_count(body: &[u8]) -> anyhow::Result<CustomerCountResponseModel> {
    serde_json::from_slice(body).with_context(|| {
        "Error converting response from downstream service to model (CustomerCountResponseModel)"
    })
}

pub fn top_customers(body: &[u8]) -> anyhow::Result<Vec<CustomerListModel>> {
    serde_json::from_slice(body).with_context(|| {
        "Error converting response from downstream service to model (TopCustomersResponseModel)"
    })
}

pub fn incidents_grouped_by_customer(
    body: &[u8],
) -> anyhow::Result<IncidentsGroupedByCustomerResponseModel> {
    serde_json::from_slice(body)
        .with_context(|| "Error converting response from downstream service to model (IncidentsGroupedByCustomerResponseModel)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_top_customer_deserialization() {
        let json = r#"[
            {
                "id": "9AC27BB7-BDDF-E108-6B44-E1C4ACD84E97",
                "name": "Neque Vitae Corporation",
                "country": "India",
                "scoring": 10
            },
            {
                "id": "1CC638E3-198F-26BA-136E-AD33AA044DED",
                "name": "Auctor Non Corp.",
                "country": "Philippines",
                "scoring": 9
            },
            {
                "id": "B973CAF9-357C-7CC8-36F9-EAA25F839207",
                "name": "Duis Associates",
                "country": "Italy",
                "scoring": 9
            },
            {
                "id": "1346371B-5270-84D9-4CE4-96B4CF785867",
                "name": "Eu Enim Etiam Foundation",
                "country": "Netherlands",
                "scoring": 9
            },
            {
                "id": "BE45A517-2575-0669-D58C-C8A52D2BC41E",
                "name": "Id Risus Associates",
                "country": "Austria",
                "scoring": 9
            }
        ]"#;

        assert!(top_customers(json.as_bytes()).is_ok())
    }
}
