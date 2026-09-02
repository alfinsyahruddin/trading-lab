use async_trait::async_trait;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::entities::app_error::AppError;

#[async_trait]
pub trait LLMTrait: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String, AppError>;
}

pub async fn generate_structured<T: DeserializeOwned>(
    llm: &dyn LLMTrait,
    prompt: &str,
) -> Result<T, AppError> {
    let raw = llm.generate(prompt).await?;
    let cleaned = GeminiLLM::clean_json_text(&raw);
    serde_json::from_str::<T>(cleaned).map_err(|e| {
        eprintln!(
            "[LLM] Failed to deserialize JSON into target type: {:?}. Raw text: {}",
            e, raw
        );
        AppError::bad_request(format!(
            "Failed to parse AI output into structured data: {e}"
        ))
    })
}

pub struct GeminiLLM {
    http: Client,
    api_key: String,
    model: String,
}

impl GeminiLLM {
    pub fn new(http: Client, api_key: String, model: String) -> Self {
        Self {
            http,
            api_key,
            model,
        }
    }

    pub fn clean_json_text(raw_text: &str) -> &str {
        let trimmed = raw_text.trim();
        if let Some(stripped) = trimmed.strip_prefix("```json") {
            let inner = stripped.strip_suffix("```").unwrap_or(stripped);
            return inner.trim();
        }
        if let Some(stripped) = trimmed.strip_prefix("```") {
            let inner = stripped.strip_suffix("```").unwrap_or(stripped);
            return inner.trim();
        }
        trimmed
    }
}

#[derive(Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiGenerationConfig {
    response_mime_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiRequestPayload {
    contents: Vec<GeminiContent>,
    generation_config: GeminiGenerationConfig,
}

#[derive(Deserialize)]
struct GeminiCandidatePart {
    text: Option<String>,
}

#[derive(Deserialize)]
struct GeminiCandidateContent {
    parts: Option<Vec<GeminiCandidatePart>>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiCandidateContent>,
}

#[derive(Deserialize)]
struct GeminiResponsePayload {
    candidates: Option<Vec<GeminiCandidate>>,
}

#[async_trait]
impl LLMTrait for GeminiLLM {
    async fn generate(&self, prompt: &str) -> Result<String, AppError> {
        if self.api_key.trim().is_empty() {
            eprintln!("[GeminiLLM] GEMINI_API_KEY is not configured");
            return Err(AppError::bad_request("Gemini API key is not configured"));
        }

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let request_body = GeminiRequestPayload {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart {
                    text: prompt.to_string(),
                }],
            }],
            generation_config: GeminiGenerationConfig {
                response_mime_type: "application/json".to_string(),
            },
        };

        let response = self
            .http
            .post(&url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "TradingLab/1.0")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                eprintln!("[GeminiLLM] HTTP request failed: {:?}", e);
                AppError::Internal
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            eprintln!(
                "[GeminiLLM] Gemini API error (status: {}): {}",
                status, body
            );
            return Err(AppError::bad_request(format!(
                "Gemini API error ({}): {}",
                status, body
            )));
        }

        let payload: GeminiResponsePayload = response.json().await.map_err(|e| {
            eprintln!(
                "[GeminiLLM] Failed to parse Gemini response envelope: {:?}",
                e
            );
            AppError::Internal
        })?;

        let candidate_text = payload
            .candidates
            .as_ref()
            .and_then(|c| c.first())
            .and_then(|c| c.content.as_ref())
            .and_then(|c| c.parts.as_ref())
            .and_then(|p| p.first())
            .and_then(|p| p.text.as_ref())
            .ok_or_else(|| {
                eprintln!("[GeminiLLM] No candidate text received from Gemini API");
                AppError::Internal
            })?;

        Ok(candidate_text.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLLM {
        response: String,
    }

    #[async_trait]
    impl LLMTrait for MockLLM {
        async fn generate(&self, _prompt: &str) -> Result<String, AppError> {
            Ok(self.response.clone())
        }
    }

    #[test]
    fn should_clean_raw_json_and_markdown_fences() {
        assert_eq!(
            GeminiLLM::clean_json_text("```json\n[\"item1\", \"item2\"]\n```"),
            "[\"item1\", \"item2\"]"
        );
        assert_eq!(
            GeminiLLM::clean_json_text("```\n{\"key\":\"value\"}\n```"),
            "{\"key\":\"value\"}"
        );
        assert_eq!(
            GeminiLLM::clean_json_text("{\"key\":\"value\"}"),
            "{\"key\":\"value\"}"
        );
    }

    #[tokio::test]
    async fn should_generate_structured_data_from_mock() {
        let mock = MockLLM {
            response: "```json\n[\"Keypoint 1\", \"Keypoint 2\"]\n```".to_string(),
        };

        let result: Vec<String> = generate_structured(&mock, "test prompt")
            .await
            .expect("deserialization succeeds");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "Keypoint 1");
        assert_eq!(result[1], "Keypoint 2");
    }
}
