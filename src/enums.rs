use std::{fmt::{self, Display}, str::FromStr};

use serde::{de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer}};

#[derive(Clone, Debug, PartialEq)]
pub enum GeminiHarmCategory {
    Unspecified,
    Derogatory,
    Toxicity,
    Violence,
    Sexual,
    Medical,
    Dangerous,
    Harassment,
    HateSpeech,
    SexuallyExplicit,
    DangerousContent
}

impl Display for GeminiHarmCategory {
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
            Self::HateSpeech => "HARM_CATEGORY_HATE_SPEECH",
            Self::SexuallyExplicit => "HARM_CATEGORY_SEXUALLY_EXPLICIT",
            Self::DangerousContent => "HARM_CATEGORY_DANGEROUS_CONTENT"
        };
        write!(f, "{}", s)
    }
}

impl FromStr for GeminiHarmCategory {
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
            "HARM_CATEGORY_HATE_SPEECH" => Ok(Self::HateSpeech),
            "HARM_CATEGORY_SEXUALLY_EXPLICIT" => Ok(Self::SexuallyExplicit),
            "HARM_CATEGORY_DANGEROUS_CONTENT" => Ok(Self::DangerousContent),
            _ => Err("undefined harm category"),
        }
    }
}

impl Serialize for GeminiHarmCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match *self {
            GeminiHarmCategory::Unspecified => "HARM_CATEGORY_UNSPECIFIED",
            GeminiHarmCategory::Derogatory => "HARM_CATEGORY_DEROGATORY",
            GeminiHarmCategory::Toxicity => "HARM_CATEGORY_TOXICITY",
            GeminiHarmCategory::Violence => "HARM_CATEGORY_VIOLENCE",
            GeminiHarmCategory::Sexual => "HARM_CATEGORY_SEXUAL",
            GeminiHarmCategory::Medical => "HARM_CATEGORY_MEDICAL",
            GeminiHarmCategory::Dangerous => "HARM_CATEGORY_DANGEROUS",
            GeminiHarmCategory::Harassment => "HARM_CATEGORY_HARASSMENT",
            GeminiHarmCategory::HateSpeech => "HARM_CATEGORY_HATE_SPEECH",
            GeminiHarmCategory::SexuallyExplicit => "HARM_CATEGORY_SEXUALLY_EXPLICIT",
            GeminiHarmCategory::DangerousContent => "HARM_CATEGORY_DANGEROUS_CONTENT"
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for GeminiHarmCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeminiHarmCategoryVisitor;

        impl<'de> Visitor<'de> for GeminiHarmCategoryVisitor {
            type Value = GeminiHarmCategory;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a safety category")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "HARM_CATEGORY_UNSPECIFIED" => Ok(GeminiHarmCategory::Unspecified),
                    "HARM_CATEGORY_DEROGATORY" => Ok(GeminiHarmCategory::Derogatory),
                    "HARM_CATEGORY_TOXICITY" => Ok(GeminiHarmCategory::Toxicity),
                    "HARM_CATEGORY_VIOLENCE" => Ok(GeminiHarmCategory::Violence),
                    "HARM_CATEGORY_SEXUAL" => Ok(GeminiHarmCategory::Sexual),
                    "HARM_CATEGORY_MEDICAL" => Ok(GeminiHarmCategory::Medical),
                    "HARM_CATEGORY_DANGEROUS" => Ok(GeminiHarmCategory::Dangerous),
                    "HARM_CATEGORY_HARASSMENT" => Ok(GeminiHarmCategory::Harassment),
                    "HARM_CATEGORY_HATE_SPEECH" => Ok(GeminiHarmCategory::HateSpeech),
                    "HARM_CATEGORY_SEXUALLY_EXPLICIT" => Ok(GeminiHarmCategory::SexuallyExplicit),
                    "HARM_CATEGORY_DANGEROUS_CONTENT" => Ok(GeminiHarmCategory::DangerousContent),
                    _ => Err(E::custom(format!("unknown category: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(GeminiHarmCategoryVisitor)
    }
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum GeminiFinishReason {
    Unspecified,
    Stop,
    MaxTokens,
    Safety,
    Recitation
}

impl Display for GeminiFinishReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Unspecified => "FINISH_REASON_UNSPECIFIED",
            Self::Stop => "STOP",
            Self::MaxTokens => "MAX_TOKENS",
            Self::Safety => "SAFETY",
            Self::Recitation => "RECITATION"
        };
        write!(f, "{}", s)
    }
}

impl FromStr for GeminiFinishReason {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FINISH_REASON_UNSPECIFIED" => Ok(Self::Unspecified),
            "STOP" => Ok(Self::Stop),
            "MAX_TOKENS" => Ok(Self::MaxTokens),
            "SAFETY" => Ok(Self::Safety),
            "RECITATION" => Ok(Self::Recitation),
            _ => Err("undefined finish reason"),
        }
    }
}

