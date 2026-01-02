use axum::http::StatusCode;
use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use lingua::{LanguageDetector, LanguageDetectorBuilder};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
struct DetectResponse {
    is_english: bool,
}

#[derive(Deserialize)]
struct BatchDetectRequest {
    texts: Vec<String>,
}

#[derive(Serialize)]
struct BatchDetectResponse {
    results: Vec<DetectResponse>,
}

async fn detect_language(
    State(detector): State<Arc<LanguageDetector>>,
    Json(payload): Json<BatchDetectRequest>,
) -> Result<Json<BatchDetectResponse>, StatusCode> {
    // Run CPU-bound work on blocking thread pool
    let texts = payload.texts;
    let results = tokio::task::spawn_blocking(move || {
        texts
            .par_iter()
            .map(|text| {
                let detected = detector.detect_language_of(text.as_str());
                let confidence = if let Some(lang) = detected {
                    detector.compute_language_confidence(text.as_str(), lang)
                } else {
                    0.0
                };
                let is_english = detected
                    .map(|l| l.iso_code_639_1().to_string() == "en" && confidence > 0.5)
                    .unwrap_or(false);
                DetectResponse { is_english }
            })
            .collect::<Vec<_>>()
    })
    .await
    .map_err(|e| {
        eprintln!("❌ Detection task failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(BatchDetectResponse { results }))
}

async fn health() -> &'static str {
    "OK"
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    println!("📚 Initializing Lingua detector (optimized for English detection)...");

    // Build lingua detector optimized for English detection precision
    // 4 languages + minimum relative distance for high precision + high accuracy mode (explicit)
    use lingua::Language::*;
    let lingua_detector = Arc::new(
        LanguageDetectorBuilder::from_languages(&[English, French, German, Spanish])
            .with_minimum_relative_distance(0.4)
            .with_preloaded_language_models()
            .build(), // High accuracy mode is default (not low accuracy)
    );

    println!("✅ Lingua detector initialized (4 languages, min_distance=0.4, high accuracy mode, confidence > 0.5 for English)");

    let app = Router::new()
        .route("/detect", post(detect_language))
        .route("/health", get(health))
        .with_state(lingua_detector);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030")
        .await
        .expect("Failed to bind to port 3030");

    println!("🚀 Lingua English Detection Service listening on 0.0.0.0:3030");
    println!("   - POST /detect (English detection with confidence > 0.5)");
    println!("   - GET  /health");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");
}
