use serde_json::{Map, Value};

pub const MAX_PROVIDER_RECORD_BYTES: usize = 256 * 1024;
pub const PROVIDER_RECORD_OVERSIZE_MARKER: &str = "[PROVIDER RECORD TRUNCATED]";
pub const REDACTED_SENSITIVE_VALUE: &str = "[REDACTED SENSITIVE VALUE]";

const SENSITIVE_KEYS: [&str; 9] = [
    "api_key",
    "apikey",
    "access_token",
    "refresh_token",
    "token",
    "password",
    "authorization",
    "secret",
    "client_secret",
];

#[derive(Default)]
pub struct StreamRedactor {
    carry: Vec<u8>,
    oversize: bool,
    discard_until_newline: bool,
}

impl StreamRedactor {
    #[cfg(test)]
    pub fn buffered_bytes(&self) -> usize {
        self.carry.len()
    }

    pub fn push(&mut self, bytes: &[u8]) -> Vec<String> {
        let mut output = Vec::new();
        let mut offset = 0;
        while offset < bytes.len() {
            if self.discard_until_newline {
                let Some(relative) = bytes[offset..].iter().position(|byte| *byte == b'\n') else {
                    break;
                };
                offset += relative + 1;
                self.discard_until_newline = false;
                output.push(self.finish_record());
                continue;
            }

            let newline = bytes[offset..].iter().position(|byte| *byte == b'\n');
            let end = newline
                .map(|relative| offset + relative + 1)
                .unwrap_or(bytes.len());
            self.append_bounded(&bytes[offset..end]);
            offset = end;

            if newline.is_some() {
                output.push(self.finish_record());
            }
            if self.discard_until_newline {
                continue;
            }
            if end == bytes.len() {
                break;
            }
        }
        output
    }

    pub fn finish(&mut self) -> Vec<String> {
        if self.carry.is_empty() {
            self.oversize = false;
            self.discard_until_newline = false;
            return Vec::new();
        }
        vec![self.finish_record()]
    }

    fn append_bounded(&mut self, bytes: &[u8]) {
        let remaining = MAX_PROVIDER_RECORD_BYTES.saturating_sub(self.carry.len());
        let take = bytes.len().min(remaining);
        self.carry.extend_from_slice(&bytes[..take]);
        if take < bytes.len() {
            self.oversize = true;
            self.discard_until_newline = true;
        }
    }

    fn finish_record(&mut self) -> String {
        let mut rendered = sanitize_record(&std::mem::take(&mut self.carry));
        if self.oversize {
            let newline = rendered.ends_with('\n');
            if newline {
                rendered.pop();
                if rendered.ends_with('\r') {
                    rendered.pop();
                }
            }
            rendered.push('\n');
            rendered.push_str(PROVIDER_RECORD_OVERSIZE_MARKER);
            if newline {
                rendered.push('\n');
            }
        }
        self.oversize = false;
        self.discard_until_newline = false;
        rendered
    }
}

pub fn sanitize_record(record: &[u8]) -> String {
    let text = String::from_utf8_lossy(record);
    let has_newline = text.ends_with('\n');
    let content = text.trim_end_matches(['\r', '\n']);
    let sanitized = serde_json::from_str::<Value>(content)
        .map(|value| sanitize_json_value(value, None))
        .ok()
        .and_then(|value| serde_json::to_string(&value).ok())
        .unwrap_or_else(|| sanitize_plain_text(content));
    if has_newline {
        format!("{sanitized}\n")
    } else {
        sanitized
    }
}

pub fn has_meaningful_provider_output(text: &str) -> bool {
    text.lines().any(|line| {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.contains(PROVIDER_RECORD_OVERSIZE_MARKER) {
            return false;
        }
        if let Ok(value) = serde_json::from_str::<Value>(trimmed) {
            return has_meaningful_json_text(&value, false);
        }
        !trimmed.starts_with("SESSION_")
            && !trimmed.starts_with("PROCESS_")
            && !trimmed.starts_with("STREAM_")
            && !trimmed.starts_with("SYSTEM_")
            && !trimmed.starts_with("[REDACTED")
    })
}

