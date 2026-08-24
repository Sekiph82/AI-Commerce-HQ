use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuntimeState {
    Stopped,
    Starting,
    Healthy,
    Degraded,
    Stopping,
    Failed,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuntimeHealth {
    Healthy,
    Degraded,
    Failed,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuntimeComponentKind {
    NativeCore,
    Sidecar,
    LegacyCommerce,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeComponent {
    pub component_id: String,
    pub display_name: String,
    pub kind: RuntimeComponentKind,
    pub state: RuntimeState,
    pub health: RuntimeHealth,
    pub started_at: Option<String>,
    pub last_heartbeat: Option<String>,
    pub restart_count: u32,
    pub last_error: Option<String>,
    pub ownership: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeStatus {
    pub architecture_mode: String,
    pub sidecar_enabled: bool,
    pub legacy_commerce_runtime: RuntimeComponent,
    pub components: Vec<RuntimeComponent>,
    pub last_error: Option<String>,
}

#[derive(Debug)]
pub struct RuntimeSupervisor {
    started_at: String,
}

impl RuntimeSupervisor {
    pub fn new() -> Self {
        Self {
            started_at: unix_timestamp(),
        }
    }

    pub fn status(&self) -> RuntimeStatus {
        let native_core = RuntimeComponent {
            component_id: "hiveai-native-core".into(),
            display_name: "H!veAI native core".into(),
            kind: RuntimeComponentKind::NativeCore,
            state: RuntimeState::Healthy,
            health: RuntimeHealth::Healthy,
            started_at: Some(self.started_at.clone()),
            last_heartbeat: None,
            restart_count: 0,
            last_error: None,
            ownership: "H!veAI Rust native core".into(),
        };
        let legacy = disabled_legacy_component();

        RuntimeStatus {
            architecture_mode: "RUST_NATIVE_NO_SIDECAR".into(),
            sidecar_enabled: false,
            legacy_commerce_runtime: legacy.clone(),
            components: vec![native_core, legacy],
            last_error: None,
        }
    }
}

pub fn disabled_legacy_component() -> RuntimeComponent {
    RuntimeComponent {
        component_id: "legacy-ai-commerce-runtime".into(),
        display_name: "Legacy AI-Commerce-HQ runtime".into(),
        kind: RuntimeComponentKind::LegacyCommerce,
        state: RuntimeState::Disabled,
        health: RuntimeHealth::Disabled,
        started_at: None,
        last_heartbeat: None,
        restart_count: 0,
        last_error: None,
        ownership: "Excluded from H!veAI startup".into(),
    }
}

pub fn valid_transition(from: RuntimeState, to: RuntimeState) -> bool {
    matches!(
        (from, to),
        (RuntimeState::Stopped, RuntimeState::Starting)
            | (RuntimeState::Starting, RuntimeState::Healthy)
            | (RuntimeState::Starting, RuntimeState::Failed)
            | (RuntimeState::Starting, RuntimeState::Stopping)
            | (RuntimeState::Healthy, RuntimeState::Degraded)
            | (RuntimeState::Healthy, RuntimeState::Stopping)
            | (RuntimeState::Degraded, RuntimeState::Healthy)
            | (RuntimeState::Degraded, RuntimeState::Failed)
            | (RuntimeState::Degraded, RuntimeState::Stopping)
            | (RuntimeState::Failed, RuntimeState::Starting)
            | (RuntimeState::Failed, RuntimeState::Stopped)
            | (RuntimeState::Stopping, RuntimeState::Stopped)
            | (RuntimeState::Disabled, RuntimeState::Disabled)
    )
}

pub fn restart_backoff_ms(restart_count: u32) -> u64 {
    1_000_u64
        .saturating_mul(2_u64.saturating_pow(restart_count.min(5)))
        .min(30_000)
}

pub fn sanitize_error(error: &str) -> String {
    let lower = error.to_ascii_lowercase();
    if ["secret", "token", "password", "api_key", "private key"]
        .iter()
        .any(|term| lower.contains(term))
    {
        return "Runtime error redacted.".into();
    }

    error
        .lines()
        .next()
        .unwrap_or("Unknown runtime error")
        .chars()
        .take(240)
        .collect()
}

fn unix_timestamp() -> String {
    crate::time::utc_timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_status_serializes_truthful_native_boundary() {
        let status = RuntimeSupervisor::new().status();
        let json = serde_json::to_string(&status).expect("runtime status should serialize");
        assert!(json.contains("RUST_NATIVE_NO_SIDECAR"));
        assert!(json.contains("\"state\":\"DISABLED\""));
        assert!(!status.sidecar_enabled);
    }

    #[test]
    fn state_transitions_are_explicit() {
        assert!(valid_transition(
            RuntimeState::Stopped,
            RuntimeState::Starting
        ));
        assert!(valid_transition(
            RuntimeState::Starting,
            RuntimeState::Healthy
        ));
        assert!(!valid_transition(
            RuntimeState::Healthy,
            RuntimeState::Starting
        ));
    }

    #[test]
    fn legacy_component_is_disabled() {
        let legacy = disabled_legacy_component();
        assert_eq!(legacy.state, RuntimeState::Disabled);
        assert_eq!(legacy.health, RuntimeHealth::Disabled);
        assert_eq!(legacy.restart_count, 0);
    }

    #[test]
    fn restart_backoff_is_bounded() {
        assert_eq!(restart_backoff_ms(0), 1_000);
        assert_eq!(restart_backoff_ms(4), 16_000);
        assert_eq!(restart_backoff_ms(99), 30_000);
    }

    #[test]
    fn sensitive_errors_are_sanitized() {
        assert_eq!(
            sanitize_error("request failed with API_KEY=hidden"),
            "Runtime error redacted."
        );
        assert_eq!(sanitize_error("first line\nprivate details"), "first line");
    }
}
