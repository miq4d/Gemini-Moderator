use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize, Serializer, Deserializer, de::{self, Visitor}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiPostBodyPartsBody {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiPostBodyParts {
    pub parts: Vec<GeminiPostBodyPartsBody>,
}

#[derive(Clone, Debug)]
pub enum GeminiSafetyCategory {
    Unspecified,
    Derogatory,
    Toxicity,
    Violence,
    Sexual,
    Medical,
    Dangerous,
    Harassment,
    Speech,
    Explicit,
    DangerousContent
}

impl Display for GeminiSafetyCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Unspecified => "HARM_CATEGORY_UNSPECIFIED",
            Self::Derogatory => "HARM_CATEGORY_DEROGATORY",
            Self::Toxicity => "HARM_CATEGORY_TOXICITY",
            Self::Violence => "HARM_CATEGORY_VIOLENCE",
            Self::Sexual => "HARM_CATEGORY_SEXUAL",
            Self::Medical => "HARM_CATEGORY_MEDICAL",
            Self::Dangerous => "HARM_CATEGORY_DANGEROUS",
            Self::Harassment => "HARM_CATEGORY_HARASSMENT",
            Self::Speech => "HARM_CATEGORY_SPEECH",
            Self::Explicit => "HARM_CATEGORY_EXPLICIT",
            Self::DangerousContent => "HARM_CATEGORY_DANGEROUS_CONTENT"
        };
        write!(f, "{}", s)
    }
}

impl FromStr for GeminiSafetyCategory {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HARM_CATEGORY_UNSPECIFIED" => Ok(Self::Unspecified),
            "HARM_CATEGORY_DEROGATORY" => Ok(Self::Derogatory),
            "HARM_CATEGORY_TOXICITY" => Ok(Self::Toxicity),
            "HARM_CATEGORY_VIOLENCE" => Ok(Self::Violence),
            "HARM_CATEGORY_SEXUAL" => Ok(Self::Sexual),
            "HARM_CATEGORY_MEDICAL" => Ok(Self::Medical),
            "HARM_CATEGORY_DANGEROUS" => Ok(Self::Dangerous),
            "HARM_CATEGORY_HARASSMENT" => Ok(Self::Harassment),
            "HARM_CATEGORY_SPEECH" => Ok(Self::Speech),
            "HARM_CATEGORY_EXPLICIT" => Ok(Self::Explicit),
            "HARM_CATEGORY_DANGEROUS_CONTENT" => Ok(Self::DangerousContent),
            _ => Err("undefined harm category"),
        }
    }
}

impl Serialize for GeminiSafetyCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match *self {
            GeminiSafetyCategory::Unspecified => "HARM_CATEGORY_UNSPECIFIED",
            GeminiSafetyCategory::Derogatory => "HARM_CATEGORY_DEROGATORY",
            GeminiSafetyCategory::Toxicity => "HARM_CATEGORY_TOXICITY",
            GeminiSafetyCategory::Violence => "HARM_CATEGORY_VIOLENCE",
            GeminiSafetyCategory::Sexual => "HARM_CATEGORY_SEXUAL",
            GeminiSafetyCategory::Medical => "HARM_CATEGORY_MEDICAL",
            GeminiSafetyCategory::Dangerous => "HARM_CATEGORY_DANGEROUS",
            GeminiSafetyCategory::Harassment => "HARM_CATEGORY_HARASSMENT",
            GeminiSafetyCategory::Speech => "HARM_CATEGORY_SPEECH",
            GeminiSafetyCategory::Explicit => "HARM_CATEGORY_EXPLICIT",
            GeminiSafetyCategory::DangerousContent => "HARM_CATEGORY_DANGEROUS_CONTENT"
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for GeminiSafetyCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeminiSafetyCategoryVisitor;

