use models::Item;
use spin_contrib_http::cors::{
    CorsConfig, CorsResponseBuilder, CorsRouter, ALL_HEADERS, ALL_METHODS, NO_ORIGINS,
};
use spin_contrib_http::request::Contrib;
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::sqlite::{Connection, Value};
use spin_sdk::{http_component, variables};

mod models;
fn load_cors_config() -> CorsConfig {
    CorsConfig::new(
        variables::get("cors_allowed_origins").unwrap_or(NO_ORIGINS.into()),
        variables::get("cors_allowed_methods").unwrap_or(ALL_METHODS.into()),
        variables::get("cors_allowed_headers").unwrap_or(ALL_HEADERS.into()),
        variables::get("cors_allow_credentials")
            .unwrap_or("true".to_string())
            .parse()
            .unwrap_or(true),
        variables::get("cors_max_age")
            .ok()
            .and_then(|v| v.parse::<u32>().ok()),
    )
}

#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let cfg = load_cors_config();
    println!("Using CORS config: {:?}", cfg);
    let mut router = Router::default();
    router.register_options_handler(&cfg);
    router.get("/items", get_items);
    router.post("/items", post_item);
    router.delete("/items/:id", delete_item);

    println!("Handing {:?} {:?}", req.method(), req.uri());
    let method = &req.method().clone();
    let request_origin = req.get_header_value_as_string("origin");

    Ok(router
        .handle(req)
        .into_builder()
        .build_with_cors(method, request_origin, &cfg))
}

fn get_items(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let connection = Connection::open_default()?;
    let values = [];
    let result = connection.execute("SELECT ID, NAME FROM ITEMS", values.as_slice())?;
    let items: Vec<_> = result
        .rows()
        .map(|row| {
            let id: i64 = row.get::<i64>("ID").expect("ID not set");
            let name: String = row.get::<&str>("NAME").unwrap_or_default().to_string();
            Item::new(id, name)
        })
        .collect();
    let payload = serde_json::to_vec(&items)?;

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(payload)
        .build())
}

fn post_item(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(payload) = serde_json::from_slice::<Item>(req.body()) else {
        return Ok(Response::new(500, "invalid payload received"));
    };
    let connection = Connection::open_default()?;
    let values = [Value::Text(payload.name.clone())];
    connection.execute("INSERT INTO ITEMS (NAME) VALUES (?)", values.as_slice())?;

    Ok(Response::builder().status(200).body(()).build())
}

fn delete_item(_: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(404, ()));
    };

    let Ok(id) = id.parse::<i64>() else {
        return Ok(Response::new(400, ()));
    };

    let connection = Connection::open_default()?;
    let values = [Value::Integer(id)];
    connection.execute("DELETE FROM ITEMS WHERE ID = ?", values.as_slice())?;
    Ok(Response::builder().status(200).body(()).build())
}
