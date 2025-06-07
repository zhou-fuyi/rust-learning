use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use axum::{extract::State, http::StatusCode, routing::get, Router};
use tracing::info;

#[derive(Debug)]
struct HttpServerState {
    path: PathBuf,
}

pub async  fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving directory '{:?}' on port {}", path, port);

    let state = HttpServerState { path : path.clone() };

    let serve_dir = tower_http::services::ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_deflate()
        .precompressed_zstd();
        // .fallback(tower_http::services::ServeFile::new("404.html"));

    let router = Router::new()
        // .route("/{*subpath}", get(file_handler))
        .fallback_service(serve_dir)
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, router)
        .await?;
    Ok(())
}

async fn file_handler(State(root): State<Arc<HttpServerState>>, axum::extract::Path(subpath): axum::extract::Path<String>) -> (StatusCode, String) {
    let full_path = root.path.join(&subpath);
    info!("Requested file: {:?}", full_path);

    if full_path.exists() {
        match tokio::fs::read_to_string(full_path).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error reading file: {}", e)),
        }
    } else {
        (StatusCode::NOT_FOUND, format!("File not found: {:?}", full_path))
    }
}