        impl<'de> Visitor<'de> for GeminiSafetyCategoryVisitor {
            type Value = GeminiSafetyCategory;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a safety category")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "HARM_CATEGORY_UNSPECIFIED" => Ok(GeminiSafetyCategory::Unspecified),
                    "HARM_CATEGORY_DEROGATORY" => Ok(GeminiSafetyCategory::Derogatory),
                    "HARM_CATEGORY_TOXICITY" => Ok(GeminiSafetyCategory::Toxicity),
                    "HARM_CATEGORY_VIOLENCE" => Ok(GeminiSafetyCategory::Violence),
                    "HARM_CATEGORY_SEXUAL" => Ok(GeminiSafetyCategory::Sexual),
                    "HARM_CATEGORY_MEDICAL" => Ok(GeminiSafetyCategory::Medical),
                    "HARM_CATEGORY_DANGEROUS" => Ok(GeminiSafetyCategory::Dangerous),
                    "HARM_CATEGORY_HARASSMENT" => Ok(GeminiSafetyCategory::Harassment),
                    "HARM_CATEGORY_SPEECH" => Ok(GeminiSafetyCategory::Speech),
                    "HARM_CATEGORY_EXPLICIT" => Ok(GeminiSafetyCategory::Explicit),
                    "HARM_CATEGORY_DANGEROUS_CONTENT" => Ok(GeminiSafetyCategory::DangerousContent),
                    _ => Err(E::custom(format!("unknown category: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(GeminiSafetyCategoryVisitor)
    }
}

#[derive(Clone, Debug)]
pub enum GeminiSafetyThreshold {
    Unspecified,
    None,
    OnlyHigh,
    MediumAndAbove,
    LowAndAbove,
}

impl Display for GeminiSafetyThreshold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Unspecified => "HARM_BLOCK_THRESHOLD_UNSPECIFIED",
            Self::None => "BLOCK_NONE",
            Self::OnlyHigh => "BLOCK_ONLY_HIGH",
            Self::MediumAndAbove => "BLOCK_MEDIUM_AND_ABOVE",
            Self::LowAndAbove => "BLOCK_LOW_AND_ABOVE",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for GeminiSafetyThreshold {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HARM_BLOCK_THRESHOLD_UNSPECIFIED" => Ok(Self::Unspecified),
            "BLOCK_NONE" => Ok(Self::None),
            "BLOCK_ONLY_HIGH" => Ok(Self::OnlyHigh),
            "BLOCK_MEDIUM_AND_ABOVE" => Ok(Self::MediumAndAbove),
            "BLOCK_LOW_AND_ABOVE" => Ok(Self::LowAndAbove),
            _ => Err("undefined harm threshold"),
        }
    }
}

impl Serialize for GeminiSafetyThreshold {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
       let s = match *self {
            GeminiSafetyThreshold::Unspecified => "HARM_BLOCK_THRESHOLD_UNSPECIFIED",
            GeminiSafetyThreshold::None => "BLOCK_NONE",
            GeminiSafetyThreshold::OnlyHigh => "BLOCK_ONLY_HIGH",
            GeminiSafetyThreshold::MediumAndAbove => "BLOCK_MEDIUM_AND_ABOVE",
            GeminiSafetyThreshold::LowAndAbove => "BLOCK_LOW_AND_ABOVE",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for GeminiSafetyThreshold {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    { 
        struct GeminiSafetyThresholdVisitor;

        impl<'de> Visitor<'de> for GeminiSafetyThresholdVisitor {
            type Value = GeminiSafetyThreshold;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a safety threshold")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "HARM_BLOCK_THRESHOLD_UNSPECIFIED" => Ok(GeminiSafetyThreshold::Unspecified),
                    "BLOCK_NONE" => Ok(GeminiSafetyThreshold::None),
                    "BLOCK_ONLY_HIGH" => Ok(GeminiSafetyThreshold::OnlyHigh),
                    "BLOCK_MEDIUM_AND_ABOVE" => Ok(GeminiSafetyThreshold::MediumAndAbove),
                    "BLOCK_LOW_AND_ABOVE" => Ok(GeminiSafetyThreshold::LowAndAbove),
                    _ => Err(E::custom(format!("unknown threshold: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(GeminiSafetyThresholdVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiPostBodySafetySettings {
    category: GeminiSafetyCategory,
    threshold: GeminiSafetyThreshold
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiPostBodyGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    candidate_count: Option<u8>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiPostBody {
    #[serde(default)]
    pub contents: Vec<GeminiPostBodyParts>,
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