fn has_meaningful_json_text(value: &Value, provider_result: bool) -> bool {
    match value {
        Value::Object(object) => {
            let result_context = provider_result
                || object
                    .get("type")
                    .and_then(Value::as_str)
                    .is_some_and(|kind| {
                        matches!(
                            kind,
                            "assistant" | "result" | "message" | "content_block_delta"
                        )
                    });
            object.iter().any(|(key, child)| {
                let child_context =
                    result_context || matches!(key.as_str(), "result" | "message" | "content");
                has_meaningful_json_text(child, child_context)
            })
        }
        Value::Array(values) => values
            .iter()
            .any(|value| has_meaningful_json_text(value, provider_result)),
        Value::String(value) => provider_result && !value.trim().is_empty(),
        _ => false,
    }
}

fn sanitize_json_value(value: Value, key: Option<&str>) -> Value {
    match value {
        Value::Object(object) => Value::Object(
            object
                .into_iter()
                .map(|(name, value)| {
                    let next = if is_sensitive_key(&name) {
                        Value::String(REDACTED_SENSITIVE_VALUE.into())
                    } else {
                        sanitize_json_value(value, Some(&name))
                    };
                    (name, next)
                })
                .collect::<Map<String, Value>>(),
        ),
        Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(|value| sanitize_json_value(value, key))
                .collect(),
        ),
        Value::String(value) => {
            if key.is_some_and(is_sensitive_key) {
                Value::String(REDACTED_SENSITIVE_VALUE.into())
            } else {
                Value::String(sanitize_plain_text(&value))
            }
        }
        other => other,
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let normalized = key.to_ascii_lowercase().replace('-', "_");
    SENSITIVE_KEYS
        .iter()
        .any(|candidate| *candidate == normalized)
}

fn sanitize_plain_text(value: &str) -> String {
    let mut sanitized = value.to_string();
    for key in SENSITIVE_KEYS {
        sanitized = redact_assignment(&sanitized, key);
    }
    sanitized = redact_marker_span(&sanitized, "bearer ", false);
    redact_marker_span(&sanitized, "sk-", true)
}

pub fn sanitize_text(value: &str) -> String {
    sanitize_plain_text(value)
}

fn redact_assignment(source: &str, key: &str) -> String {
    let lower = source.to_ascii_lowercase();
    let mut result = source.to_string();
    let mut cursor = 0;
    while let Some(relative) = lower[cursor..].find(key) {
        let start = cursor + relative;
        let end = start + key.len();
        let boundary_before = start == 0
            || !lower.as_bytes()[start - 1].is_ascii_alphanumeric()
                && lower.as_bytes()[start - 1] != b'_';
        let boundary_after = end == lower.len()
            || !lower.as_bytes()[end].is_ascii_alphanumeric() && lower.as_bytes()[end] != b'_';
        let mut value_start = end;
        while value_start < lower.len() && lower.as_bytes()[value_start].is_ascii_whitespace() {
            value_start += 1;
        }
        let assignment =
            value_start < lower.len() && matches!(lower.as_bytes()[value_start], b'=' | b':');
        if !boundary_before || !boundary_after || !assignment {
            cursor = end;
            continue;
        }
        value_start += 1;
        while value_start < lower.len() && lower.as_bytes()[value_start].is_ascii_whitespace() {
            value_start += 1;
        }
        let bearer = lower[value_start..].starts_with("bearer ");
        if bearer {
            value_start += "bearer ".len();
        }
        let quoted = !bearer
            && (lower.as_bytes().get(value_start) == Some(&b'"')
                || lower.as_bytes().get(value_start) == Some(&b'\''));
        if quoted {
            value_start += 1;
        }
        let mut value_end = value_start;
        while value_end < lower.len() {
            let byte = lower.as_bytes()[value_end];
            if (quoted && (byte == b'"' || byte == b'\''))
                || (!quoted && (byte.is_ascii_whitespace() || b",;}\n".contains(&byte)))
            {
                break;
            }
            value_end += 1;
        }
        if value_end > value_start {
            if source[value_start..].starts_with(REDACTED_SENSITIVE_VALUE) {
                cursor = value_start + REDACTED_SENSITIVE_VALUE.len();
                continue;
            }
            result.replace_range(value_start..value_end, REDACTED_SENSITIVE_VALUE);
            return redact_assignment(&result, key);
        }
        cursor = end;
    }
    result
}

