use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let state = HttpServeState { path };

    // axum router
    let router = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting http server on port {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    info!("Hello from {:?}, request path {}", state, path);
    let p = std::path::Path::new(&state.path).join(path);
    if !p.exists() {
        (StatusCode::NOT_FOUND, "404 Not Found".to_string())
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => (StatusCode::OK, content),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "500 Internal Server Error".to_string(),
            ),
        }
    }
}
