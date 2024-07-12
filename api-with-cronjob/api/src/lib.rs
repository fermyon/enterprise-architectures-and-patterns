use anyhow::Context;

use models::{Counter, COUNTER_KEY};
use spin_sdk::http::{IntoResponse, Params, Request, Response, ResponseBuilder, Router};
use spin_sdk::key_value::Store;
use spin_sdk::{http_component, variables};

mod models;

const VAR_STORE: &str = "store";
const CONTENT_TYPE_JSON: &str = "application/json";

#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get("/", print_help);
    router.get("/value", get_value);
    router.post("/value", set_value);
    router.delete("/value", delete_value);
    Ok(router.handle(req))
}

fn delete_value(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let store = get_store()?;
    store.delete(COUNTER_KEY)?;
    Ok(Response::new(204, ()))
}

fn set_value(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let store = get_store()?;
    let mut counter = store.get_json::<Counter>(COUNTER_KEY)?.unwrap_or_default();
    counter.count += 1;
    store.set_json(COUNTER_KEY, &counter)?;
    Ok(ResponseBuilder::new(200)
        .header("Content-Type", CONTENT_TYPE_JSON)
        .body(counter)
        .build())
}

fn get_value(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let store = get_store()?;
    let counter = store.get_json::<Counter>(COUNTER_KEY)?.unwrap_or_default();
    Ok(ResponseBuilder::new(200)
        .header("Content-Type", CONTENT_TYPE_JSON)
        .body(counter)
        .build())
}

fn print_help(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let help = r#"<html>
<head><title>API with Cron</title></head>
<body>
    <h1>API with Cron</h1>
    <br/>
    <p>This API exposes three endpoints that you can invoke via HTTP:</p>
    <ul>
        <li><code>GET /value</code>: to return the value of the counter</li>
        <li><code>POST /value</code>: to increase the counter by 1</li>
        <li><code>DELETE /value</code>: set the value of the counter to 0</li>
    </ul>
    <p>You can run the API locally using <code>spin up -f ./spin.toml --runtime-config-file ./local.rtc.toml</cpde>

    <h2>The Cron</h2>
    <p>The Cron will wipe the underlying key-value store (which results in the counter being again set to 0). You can start the Cron on your local machine at any time using the <code>spin up -f ./spin-cron.toml --runtime-config-file ./local.rtc.toml</code> command.</p>
    </body>
</html>"#;

    Ok(ResponseBuilder::new(200)
        .header("Content-Type", "text/html")
        .body(help)
        .build())
}

fn get_store() -> anyhow::Result<Store> {
    let store_name = variables::get(VAR_STORE)?;
    println!("Will use store {}", store_name);
    Store::open(store_name.as_str()).context(format!("Error while opening store {}", store_name))
}