fn redact_marker_span(source: &str, marker: &str, include_marker: bool) -> String {
    let mut result = source.to_string();
    let marker_lower = marker.to_ascii_lowercase();
    let mut cursor = 0;
    loop {
        let lower = result.to_ascii_lowercase();
        let Some(relative) = lower[cursor..].find(&marker_lower) else {
            break;
        };
        let marker_start = cursor + relative;
        let value_start = if include_marker {
            marker_start
        } else {
            marker_start + marker.len()
        };
        if result[value_start..].starts_with(REDACTED_SENSITIVE_VALUE) {
            cursor = value_start + REDACTED_SENSITIVE_VALUE.len();
            continue;
        }
        let mut value_end = value_start;
        while value_end < lower.len()
            && !lower.as_bytes()[value_end].is_ascii_whitespace()
            && !b"\"',;}])".contains(&lower.as_bytes()[value_end])
        {
            value_end += 1;
        }
        if value_end <= value_start {
            cursor = marker_start + marker.len();
            continue;
        }
        result.replace_range(value_start..value_end, REDACTED_SENSITIVE_VALUE);
        cursor = value_start + REDACTED_SENSITIVE_VALUE.len();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn selective_json_sanitization_preserves_metadata_and_assistant_text() {
        let record = json!({
            "type": "assistant",
            "message": {"role": "assistant", "content": [{"type": "text", "text": "A secret filename secret.txt and token usage remain readable."}]},
            "input_tokens": 12,
            "output_tokens": 34,
            "thinking_tokens": 5,
            "rate_limit_info": {"status": "allowed"},
            "model": "claude-sonnet",
            "filename": "secret.txt",
            "api_key": "sk-live-value",
            "access_token": "access-value",
            "refresh_token": "refresh-value",
            "token": "credential-value",
            "password": "password-value",
            "authorization": "Bearer bearer-value",
            "secret": "secret-value",
            "client_secret": "client-value"
        });
        let sanitized: Value =
            serde_json::from_str(&sanitize_record(format!("{record}\n").as_bytes())).unwrap();
        assert_eq!(sanitized["input_tokens"], 12);
        assert_eq!(sanitized["output_tokens"], 34);
        assert_eq!(sanitized["thinking_tokens"], 5);
        assert_eq!(sanitized["rate_limit_info"]["status"], "allowed");
        assert_eq!(sanitized["model"], "claude-sonnet");
        assert_eq!(sanitized["filename"], "secret.txt");
        assert!(sanitized["message"].to_string().contains("secret.txt"));
        assert!(!sanitized.to_string().contains("sk-live-value"));
        assert!(!sanitized.to_string().contains("bearer-value"));
        assert_eq!(sanitized["api_key"], REDACTED_SENSITIVE_VALUE);
    }

    #[test]
    fn plain_text_masks_credentials_without_erasing_token_usage() {
        let output =
            sanitize_record(b"token usage: input_tokens=12 output_tokens=34 api_key=sk-abc123\n");
        assert!(output.contains("input_tokens=12"));
        assert!(output.contains("output_tokens=34"));
        assert!(output.contains("api_key=[REDACTED SENSITIVE VALUE]"));
        assert!(!output.contains("sk-abc123"));
    }

    #[test]
    fn split_credentials_and_large_normal_records_survive_bounded_streaming() {
        let mut redactor = StreamRedactor::default();
        let record = format!(
            "{{\"type\":\"assistant\",\"message\":{{\"text\":\"{}\"}},\"authorization\":\"Bearer secret-value\"}}\n",
            "assistant answer ".to_string() + &"x".repeat(8000)
        );
        let mut output = Vec::new();
        for chunk in record.as_bytes().chunks(17) {
            output.extend(redactor.push(chunk));
        }
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("assistant answer"));
        assert!(!output[0].contains("secret-value"));
        assert!(!output[0].contains("REDACTED SENSITIVE OUTPUT"));
    }

    #[test]
    fn over_bound_record_reports_truncation_instead_of_silent_discard() {
        let mut redactor = StreamRedactor::default();
        let mut record = vec![b'a'; MAX_PROVIDER_RECORD_BYTES + 10];
        record.extend_from_slice(b"\n");
        let output = redactor.push(&record);
        assert_eq!(output.len(), 1);
        assert!(output[0].contains(PROVIDER_RECORD_OVERSIZE_MARKER));
        assert!(!output[0].contains("REDACTED SENSITIVE OUTPUT"));
    }
}
