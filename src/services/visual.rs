use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// =================================================
// Client Definition
// =================================================

/// A client for the Volcengine Visual (CV) service.
pub struct VisualClient<'a> {
    access_key: &'a str,
    secret_key: &'a str,
    region: &'a str,
}

impl<'a> VisualClient<'a> {
    /// Creates a new VisualClient.
    pub fn new(access_key: &'a str, secret_key: &'a str, region: &'a str) -> Self {
        Self {
            access_key,
            secret_key,
            region,
        }
    }

    /// Calls the OCRNormal (General OCR) API.
    /// https://www.volcengine.com/docs/6790/117730
    pub async fn ocr_normal(
        &self,
        req: OcrNormalRequest,
    ) -> Result<OcrNormalResponse, Box<dyn std::error::Error>> {
        let endpoint = "https://visual.volcengineapi.com";
        let service = "cv";
        let content_type = "application/x-www-form-urlencoded";

        let mut query_params = BTreeMap::new();
        query_params.insert("Action", "OCRNormal");
        query_params.insert("Version", "2020-08-26");

        // Serialize the request struct into a URL-encoded string.
        // This correctly handles percent-encoding for special characters in base64 strings.
        let body_str = serde_urlencoded::to_string(req)?;

        crate::volce::send_request(
            self.access_key,
            self.secret_key,
            endpoint,
            self.region,
            service,
            "POST",
            content_type,
            query_params,
            body_str,
        )
        .await
    }

    /// Calls the Text-to-Image API.
    pub async fn text_to_image(
        &self,
        req: TextToImageRequest,
    ) -> Result<TextToImageResponse, Box<dyn std::error::Error>> {
        let endpoint = "https://visual.volcengineapi.com";
        let service = "cv";
        let content_type = "application/json";

        let mut query_params = BTreeMap::new();
        query_params.insert("Action", "CVProcess");
        query_params.insert("Version", "2022-08-31");

        let body_str = serde_json::to_string(&req)?;

        crate::volce::send_request(
            self.access_key,
            self.secret_key,
            endpoint,
            self.region,
            service,
            "POST",
            content_type,
            query_params,
            body_str,
        )
        .await
    }
}

// =================================================
// OCR Normal Data Structures
// =================================================

#[derive(Serialize, Debug, Default)]
pub struct OcrNormalRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_base64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_pixel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_thresh: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub half_to_full: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OcrNormalResponse {
    pub code: i64,
    pub data: Option<OcrNormalData>,
    pub message: String,
    pub request_id: String,
    pub time_elapsed: String,
}

#[derive(Deserialize, Debug)]
pub struct OcrNormalData {
    pub line_texts: Vec<String>,
    pub line_rects: Vec<RectInfo>,
    pub line_probs: Vec<f32>,
    pub chars: Vec<Vec<CharInfo>>,
    pub polygons: Vec<Vec<Vec<i32>>>,
}

#[derive(Deserialize, Debug)]
pub struct RectInfo {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Deserialize, Debug)]
pub struct CharInfo {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub score: f32,
    pub char: String,
}

// =================================================
// Text to Image Data Structures
// =================================================

#[derive(Serialize, Debug)]
pub struct TextToImageRequest {
    pub req_key: String,
    pub prompt: String,
    pub model_version: String,
    pub seed: i64,
    pub scale: f64,
    pub ddim_steps: i64,
    pub width: i64,
    pub height: i64,
    pub use_sr: bool,
    pub return_url: bool,
}

#[derive(Deserialize, Debug)]
pub struct TextToImageResponse {
    pub code: i64,
    pub message: String,
    pub data: Option<TextToImageData>,
}

#[derive(Deserialize, Debug)]
pub struct TextToImageData {
    pub image_urls: Vec<String>,
}