impl Serialize for GeminiFinishReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    { 
        let s = match *self {
            GeminiFinishReason::Unspecified => "FINISH_REASON_UNSPECIFIED",
            GeminiFinishReason::Stop => "STOP",
            GeminiFinishReason::MaxTokens => "MAX_TOKENS",
            GeminiFinishReason::Safety => "SAFETY",
            GeminiFinishReason::Recitation => "RECITATION"
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for GeminiFinishReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    { 
        struct GeminiFinishReasonVisitor;

        impl<'de> Visitor<'de> for GeminiFinishReasonVisitor {
            type Value = GeminiFinishReason;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a finish reason")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            { 
                match value {
                    "FINISH_REASON_UNSPECIFIED" => Ok(GeminiFinishReason::Unspecified),
                    "STOP" => Ok(GeminiFinishReason::Stop),
                    "MAX_TOKENS" => Ok(GeminiFinishReason::MaxTokens),
                    "SAFETY" => Ok(GeminiFinishReason::Safety),
                    "RECITATION" => Ok(GeminiFinishReason::Recitation),
                    _ => Err(E::custom(format!("unknown finish reason: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(GeminiFinishReasonVisitor)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GeminiHarmProbability {
    Unspecified,
    Negligible,
    Low,
    Medium,
    High,
}

impl Display for GeminiHarmProbability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Unspecified => "HARM_PROBABILITY_UNSPECIFIED",
            Self::Negligible => "NEGLIGIBLE",
            Self::Low => "LOW",
            Self::Medium => "MEDIUM",
            Self::High => "HIGH",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for GeminiHarmProbability {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HARM_PROBABILITY_UNSPECIFIED" => Ok(Self::Unspecified),
            "NEGLIGIBLE" => Ok(Self::Negligible),
            "LOW" => Ok(Self::Low),
            "MEDIUM" => Ok(Self::Medium),
            "HIGH" => Ok(Self::High),
            _ => Err("undefined harm probability"),
        }
    }
}

impl Serialize for GeminiHarmProbability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    { 
        let s = match *self {
            GeminiHarmProbability::Unspecified => "HARM_PROBABILITY_UNSPECIFIED",
            GeminiHarmProbability::Negligible => "NEGLIGIBLE",
            GeminiHarmProbability::Low => "LOW",
            GeminiHarmProbability::Medium => "MEDIUM",
            GeminiHarmProbability::High => "HIGH",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for GeminiHarmProbability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    { 
        struct GeminiHarmProbabilityVisitor;

        impl<'de> Visitor<'de> for GeminiHarmProbabilityVisitor {
            type Value = GeminiHarmProbability;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a harm probability")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            { 
                match value {
                    "HARM_PROBABILITY_UNSPECIFIED" => Ok(GeminiHarmProbability::Unspecified),
                    "NEGLIGIBLE" => Ok(GeminiHarmProbability::Negligible),
                    "LOW" => Ok(GeminiHarmProbability::Low),
                    "MEDIUM" => Ok(GeminiHarmProbability::Medium),
                    "HIGH" => Ok(GeminiHarmProbability::High),
                    _ => Err(E::custom(format!("unknown harm probability: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(GeminiHarmProbabilityVisitor)
    }
}