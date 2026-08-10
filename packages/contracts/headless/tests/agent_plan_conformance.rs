//! Cross-runtime conformance for the agent plan: runs the shared vectors in
//! `vectors/agent-plan.json` against the Rust mirror. The TypeScript core runs
//! the same vectors.
//!
//! The decision lifecycle is what the host acts on, so a divergence here would
//! have one target re-decide a plan another target considers settled.

use serde_json::Value;

use poodle_headless::agent_plan::*;

fn vectors() -> Value {
    let raw = include_str!("../vectors/agent-plan.json");
    serde_json::from_str(raw).expect("vectors parse")
}

fn s<'a>(value: &'a Value, key: &str) -> &'a str {
    value.get(key).and_then(Value::as_str).unwrap_or("")
}

#[test]
fn decisions_match_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["decide"].as_array().expect("decide cases") {
        let name = s(case, "name");
        let status = AgentPlanStatus::from_str_or_default(s(case, "status"));
        let next = AgentPlanStatus::from_str_or_default(s(case, "next"));
        let decided_at = case.get("decidedAt").and_then(Value::as_str);

        let got = decide_plan(status, next, decided_at);

        match &case["decision"] {
            Value::Null => assert!(got.is_none(), "{name}: decision should be None"),
            want => {
                let got = got.unwrap_or_else(|| panic!("{name}: expected a decision"));
                assert_eq!(
                    got.status.as_str(),
                    s(want, "status"),
                    "{name}: decision status"
                );
                assert_eq!(
                    got.decided_at.as_deref(),
                    want.get("decidedAt").and_then(Value::as_str),
                    "{name}: decidedAt"
                );
            }
        }

        assert_eq!(
            can_decide_plan(status),
            case["canDecide"].as_bool().unwrap_or(false),
            "{name}: canDecide"
        );
    }
}

#[test]
fn status_labels_match_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["labels"].as_array().expect("label cases") {
        let status = AgentPlanStatus::from_str_or_default(s(case, "status"));
        assert_eq!(
            plan_status_label(status),
            s(case, "label"),
            "{}",
            s(case, "status")
        );
    }
}

#[test]
fn record_summaries_match_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["summary"].as_array().expect("summary cases") {
        let name = s(case, "name");
        assert_eq!(
            plan_record_summary(
                s(case, "plan"),
                case["maxLength"].as_u64().unwrap_or(0) as usize
            ),
            s(case, "summary"),
            "{name}"
        );
    }
}
