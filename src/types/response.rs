use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Response from the OpenAI Responses API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    /// Unique identifier for the response
    pub id: String,

    /// The model used to generate the response
    pub model: String,

    /// The output items generated by the model
    pub output: Vec<crate::types::ResponseItem>,

    /// Optional ID of the previous response in the conversation
    pub previous_response_id: Option<String>,

    /// Creation timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,

    /// Optional metadata associated with the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Response {
    /// Returns the response ID
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the response output as text if available
    #[must_use]
    pub fn output_text(&self) -> String {
        self.output
            .iter()
            .filter_map(|item| match item {
                crate::types::ResponseItem::Message { content, .. } => Some(
                    content
                        .iter()
                        .map(|c| match c {
                            crate::types::MessageContent::OutputText { text, .. } => text.as_str(),
                        })
                        .collect::<String>(),
                ),
                crate::types::ResponseItem::Text { content, .. } => Some(content.clone()),
                _ => None,
            })
            .collect::<String>()
    }

    /// Returns all tool calls in the response
    #[must_use]
    pub fn tool_calls(&self) -> Vec<crate::types::FunctionCallInfo> {
        self.output
            .iter()
            .filter_map(|item| match item {
                crate::types::ResponseItem::FunctionCall {
                    name,
                    arguments,
                    call_id,
                    ..
                } => Some(crate::types::FunctionCallInfo {
                    name: name.clone(),
                    arguments: arguments.clone(),
                    call_id: call_id.clone(),
                }),
                crate::types::ResponseItem::ToolCall(tool_call) => {
                    Some(crate::types::FunctionCallInfo {
                        name: tool_call.name.clone(),
                        arguments: tool_call.arguments.to_string(),
                        call_id: tool_call.id.clone(),
                    })
                }
                _ => None,
            })
            .collect()
    }
}
