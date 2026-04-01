//! HTTP server — axum REST API and embedded SPA.

use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, put},
};
use serde_json::{json, Value};

use crate::Jayson;
use crate::layer::LayerSource;

type AppState = Arc<RwLock<Jayson>>;

pub async fn run(jayson: Jayson) -> anyhow::Result<()> {
    let port = jayson.port;
    let host = jayson.host.clone();

    let state: AppState = Arc::new(RwLock::new(jayson));

    let app = Router::new()
        .route("/", get(serve_spa))
        .route("/api/schema", get(get_schema))
        .route("/api/layers", get(list_layers))
        .route("/api/layers/{id}", get(get_layer))
        .route("/api/layers/{id}", put(update_layer))
        .route("/api/merged", get(get_merged))
        .route("/api/preview", get(get_preview))
        .route("/api/validate", get(get_validate))
        .route("/api/config", get(get_config))
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let actual_addr = listener.local_addr()?;

    let url = format!("http://{}", actual_addr);
    log::info!("Jayson editor serving at {}", url);

    if let Err(e) = open::that(&url) {
        log::warn!("Failed to open browser: {}. Open {} manually.", e, url);
    }

    axum::serve(listener, app).await?;

    Ok(())
}

async fn serve_spa() -> Html<&'static str> {
    Html(crate::ui::INDEX_HTML)
}

async fn get_schema(State(state): State<AppState>) -> Json<Value> {
    let jayson = state.read().await;
    Json(jayson.schema.clone())
}

async fn list_layers(State(state): State<AppState>) -> Json<Value> {
    let jayson = state.read().await;
    let layers: Vec<Value> = (0..jayson.stack.len())
        .filter_map(|i| {
            jayson.stack.get(i).map(|layer| {
                json!({
                    "id": i,
                    "name": layer.name,
                    "readonly": layer.readonly,
                })
            })
        })
        .collect();
    Json(Value::Array(layers))
}

async fn get_layer(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    let jayson = state.read().await;
    match jayson.stack.get(id) {
        Some(layer) => Json(layer.data.clone()).into_response(),
        None => (StatusCode::NOT_FOUND, Json(json!({"error": "Layer not found"}))).into_response(),
    }
}

async fn update_layer(
    State(state): State<AppState>,
    Path(id): Path<usize>,
    Json(data): Json<Value>,
) -> impl IntoResponse {
    // Collect all data under the write lock, then release before I/O
    let (save_info, merged, has_callback) = {
        let mut jayson = state.write().await;

        match jayson.stack.get(id) {
            Some(l) if l.readonly => {
                return (StatusCode::FORBIDDEN, Json(json!({"error": "Layer is readonly"}))).into_response();
            }
            Some(_) => {}
            None => {
                return (StatusCode::NOT_FOUND, Json(json!({"error": "Layer not found"}))).into_response();
            }
        }

        let layer = jayson.stack.get_mut(id).expect("checked above");
        layer.data = data;

        // Serialize while we still hold the lock to avoid races
        let save_info = match layer.source {
            LayerSource::File(ref path) => {
                let content = serde_json::to_string_pretty(&layer.data)
                    .expect("Value serialization is infallible");
                Some((path.clone(), content))
            }
            LayerSource::Memory => None,
        };

        let merged = jayson.stack.merged();
        let has_callback = jayson.on_save.is_some();

        (save_info, merged, has_callback)
    };

    if let Some((path, content)) = save_info {
        if let Err(e) = tokio::fs::write(&path, &content).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to save: {}", e)})),
            ).into_response();
        }
    }

    if has_callback {
        let jayson = state.read().await;
        if let Some(ref on_save) = jayson.on_save {
            on_save(&merged);
        }
    }

    Json(json!({"ok": true})).into_response()
}

async fn get_merged(State(state): State<AppState>) -> Json<Value> {
    let jayson = state.read().await;
    Json(jayson.merged_value())
}

async fn get_preview(State(state): State<AppState>) -> impl IntoResponse {
    let jayson = state.read().await;
    match jayson.render_preview() {
        Ok(html) => {
            let content_type = jayson.preview.content_type().to_string();
            (
                StatusCode::OK,
                [(axum::http::header::CONTENT_TYPE, content_type)],
                html,
            ).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.message})),
        ).into_response(),
    }
}

async fn get_config(State(state): State<AppState>) -> Json<Value> {
    let jayson = state.read().await;
    Json(json!({
        "title": jayson.title,
    }))
}

async fn get_validate(State(state): State<AppState>) -> impl IntoResponse {
    let jayson = state.read().await;
    match jayson.validate() {
        Ok(errors) if errors.is_empty() => {
            Json(json!({"valid": true, "errors": []})).into_response()
        }
        Ok(errors) => {
            Json(json!({"valid": false, "errors": errors})).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("{}", e)})),
        ).into_response(),
    }
}
