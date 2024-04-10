use futures::try_join;
use spin_sdk::http::{send, IntoResponse, Params, Request, Response};

use crate::{config::Config, models::DashboardModel, request_builder, response_parser};

pub async fn get_dashboard(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let cfg = Config::load()?;
    let req_customer_count = request_builder::customer_count(&cfg);
    let req_top_customers = request_builder::top_customers(&cfg);
    let req_incidents_grouped_by_customer = request_builder::incidents_grouped_by_customer(&cfg);

    let customer_count_future = send::<_, Response>(req_customer_count);
    let top_customers_future = send::<_, Response>(req_top_customers);
    let incidents_grouped_by_customer_future =
        send::<_, Response>(req_incidents_grouped_by_customer);

    // refactor to let-else
    match try_join!(
        customer_count_future,
        top_customers_future,
        incidents_grouped_by_customer_future
    ) {
        Ok(results) => {
            let customer_count = response_parser::customer_count(results.0.body())?;
            let top_customers = response_parser::top_customers(results.1.body())?;
            let incidents_grouped_by_customer =
                response_parser::incidents_grouped_by_customer(results.2.body())?;

            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(DashboardModel::from(
                    customer_count,
                    top_customers,
                    incidents_grouped_by_customer,
                ))
                .build())
        }
        Err(e) => Ok(Response::new(500, e.to_string())),
    }
}
