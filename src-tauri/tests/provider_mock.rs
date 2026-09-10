//! End-to-end checks against a local mock provider (M6).
//! Serves canned OpenAI-compatible responses over HTTP on 127.0.0.1.

use std::sync::{Arc, Mutex};

use imagen_lib::error::ErrorKind;
use imagen_lib::openai::{
    generate_inner, test_connection, ConnectionTestRequest, GenerationRequest,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const PIXEL_PNG: &str =
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}

/// Spawns a mock server; `handler` maps request path → (status, body).
async fn spawn_mock(
    handler: impl Fn(&str) -> (u16, String) + Send + 'static,
) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            let mut buffer = vec![0u8; 16384];
            let read = socket.read(&mut buffer).await.unwrap_or(0);
            let request = String::from_utf8_lossy(&buffer[..read]).to_string();
            let path = request
                .split_whitespace()
                .nth(1)
                .unwrap_or("/")
                .to_string();
            let (status, body) = handler(&path);
            let response = format!(
                "HTTP/1.1 {} {}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                status,
                reason(status),
                body.len(),
                body
            );
            let _ = socket.write_all(response.as_bytes()).await;
        }
    });
    format!("http://{addr}")
}

fn request(base_url: &str) -> GenerationRequest {
    GenerationRequest {
        request_id: "test".into(),
        provider_id: "test".into(),
        base_url: base_url.into(),
        api_key: "sk-test".into(),
        model: "gpt-image-1".into(),
        prompt: "a test image".into(),
        aspect_ratio: "1:1".into(),
        quality: "Standard".into(),
        count: 1,
    }
}

fn ok_handler(path: &str) -> (u16, String) {
    if path.ends_with("/models") {
        return (
            200,
            r#"{"data":[{"id":"gpt-image-1"},{"id":"dall-e-3"}]}"#.into(),
        );
    }
    (
        200,
        format!(r#"{{"data":[{{"b64_json":"{PIXEL_PNG}"}}]}}"#),
    )
}

fn temp_dir() -> std::path::PathBuf {
    let dir = std::env::temp_dir()
        .join("imagen-test")
        .join(uuid_v4());
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{nanos:x}-{}", std::process::id())
}

#[tokio::test]
async fn generates_and_saves_b64_image() {
    let base = spawn_mock(ok_handler).await;
    let dir = temp_dir();
    let written = Arc::new(Mutex::new(Vec::new()));
    let images = generate_inner(&request(&base), dir.clone(), written.clone())
        .await
        .unwrap();
    assert_eq!(images.len(), 1);
    assert!(std::path::Path::new(&images[0].path).exists());
    assert!(images[0].thumb.is_some());
    assert_eq!(images[0].width, 1024);
    assert_eq!(written.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn maps_auth_error() {
    let base =
        spawn_mock(|_| (401, r#"{"error":{"message":"bad key"}}"#.into())).await;
    let error = generate_inner(
        &request(&base),
        temp_dir(),
        Arc::new(Mutex::new(Vec::new())),
    )
    .await
    .unwrap_err();
    assert!(matches!(error.kind, ErrorKind::Auth));
    assert!(error.message.contains("bad key"));
}

#[tokio::test]
async fn maps_rate_limit_and_server_errors() {
    for (status, expect) in [(429u16, ErrorKind::RateLimit), (500, ErrorKind::Server)] {
        let base = spawn_mock(move |_| (status, "{}".into())).await;
        let error = generate_inner(
            &request(&base),
            temp_dir(),
            Arc::new(Mutex::new(Vec::new())),
        )
        .await
        .unwrap_err();
        assert!(
            std::mem::discriminant(&error.kind) == std::mem::discriminant(&expect),
            "status {status} should map to {expect:?}, got {:?}",
            error.kind
        );
    }
}

#[tokio::test]
async fn rejects_invalid_json_and_empty_data() {
    let base = spawn_mock(|_| (200, "not json".into())).await;
    let error = generate_inner(
        &request(&base),
        temp_dir(),
        Arc::new(Mutex::new(Vec::new())),
    )
    .await
    .unwrap_err();
    assert!(matches!(error.kind, ErrorKind::InvalidResponse));

    let base = spawn_mock(|_| (200, r#"{"data":[]}"#.into())).await;
    let error = generate_inner(
        &request(&base),
        temp_dir(),
        Arc::new(Mutex::new(Vec::new())),
    )
    .await
    .unwrap_err();
    assert!(matches!(error.kind, ErrorKind::InvalidResponse));
}

#[tokio::test]
async fn rejects_corrupt_image_bytes() {
    let base = spawn_mock(|_| {
        (
            200,
            r#"{"data":[{"b64_json":"!!!not-base64!!!"}]}"#.into(),
        )
    })
    .await;
    let error = generate_inner(
        &request(&base),
        temp_dir(),
        Arc::new(Mutex::new(Vec::new())),
    )
    .await
    .unwrap_err();
    assert!(matches!(error.kind, ErrorKind::InvalidResponse));
}

#[tokio::test]
async fn connection_test_returns_models_and_latency() {
    let base = spawn_mock(ok_handler).await;
    let result = test_connection(ConnectionTestRequest {
        base_url: base,
        api_key: "sk-test".into(),
        provider_id: String::new(),
    })
    .await
    .unwrap();
    assert_eq!(result.models, vec!["gpt-image-1", "dall-e-3"]);
}

#[tokio::test]
async fn connection_test_maps_auth() {
    let base =
        spawn_mock(|_| (401, r#"{"error":{"message":"bad key"}}"#.into())).await;
    let error = test_connection(ConnectionTestRequest {
        base_url: base,
        api_key: "sk-test".into(),
        provider_id: String::new(),
    })
    .await
    .unwrap_err();
    assert!(matches!(error.kind, ErrorKind::Auth));
}
