use anyhow::Result;
use spin_sdk::http::{
    send, HeaderValue, IntoResponse, Params, Request, RequestBuilder, Response, ResponseBuilder,
    Router,
};
use spin_sdk::http_component;

const QUERY_ROOT_URL: &str = "https://queries.spin.internal/employees";
const COMMAND_ROOT_URL: &str = "https://commands.spin.internal";

#[http_component]
fn handle_gateway(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get_async("/employees", get_employees);
    router.get_async("/employees/:id", get_employee_by_id);

    router.post_async("/employees", create_employee);
    router.put_async("/employees/:id", update_employee_by_id);
    router.delete_async("/employees/:id", delete_employee_by_id);
    Ok(router.handle(req))
}

async fn create_employee(req: Request, _: Params) -> Result<impl IntoResponse> {
    let url = format!("{}/create", COMMAND_ROOT_URL);
    execute_command(url, req.header("content-type"), Some(req.body().to_vec())).await
}

async fn update_employee_by_id(req: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(400, ()));
    };
    let url = format!("{}/update/{}", COMMAND_ROOT_URL, id);
    let ct = req.header("content-type");
    execute_command(url, ct, Some(req.body().to_vec())).await
}

async fn delete_employee_by_id(_: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(400, ()));
    };
    let url = format!("{}/delete/{}", COMMAND_ROOT_URL, id);

    execute_command(url, None, None).await
}

async fn execute_command(
    url: String,
    content_type: Option<&HeaderValue>,
    payload: Option<Vec<u8>>,
) -> Result<Response> {
    let req: Request = match content_type {
        Some(ct) => RequestBuilder::new(spin_sdk::http::Method::Post, url)
            .header("Accept", "application/json")
            .header("Content-Type", ct.as_str().unwrap())
            .body(payload)
            .build(),
        None => RequestBuilder::new(spin_sdk::http::Method::Post, url)
            .header("Accept", "application/json")
            .body(())
            .build(),
    };

    let res: Response = send(req).await?;
    parse_result(res)
}

async fn get_employee_by_id(_: Request, params: Params) -> Result<impl IntoResponse> {
    match params.get("id") {
        Some(id) => {
            let url = format!("{}/{}", QUERY_ROOT_URL, id);
            execute_query(url.as_str()).await
        }
        None => Ok(Response::new(200, ())),
    }
}

async fn execute_query(url: &str) -> Result<Response> {
    let req: Request = RequestBuilder::new(spin_sdk::http::Method::Get, url)
        .header("Accept", "application/json")
        .build();
    let res: Response = send(req).await?;
    parse_result(res)
}

fn parse_result(res: Response) -> Result<Response> {
    match res.status() {
        300..=499 => Ok(Response::new(*res.status(), ())),
        500..=599 => {
            println!("{}", String::from_utf8_lossy(res.body()));
            Ok(Response::new(500, "Internal Server Error"))
        }
        200 | 201 | 204 => Ok(ResponseBuilder::new(*res.status())
            .header("Content-Type", "application/json")
            .body(res.into_body())
            .build()),
        _ => {
            println!("{}", String::from_utf8_lossy(res.body()));
            Ok(Response::new(*res.status(), ()))
        }
    }
}

async fn get_employees(_: Request, _: Params) -> Result<impl IntoResponse> {
    execute_query(QUERY_ROOT_URL).await
}
