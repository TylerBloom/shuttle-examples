use axum::{extract::Path, routing::get, Router};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/api/v1/hello", get(hello))
        .route("/api/v1/hello/:name", get(hello_there))
        .fallback(unknown);

    Ok(router.into())
}

async fn hello() -> &'static str {
    "Give you your name, please."
}

async fn hello_there(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}

async fn unknown() -> &'static str {
    "Unknown path... try again"
}
