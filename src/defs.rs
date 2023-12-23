use serde::{Deserialize, Serialize};

use crate::enums::{GeminiFinishReason, GeminiHarmCategory, GeminiSafetyThreshold, GeminiHarmProbability};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiContentBody {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiContentBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}


#[derive(Debug, Clone, Serialize)]
pub struct GeminiPostBodySafetySettings {
    pub category: GeminiHarmCategory,
    pub threshold: GeminiSafetyThreshold
}

#[derive(Debug, Clone, Serialize,)]
#[serde(rename_all = "camelCase")]
pub struct GeminiPostBodyGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<u8>
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiPostBody {
    #[serde(default)]
    pub contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<GeminiPostBodySafetySettings>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GeminiPostBodyGenerationConfig>,
}

impl Default for GeminiPostBody {
    fn default() -> Self {
        Self {
            contents: vec![],
            safety_settings: None,
            generation_config: None
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeminiSafetyRating {
    pub category: GeminiHarmCategory,
    pub probability: GeminiHarmProbability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiPostResponseCandidate {
    pub content: GeminiContent,
    pub finish_reason: Option<GeminiFinishReason>,
    pub safety_ratings: Option<Vec<GeminiSafetyRating>>,
    pub token_count: Option<u32>,
    pub index: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiPostResponse {
    pub candidates: Vec<GeminiPostResponseCandidate>,
}