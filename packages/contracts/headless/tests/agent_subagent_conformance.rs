//! Cross-runtime conformance for the agent subagent: runs the shared vectors
//! in `vectors/agent-subagent.json` against the Rust mirror. The TypeScript
//! core runs the same vectors (packages/core/test/agent-subagent-conformance.test.ts).
//!
//! The badge wording and the terminal mapping are what the transcript renders,
//! so a divergence would have one target show a child as finished while
//! another still shows it working.

use serde_json::Value;

use poodle_headless::agent_subagent::{
    is_terminal_subagent_status, subagent_status_label, subagent_status_spins, AgentSubagentStatus,
};

fn vectors() -> Value {
    let raw = include_str!("../vectors/agent-subagent.json");
    serde_json::from_str(raw).expect("vectors parse")
}

fn s<'a>(value: &'a Value, key: &str) -> &'a str {
    value.get(key).and_then(Value::as_str).unwrap_or("")
}

#[test]
fn status_labels_match_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["labels"].as_array().expect("label cases") {
        let status = AgentSubagentStatus::from_str_or_default(s(case, "status"));
        assert_eq!(
            subagent_status_label(status),
            s(case, "label"),
            "{}",
            s(case, "status")
        );
    }
}

#[test]
fn terminal_mapping_matches_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["terminal"].as_array().expect("terminal cases") {
        let status = AgentSubagentStatus::from_str_or_default(s(case, "status"));
        assert_eq!(
            is_terminal_subagent_status(status),
            case["terminal"].as_bool().unwrap_or(false),
            "{}",
            s(case, "status")
        );
    }
}

#[test]
fn spinner_mapping_matches_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["spinning"].as_array().expect("spinning cases") {
        let status = AgentSubagentStatus::from_str_or_default(s(case, "status"));
        assert_eq!(
            subagent_status_spins(status),
            case["spinning"].as_bool().unwrap_or(false),
            "{}",
            s(case, "status")
        );
    }
}
