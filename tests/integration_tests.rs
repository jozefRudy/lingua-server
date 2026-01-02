use axum::{
    body::Body,
    http::{self, Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use lingua_service::{create_app, BatchDetectResponse};
use serde_json::json;
use tower::ServiceExt;

async fn check_text(app: &Router, text: &str, expected: bool) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/detect")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "texts": [text] }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Failed for text: {}",
        text
    );
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: BatchDetectResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body.results[0].is_english, expected,
        "Mismatch for text: \"{}\" (expected {}, got {})",
        text, expected, body.results[0].is_english
    );
}

#[tokio::test]
async fn test_health() {
    let app = create_app();
    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"OK");
}

#[tokio::test]
async fn test_validation_suite() {
    let app = create_app();

    // === English Tests ===
    let english_cases = vec![
        "the",
        "programming",
        "I love programming",
        "Machine learning is a subset of artificial intelligence that enables computers to learn without being explicitly programmed",
        "This is a comprehensive explanation of machine learning algorithms and their applications in modern software development practices",
    ];

    for text in english_cases {
        check_text(&app, text, true).await;
    }

    // === Non-English & Ambiguous Tests (Should be false) ===
    let non_english_cases = vec![
        "Bonjour",
        "Guten Tag",
        "Hola",
        "Soy un desarrollador de software y me gusta trabajar con tecnologías modernas",
        "Je suis développeur de logiciels et j'aime travailler avec des technologies modernes",
        "Ich bin Softwareentwickler und arbeite gerne mit modernen Technologien",
        "Ciao",
        "Sono uno sviluppatore di software e mi piace lavorare con tecnologie moderne",
        "Hallo",
        "Ik ben een softwareontwikkelaar en ik werk graag met moderne technologieën",
        "Hej",
        "Jag är en mjukvaruutvecklare och jag gillar att arbeta med moderna teknologier",
        "Привет",
        "Я разработчик программного обеспечения и люблю работать с современными технологиями",
        "Cześć",
        "Jestem programistą i lubię pracować z nowoczesnymi technologiami",
        "こんにちは",
        "私はソフトウェア開発者で、最新のテクノロジーで働くのが好きです",
        "안녕하세요",
        "저는 소프트웨어 개발자이고 현대 기술로 일하는 것을 좋아합니다",
        "नमस्ते",
        "मैं एक सॉफ्टवेयर डेवलपर हूं और आधुनिक तकनीकों के साथ काम करना पसंद करता हूं",
        "สวัสดี",
        "ฉันเป็นนักพัฒนาซอฟต์แวร์และชอบทำงานกับเทคโนโลยีสมัยใหม่",
        "مرحبا",
        "أنا مطور برمجيات وأحب العمل مع التقنيات الحديثة",
        "שלום",
        "אני מפתח תוכנה ואני אוהב לעבוד עם טכנולוגיות מודרניות",
        "merhaba",
        "Yazılım geliştiricisiyim ve modern teknolojilerle çalışmayı seviyorum",
        "sawubona",
        "Ngiyisiphakeli se-software futhi ngiyathanda ukusebenza nobuchwepheshe besimanje",
        "hello",
        "hi",
        "TIL",
        "AITA",
    ];

    for text in non_english_cases {
        check_text(&app, text, false).await;
    }
}
