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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DetectResponse {
    pub is_english: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BatchDetectRequest {
    pub texts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BatchDetectResponse {
    pub results: Vec<DetectResponse>,
}

pub async fn detect_language(
    State(detector): State<Arc<LanguageDetector>>,
    Json(payload): Json<BatchDetectRequest>,
) -> Result<Json<BatchDetectResponse>, StatusCode> {
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

pub async fn health() -> &'static str {
    "OK"
}

pub fn create_app() -> Router {
    println!("📚 Initializing Lingua detector (optimized for English detection)...");

    use lingua::Language::*;
    let lingua_detector = Arc::new(
        LanguageDetectorBuilder::from_languages(&[English, French, German, Spanish])
            .with_minimum_relative_distance(0.4)
            .with_preloaded_language_models()
            .build(),
    );

    println!("✅ Lingua detector initialized (4 languages, min_distance=0.4, high accuracy mode, confidence > 0.5 for English)");

    Router::new()
        .route("/detect", post(detect_language))
        .route("/health", get(health))
        .with_state(lingua_detector)
}
