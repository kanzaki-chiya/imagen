use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use base64::Engine;
use futures_util::StreamExt;
use reqwest::{Client, StatusCode, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::error::{BackendError, ErrorKind};
use crate::keys;

const MAX_IMAGE_BYTES: u64 = 50 * 1024 * 1024;
const GENERATION_TIMEOUT: Duration = Duration::from_secs(300);
const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(90);
const TEST_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationRequest {
    pub request_id: String,
    pub provider_id: String,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub prompt: String,
    pub aspect_ratio: String,
    pub quality: String,
    pub count: u8,
    #[serde(default = "default_resolution")]
    pub resolution: String,
    #[serde(default)]
    pub references: Vec<ReferencePayload>,
}

fn default_resolution() -> String {
    "1K".into()
}

/// A reference image coming from the frontend. `src` is a data URL
/// (`data:<mime>;base64,<bytes>`) as produced by `FileReader.readAsDataURL`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferencePayload {
    #[serde(default)]
    pub name: String,
    pub src: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionTestRequest {
    pub base_url: String,
    pub api_key: String,
    #[serde(default)]
    pub provider_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedImage {
    pub path: String,
    pub thumb: Option<String>,
    pub width: u32,
    pub height: u32,
    /// True when the stored file was upscaled locally to reach the
    /// requested resolution (the provider returned a smaller image).
    pub upscaled: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionTestResult {
    pub models: Vec<String>,
    pub latency_ms: u64,
}

#[derive(Deserialize)]
struct ImagesResponse {
    data: Option<Vec<ImageItem>>,
}

#[derive(Deserialize)]
struct ImageItem {
    b64_json: Option<String>,
    url: Option<String>,
}

#[derive(Deserialize)]
struct ErrorBody {
    error: Option<ErrorDetail>,
}

#[derive(Deserialize)]
struct ErrorDetail {
    message: Option<String>,
}

fn endpoint(base_url: &str, path: &str) -> Result<Url, BackendError> {
    let joined = format!("{}/{}", base_url.trim_end_matches('/'), path);
    let url = Url::parse(&joined).map_err(|_| {
        BackendError::new(
            ErrorKind::Config,
            "Enter a valid base URL, including https://.",
        )
    })?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err(BackendError::new(
            ErrorKind::Config,
            "Use an http:// or https:// base URL.",
        ));
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(BackendError::new(
            ErrorKind::Config,
            "Do not include credentials in the base URL.",
        ));
    }
    Ok(url)
}

fn network_error(error: reqwest::Error) -> BackendError {
    if error.is_timeout() {
        BackendError::new(
            ErrorKind::Network,
            "The provider timed out. Check your connection and try again.",
        )
    } else if error.is_connect() {
        BackendError::new(
            ErrorKind::Network,
            "Could not reach the provider. Check the base URL and your network.",
        )
    } else {
        BackendError::new(ErrorKind::Network, error.to_string())
    }
}

fn http_error(status: StatusCode, body: &str) -> BackendError {
    let detail = serde_json::from_str::<ErrorBody>(body)
        .ok()
        .and_then(|parsed| parsed.error.and_then(|error| error.message))
        .unwrap_or_else(|| {
            let trimmed = body.trim();
            if trimmed.is_empty() {
                format!("The provider returned HTTP {}.", status.as_u16())
            } else {
                trimmed.chars().take(240).collect()
            }
        });
    let kind = match status.as_u16() {
        400 | 404 | 422 => ErrorKind::InvalidRequest,
        401 | 403 => ErrorKind::Auth,
        429 => ErrorKind::RateLimit,
        code if code >= 500 => ErrorKind::Server,
        _ => ErrorKind::InvalidResponse,
    };
    BackendError::with_status(kind, detail, status.as_u16())
}

fn openai_size(model: &str, aspect_ratio: &str) -> (u32, u32, &'static str) {
    let portrait = matches!(aspect_ratio, "9:16" | "2:3" | "3:4");
    let square = aspect_ratio == "1:1";
    if model.starts_with("dall-e-3") {
        if square {
            (1024, 1024, "1024x1024")
        } else if portrait {
            (1024, 1792, "1024x1792")
        } else {
            (1792, 1024, "1792x1024")
        }
    } else if model.starts_with("dall-e-2") {
        (1024, 1024, "1024x1024")
    } else if square {
        (1024, 1024, "1024x1024")
    } else if portrait {
        (1024, 1536, "1024x1536")
    } else {
        (1536, 1024, "1536x1024")
    }
}

/// Frontend base grid for each aspect ratio (mirrors `dimensions()` in TS).
fn base_dimensions(aspect_ratio: &str) -> (u32, u32) {
    match aspect_ratio {
        "3:2" => (1536, 1024),
        "4:3" => (1280, 960),
        "16:9" => (1536, 864),
        "9:16" => (864, 1536),
        "2:3" => (1024, 1536),
        "3:4" => (960, 1280),
        _ => (1024, 1024),
    }
}

fn resolution_scale(resolution: &str) -> u32 {
    match resolution {
        "2K" => 2,
        "4K" => 4,
        _ => 1,
    }
}

/// Returns `(target_w, target_h, size_param)`: the requested output dims and
/// the `size` value sent to the provider. OpenAI models only accept their
/// fixed size enums; compatible endpoints get exact `WxH` when above 1K so
/// they can deliver native 2K/4K output.
fn request_size(model: &str, aspect_ratio: &str, resolution: &str) -> (u32, u32, String) {
    let (base_w, base_h) = base_dimensions(aspect_ratio);
    let scale = resolution_scale(resolution);
    let (target_w, target_h) = (base_w * scale, base_h * scale);
    let openai_model =
        model.starts_with("dall-e") || model.starts_with("gpt-image");
    let (_, _, enum_size) = openai_size(model, aspect_ratio);
    let size = if openai_model || scale == 1 {
        enum_size.to_string()
    } else {
        format!("{target_w}x{target_h}")
    };
    (target_w, target_h, size)
}

/// Decodes a `data:<mime>;base64,<bytes>` reference payload from the UI.
fn decode_reference(
    src: &str,
) -> Result<(Vec<u8>, String, &'static str), BackendError> {
    let invalid = || {
        BackendError::new(
            ErrorKind::InvalidRequest,
            "A reference image could not be read.",
        )
    };
    let (header, payload) = src.split_once(',').ok_or_else(invalid)?;
    let mime = header
        .strip_prefix("data:")
        .and_then(|rest| rest.strip_suffix(";base64"))
        .ok_or_else(invalid)?;
    if !matches!(mime, "image/png" | "image/jpeg" | "image/webp") {
        return Err(BackendError::new(
            ErrorKind::InvalidRequest,
            "Reference images must be PNG, JPEG or WebP.",
        ));
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(payload)
        .map_err(|_| invalid())?;
    if bytes.len() as u64 > MAX_IMAGE_BYTES {
        return Err(BackendError::new(
            ErrorKind::InvalidRequest,
            "A reference image exceeds the 50 MB limit.",
        ));
    }
    Ok((bytes, mime.to_string(), extension_for(Some(mime))))
}

/// Builds the multipart body for `images/edits` (image-to-image).
fn edits_form(
    request: &GenerationRequest,
    size: &str,
    count: u8,
) -> Result<reqwest::multipart::Form, BackendError> {
    let mut form = reqwest::multipart::Form::new()
        .text("model", request.model.clone())
        .text("prompt", request.prompt.clone())
        .text("size", size.to_string())
        .text("n", count.to_string());
    if let Some(quality) = openai_quality(&request.model, &request.quality) {
        form = form.text("quality", quality.to_string());
    }
    if request.model.starts_with("dall-e") {
        form = form.text("response_format", "b64_json");
    }
    // OpenAI documents `image` for a single file and `image[]` for arrays.
    let field = if request.references.len() == 1 {
        "image"
    } else {
        "image[]"
    };
    for (index, reference) in request.references.iter().enumerate() {
        let (bytes, mime, extension) = decode_reference(&reference.src)?;
        let filename = if reference.name.trim().is_empty() {
            format!("reference-{}.{}", index, extension)
        } else {
            reference.name.clone()
        };
        let part = reqwest::multipart::Part::bytes(bytes)
            .file_name(filename)
            .mime_str(&mime)
            .map_err(|_| {
                BackendError::new(
                    ErrorKind::InvalidRequest,
                    "A reference image has an unsupported type.",
                )
            })?;
        form = form.part(field.to_string(), part);
    }
    Ok(form)
}

fn openai_quality<'a>(model: &str, quality: &str) -> Option<&'a str> {
    if model.starts_with("gpt-image") {
        Some(match quality {
            "High" => "high",
            "Standard" => "medium",
            _ => "auto",
        })
    } else if model.starts_with("dall-e") {
        Some(match quality {
            "High" => "hd",
            _ => "standard",
        })
    } else {
        None
    }
}

fn resolve_api_key(provider_id: &str, provided: &str) -> Result<String, BackendError> {
    keys::resolve(provider_id, provided)?.ok_or_else(|| {
        BackendError::new(
            ErrorKind::Auth,
            "Add an API key in Settings before generating with a live provider.",
        )
    })
}

fn make_thumbnail(app_data: &PathBuf, source: &PathBuf) -> Option<PathBuf> {
    let thumbs = app_data.join("thumbs");
    std::fs::create_dir_all(&thumbs).ok()?;
    let target = thumbs.join(format!("{}.jpg", Uuid::new_v4()));
    let decoded = image::ImageReader::open(source).ok()?.decode().ok()?;
    let thumbnail = decoded.thumbnail(384, 384);
    thumbnail
        .save_with_format(&target, image::ImageFormat::Jpeg)
        .ok()?;
    Some(target)
}

fn extension_for(content_type: Option<&str>) -> &'static str {
    match content_type {
        Some(value) if value.contains("jpeg") || value.contains("jpg") => "jpg",
        Some(value) if value.contains("webp") => "webp",
        _ => "png",
    }
}

async fn save_bytes(
    dir: &PathBuf,
    bytes: &[u8],
    extension: &str,
    written: &Arc<Mutex<Vec<PathBuf>>>,
) -> Result<PathBuf, BackendError> {
    if bytes.is_empty() {
        return Err(BackendError::new(
            ErrorKind::InvalidResponse,
            "The provider returned an empty image.",
        ));
    }
    if bytes.len() as u64 > MAX_IMAGE_BYTES {
        return Err(BackendError::new(
            ErrorKind::InvalidResponse,
            "The provider returned an image larger than 50 MB.",
        ));
    }
    let path = dir.join(format!("{}.{}", Uuid::new_v4(), extension));
    tokio::fs::write(&path, bytes)
        .await
        .map_err(|error| {
            BackendError::new(
                ErrorKind::Server,
                format!("Could not save the image locally: {error}"),
            )
        })?;
    written.lock().unwrap().push(path.clone());
    Ok(path)
}

async fn fetch_image_bytes(
    client: &Client,
    url: &str,
) -> Result<(Vec<u8>, &'static str), BackendError> {
    let response = client
        .get(url)
        .timeout(DOWNLOAD_TIMEOUT)
        .send()
        .await
        .map_err(network_error)?;
    if !response.status().is_success() {
        return Err(http_error(
            response.status(),
            "The provider returned an image URL that could not be downloaded.",
        ));
    }
    if let Some(length) = response.content_length() {
        if length > MAX_IMAGE_BYTES {
            return Err(BackendError::new(
                ErrorKind::InvalidResponse,
                "The generated image exceeds the 50 MB download limit.",
            ));
        }
    }
    let extension = extension_for(
        response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
    );
    let mut bytes = Vec::new();
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(network_error)?;
        if bytes.len() as u64 + chunk.len() as u64 > MAX_IMAGE_BYTES {
            return Err(BackendError::new(
                ErrorKind::InvalidResponse,
                "The generated image exceeds the 50 MB download limit.",
            ));
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok((bytes, extension))
}

struct ProcessedImage {
    bytes: Vec<u8>,
    extension: &'static str,
    width: u32,
    height: u32,
    upscaled: bool,
}

/// Decodes the provider output to learn its real dimensions and, when a
/// higher resolution tier was requested, upscales locally toward the target
/// (aspect ratio preserved) so the stored file honestly reports its size.
/// The provider's aspect ratio is never altered — when it ignores `size`,
/// the returned dims are stored as-is and the UI flags the mismatch.
fn process_image(
    bytes: Vec<u8>,
    extension: &'static str,
    target_w: u32,
    target_h: u32,
    upscale: bool,
) -> ProcessedImage {
    let decoded = match image::load_from_memory(&bytes) {
        Ok(image) => image,
        Err(_) => {
            // Undecodable payload: keep the raw bytes rather than fail a
            // generation that produced a file the user may still open.
            return ProcessedImage {
                bytes,
                extension,
                width: target_w,
                height: target_h,
                upscaled: false,
            };
        }
    };
    let (width, height) = (decoded.width(), decoded.height());
    let factor =
        (target_w as f64 / width as f64).min(target_h as f64 / height as f64);
    if !upscale || factor <= 1.05 {
        return ProcessedImage {
            bytes,
            extension,
            width,
            height,
            upscaled: false,
        };
    }
    let new_w = ((width as f64) * factor).round() as u32;
    let new_h = ((height as f64) * factor).round() as u32;
    let resized =
        decoded.resize(new_w, new_h, image::imageops::FilterType::Lanczos3);
    let mut encoded = std::io::Cursor::new(Vec::new());
    match resized.write_to(&mut encoded, image::ImageFormat::Png) {
        Ok(()) => ProcessedImage {
            bytes: encoded.into_inner(),
            extension: "png",
            width: resized.width(),
            height: resized.height(),
            upscaled: true,
        },
        Err(_) => ProcessedImage {
            bytes,
            extension,
            width,
            height,
            upscaled: false,
        },
    }
}

pub async fn generate(
    app: &AppHandle,
    request: &GenerationRequest,
    written: Arc<Mutex<Vec<PathBuf>>>,
) -> Result<Vec<GeneratedImage>, BackendError> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|error| BackendError::new(ErrorKind::Config, error.to_string()))?
        .join("images");
    generate_inner(request, dir, written).await
}

pub async fn generate_inner(
    request: &GenerationRequest,
    dir: PathBuf,
    written: Arc<Mutex<Vec<PathBuf>>>,
) -> Result<Vec<GeneratedImage>, BackendError> {
    let api_key = resolve_api_key(&request.provider_id, &request.api_key)?;
    if request.prompt.trim().is_empty() {
        return Err(BackendError::new(
            ErrorKind::Config,
            "Describe your image before generating.",
        ));
    }
    if request.count == 0 || request.count > 4 {
        return Err(BackendError::new(
            ErrorKind::Config,
            "Choose between 1 and 4 images.",
        ));
    }
    let has_references = !request.references.is_empty();
    // Reference images switch the call to image-to-image (`images/edits`).
    let url = endpoint(
        &request.base_url,
        if has_references {
            "images/edits"
        } else {
            "images/generations"
        },
    )?;
    let (target_w, target_h, size) = request_size(
        &request.model,
        &request.aspect_ratio,
        &request.resolution,
    );
    let upscale = resolution_scale(&request.resolution) > 1;
    let mut body = json!({
        "model": request.model,
        "prompt": request.prompt,
        "size": size,
    });
    if let Some(quality) = openai_quality(&request.model, &request.quality) {
        body["quality"] = json!(quality);
    }
    if request.model.starts_with("dall-e") {
        body["response_format"] = json!("b64_json");
    }
    let client = Client::builder()
        .timeout(GENERATION_TIMEOUT)
        .build()
        .map_err(network_error)?;
    let single_image_model = request.model.starts_with("dall-e-3");
    let calls = if single_image_model { request.count } else { 1 };
    let mut items = Vec::new();
    for _ in 0..calls {
        let call_count = if single_image_model { 1 } else { request.count };
        let builder = if has_references {
            client
                .post(url.clone())
                .multipart(edits_form(request, &size, call_count)?)
        } else {
            let mut call_body = body.clone();
            call_body["n"] = json!(call_count);
            client.post(url.clone()).json(&call_body)
        };
        let response = builder
            .bearer_auth(&api_key)
            .send()
            .await
            .map_err(network_error)?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            if has_references
                && matches!(
                    status,
                    StatusCode::NOT_FOUND
                        | StatusCode::METHOD_NOT_ALLOWED
                        | StatusCode::NOT_IMPLEMENTED
                )
            {
                return Err(BackendError::with_status(
                    ErrorKind::InvalidRequest,
                    "This provider does not support reference images (image-to-image).",
                    status.as_u16(),
                ));
            }
            return Err(http_error(status, &body));
        }
        let parsed: ImagesResponse = response.json().await.map_err(|_| {
            BackendError::new(
                ErrorKind::InvalidResponse,
                "The provider returned a response that is not valid JSON.",
            )
        })?;
        items.extend(parsed.data.unwrap_or_default());
    }
    if items.is_empty() {
        return Err(BackendError::new(
            ErrorKind::InvalidResponse,
            "The provider returned no images.",
        ));
    }
    tokio::fs::create_dir_all(&dir).await.map_err(|error| {
        BackendError::new(
            ErrorKind::Server,
            format!("Could not create the image directory: {error}"),
        )
    })?;
    let data_dir = dir
        .parent()
        .map(|parent| parent.to_path_buf())
        .unwrap_or_else(|| dir.clone());
    let mut generated = Vec::with_capacity(items.len());
    for item in items {
        let (bytes, extension) = if let Some(encoded) = item.b64_json {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(encoded)
                .map_err(|_| {
                    BackendError::new(
                        ErrorKind::InvalidResponse,
                        "The provider returned an image that could not be decoded.",
                    )
                })?;
            (bytes, "png")
        } else if let Some(image_url) = item.url {
            fetch_image_bytes(&client, &image_url).await?
        } else {
            return Err(BackendError::new(
                ErrorKind::InvalidResponse,
                "The provider returned an image without data.",
            ));
        };
        if bytes.is_empty() {
            return Err(BackendError::new(
                ErrorKind::InvalidResponse,
                "The provider returned an empty image.",
            ));
        }
        if bytes.len() as u64 > MAX_IMAGE_BYTES {
            return Err(BackendError::new(
                ErrorKind::InvalidResponse,
                "The provider returned an image larger than 50 MB.",
            ));
        }
        let processed = tauri::async_runtime::spawn_blocking(move || {
            process_image(bytes, extension, target_w, target_h, upscale)
        })
        .await
        .map_err(|error| {
            BackendError::new(
                ErrorKind::Server,
                format!("Could not process the image: {error}"),
            )
        })?;
        let path = save_bytes(
            &dir,
            &processed.bytes,
            processed.extension,
            &written,
        )
        .await?;
        let thumb = {
            let data_dir = data_dir.clone();
            let source = path.clone();
            tauri::async_runtime::spawn_blocking(move || make_thumbnail(&data_dir, &source))
                .await
                .ok()
                .flatten()
        };
        generated.push(GeneratedImage {
            path: path.to_string_lossy().into_owned(),
            thumb: thumb.map(|path| path.to_string_lossy().into_owned()),
            width: processed.width,
            height: processed.height,
            upscaled: processed.upscaled,
        });
    }
    Ok(generated)
}

pub async fn test_connection(
    request: ConnectionTestRequest,
) -> Result<ConnectionTestResult, BackendError> {
    let url = endpoint(&request.base_url, "models")?;
    let client = Client::builder()
        .timeout(TEST_TIMEOUT)
        .build()
        .map_err(network_error)?;
    let api_key = keys::resolve(&request.provider_id, &request.api_key)?;
    let mut call = client.get(url);
    if let Some(key) = api_key {
        call = call.bearer_auth(key);
    }
    let started = Instant::now();
    let response = call.send().await.map_err(network_error)?;
    let latency_ms = started.elapsed().as_millis() as u64;
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(http_error(status, &body));
    }
    let models = response
        .json::<serde_json::Value>()
        .await
        .ok()
        .and_then(|value| {
            value.get("data").and_then(|data| data.as_array()).map(|list| {
                list.iter()
                    .filter_map(|entry| entry.get("id").and_then(|id| id.as_str()))
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            })
        })
        .unwrap_or_default();
    Ok(ConnectionTestResult { models, latency_ms })
}
