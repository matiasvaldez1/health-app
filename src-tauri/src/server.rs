use axum::{
    body::Body,
    extract::Request,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Extension, Router,
};
use include_dir::{include_dir, Dir};
use tower_http::cors::CorsLayer;

use crate::db::DbPool;
use crate::handlers;

static BUILD_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../build");

fn api_routes() -> Router {
    Router::new()
        .route("/weight", get(handlers::list_weights).post(handlers::create_weight))
        .route("/weight/:id", delete(handlers::delete_weight))
        .route("/meditation", get(handlers::list_meditations).post(handlers::create_meditation))
        .route("/meditation/:id", delete(handlers::delete_meditation))
        .route("/feelings", get(handlers::list_feelings).post(handlers::create_feeling))
        .route("/feelings/:id", delete(handlers::delete_feeling))
        .route("/tips/latest", get(handlers::get_latest_tip))
        .route("/tips/generate", post(handlers::generate_tips))
        .route("/tips/stale", get(handlers::check_tips_stale))
        .route("/settings", get(handlers::get_settings).post(handlers::save_settings))
}

async fn serve_static(req: Request) -> Response {
    let path = req.uri().path().trim_start_matches('/');

    // Try exact file match first
    if let Some(file) = BUILD_DIR.get_file(path) {
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();
        return Response::builder()
            .header(header::CONTENT_TYPE, mime)
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    // For SPA routing: serve index.html for any non-file path
    if let Some(file) = BUILD_DIR.get_file("index.html") {
        return Response::builder()
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    (StatusCode::NOT_FOUND, "Not found").into_response()
}

pub async fn start_server(pool: DbPool, port: u16) {
    let app = Router::new()
        .nest("/api", api_routes())
        .fallback(serve_static)
        .layer(CorsLayer::permissive())
        .layer(Extension(pool));

    let addr = format!("127.0.0.1:{}", port);
    eprintln!("HTTP server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind HTTP server");
    axum::serve(listener, app).await.expect("HTTP server error");
}
