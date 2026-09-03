use crate::stream_sanitizer::sanitize_text;
use serde_json::Value;

pub const MAX_FINAL_RESPONSE_BYTES: usize = 256 * 1024;
pub const FINAL_RESPONSE_TRUNCATED_MARKER: &str = "[FINAL ASSISTANT RESPONSE TRUNCATED]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    Claude,
    Codex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinalResponseState {
    Unavailable,
    Available,
    Truncated,
}

impl FinalResponseState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unavailable => "UNAVAILABLE",
            Self::Available => "AVAILABLE",
            Self::Truncated => "TRUNCATED",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FinalResponseCapture {
    text: String,
    truncated: bool,
}

impl FinalResponseCapture {
    pub fn observe(&mut self, provider: ProviderKind, record: &str) {
        let Ok(value) = serde_json::from_str::<Value>(record.trim()) else {
            return;
        };
        let candidate = match provider {
            ProviderKind::Claude => claude_final_text(&value),
            ProviderKind::Codex => codex_final_text(&value),
        };
        let Some(candidate) = candidate else {
            return;
        };
        let candidate = sanitize_text(candidate.trim());
        if candidate.trim().is_empty() || self.text == candidate {
            return;
        }
        if !self.text.is_empty() {
            self.text.push_str("\n\n");
        }
        self.text.push_str(&candidate);
        self.bound_text();
    }

    pub fn text(&self) -> Option<&str> {
        (!self.text.trim().is_empty()).then_some(self.text.as_str())
    }

    pub fn state(&self) -> FinalResponseState {
        if self.text().is_none() {
            FinalResponseState::Unavailable
        } else if self.truncated {
            FinalResponseState::Truncated
        } else {
            FinalResponseState::Available
        }
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    fn bound_text(&mut self) {
        if self.text.len() <= MAX_FINAL_RESPONSE_BYTES {
            return;
        }
        let marker_bytes = FINAL_RESPONSE_TRUNCATED_MARKER.len() + 1;
        let content_limit = MAX_FINAL_RESPONSE_BYTES.saturating_sub(marker_bytes);
        let mut end = content_limit.min(self.text.len());
        while end > 0 && !self.text.is_char_boundary(end) {
            end -= 1;
        }
        self.text.truncate(end);
        self.text.push('\n');
        self.text.push_str(FINAL_RESPONSE_TRUNCATED_MARKER);
        self.truncated = true;
    }
}

fn claude_final_text(value: &Value) -> Option<&str> {
    let object = value.as_object()?;
    if object.get("type").and_then(Value::as_str) != Some("result") {
        return None;
    }
    object.get("result").and_then(Value::as_str)
}

fn codex_final_text(value: &Value) -> Option<&str> {
    let object = value.as_object()?;
    let kind = object
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if kind == "result" || kind == "response.completed" {
        return object
            .get("result")
            .and_then(Value::as_str)
            .or_else(|| object.get("output_text").and_then(Value::as_str));
    }
    if kind == "item.completed" {
        let item = object.get("item")?.as_object()?;
        let item_kind = item.get("type").and_then(Value::as_str).unwrap_or_default();
        if matches!(item_kind, "agent_message" | "assistant_message") {
            return item
                .get("text")
                .and_then(Value::as_str)
                .or_else(|| item.get("content").and_then(Value::as_str));
        }
    }
    if kind == "message"
        && object.get("role").and_then(Value::as_str) == Some("assistant")
        && object.get("status").and_then(Value::as_str) == Some("completed")
    {
        return object
            .get("text")
            .and_then(Value::as_str)
            .or_else(|| object.get("content").and_then(Value::as_str));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn claude_uses_terminal_result_and_ignores_intermediate_assistant() {
        let mut capture = FinalResponseCapture::default();
        capture.observe(
            ProviderKind::Claude,
            &json!({"type":"assistant","message":{"content":[{"text":"I'll inspect the repository."}]}}).to_string(),
        );
        assert_eq!(capture.state(), FinalResponseState::Unavailable);
        capture.observe(
            ProviderKind::Claude,
            &json!({"type":"result","subtype":"success","result":"The repository contains the desktop shell."}).to_string(),
        );
        assert_eq!(
            capture.text(),
            Some("The repository contains the desktop shell.")
        );
    }

    #[test]
    fn codex_uses_completed_agent_message_not_progress_item() {
        let mut capture = FinalResponseCapture::default();
        capture.observe(
            ProviderKind::Codex,
            &json!({"type":"item.started","item":{"type":"agent_message","text":"I am exploring..."}}).to_string(),
        );
        assert_eq!(capture.state(), FinalResponseState::Unavailable);
        capture.observe(
            ProviderKind::Codex,
            &json!({"type":"item.completed","item":{"type":"agent_message","text":"The repository contains the desktop shell."}}).to_string(),
        );
        assert_eq!(
            capture.text(),
            Some("The repository contains the desktop shell.")
        );
    }

    #[test]
    fn final_response_is_utf8_safe_and_independent_of_generic_output_cap() {
        let mut capture = FinalResponseCapture::default();
        let answer = "é".repeat(MAX_FINAL_RESPONSE_BYTES);
        capture.observe(
            ProviderKind::Claude,
            &json!({"type":"result","result":answer}).to_string(),
        );
        assert_eq!(capture.state(), FinalResponseState::Truncated);
        assert!(capture
            .text()
            .unwrap()
            .is_char_boundary(capture.text().unwrap().len()));
        assert!(capture
            .text()
            .unwrap()
            .contains(FINAL_RESPONSE_TRUNCATED_MARKER));
    }

    #[test]
    fn final_response_sanitizes_credentials_but_preserves_meaningful_text() {
        let mut capture = FinalResponseCapture::default();
        capture.observe(
            ProviderKind::Claude,
            &json!({"type":"result","result":"Done. api_key=sk-secret-value"}).to_string(),
        );
        let text = capture.text().unwrap();
        assert!(text.contains("Done."));
        assert!(!text.contains("sk-secret-value"));
    }
}
