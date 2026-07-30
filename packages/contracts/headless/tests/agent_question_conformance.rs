//! Cross-runtime conformance for the agent question: runs the shared vectors in
//! `vectors/agent-question.json` against the Rust mirror. The TypeScript core
//! runs the same vectors.
//!
//! Answer resolution is what the agent receives, so a divergence here would
//! have the desktop build send a different answer from the web one for the same
//! interaction — the worst class of drift in this component.

use serde_json::Value;

use poodle_headless::agent_question::*;

fn vectors() -> Value {
    let raw = include_str!("../vectors/agent-question.json");
    serde_json::from_str(raw).expect("vectors parse")
}

fn s<'a>(value: &'a Value, key: &str) -> &'a str {
    value.get(key).and_then(Value::as_str).unwrap_or("")
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn question_from(value: &Value) -> AgentQuestionItem {
    AgentQuestionItem {
        id: s(value, "id").to_string(),
        header: value.get("header").and_then(Value::as_str).map(str::to_string),
        prompt: s(value, "prompt").to_string(),
        options: value["options"]
            .as_array()
            .map(|entries| {
                entries
                    .iter()
                    .map(|option| AgentQuestionOption {
                        value: s(option, "value").to_string(),
                        label: s(option, "label").to_string(),
                        description: option
                            .get("description")
                            .and_then(Value::as_str)
                            .map(str::to_string),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        allow_multiple: value
            .get("allowMultiple")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    }
}

fn answer_value(answer: Option<&AgentQuestionAnswer>) -> Value {
    match answer {
        None => Value::Null,
        Some(answer) => serde_json::json!({
            "questionId": answer.question_id,
            "outcome": answer.outcome.as_str(),
            "values": answer.values,
            "text": answer.text,
        }),
    }
}

#[test]
fn answer_resolution_matches_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["resolve"].as_array().expect("resolve cases") {
        let name = s(case, "name");
        let question = question_from(&case["q"]);
        let selections = strings(&case["sel"]);
        let text = s(case, "text");

        let got = resolve_question_answer(Some(&question), &selections, text);

        assert_eq!(answer_value(got.as_ref()), case["answer"], "{name}: answer");
        assert_eq!(
            can_submit_question(Some(&question), &selections, text),
            case["canSubmit"].as_bool().unwrap_or(false),
            "{name}: canSubmit"
        );
        assert_eq!(
            submits_on_select(Some(&question)),
            case["submitsOnSelect"].as_bool().unwrap_or(false),
            "{name}: submitsOnSelect"
        );
    }
}

#[test]
fn selection_toggling_matches_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["toggle"].as_array().expect("toggle cases") {
        let name = s(case, "name");
        let question = question_from(&case["q"]);
        let selections = strings(&case["sel"]);

        assert_eq!(
            toggle_question_selection(Some(&question), &selections, s(case, "value")),
            strings(&case["result"]),
            "{name}"
        );
    }
}

#[test]
fn batch_progress_matches_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["progress"].as_array().expect("progress cases") {
        let name = s(case, "name");
        let questions: Vec<AgentQuestionItem> = case["questions"]
            .as_array()
            .expect("questions")
            .iter()
            .map(question_from)
            .collect();
        let active = case["activeIndex"].as_u64().unwrap_or(0) as usize;

        let progress = question_progress(&questions, active);
        let want = &case["progress"];

        assert_eq!(
            progress
                .states
                .iter()
                .map(|state| state.as_str())
                .collect::<Vec<_>>(),
            want["states"]
                .as_array()
                .expect("states")
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>(),
            "{name}: states"
        );
        assert_eq!(
            progress.current as u64,
            want["current"].as_u64().unwrap_or(0),
            "{name}: current"
        );
        assert_eq!(
            progress.total as u64,
            want["total"].as_u64().unwrap_or(0),
            "{name}: total"
        );
        assert_eq!(
            shows_question_progress(&questions),
            case["shows"].as_bool().unwrap_or(false),
            "{name}: shows"
        );
        assert_eq!(
            next_question_index(&questions, active) as u64,
            case["next"].as_u64().unwrap_or(0),
            "{name}: next"
        );
        assert_eq!(
            question_batch_complete(&questions, active),
            case["complete"].as_bool().unwrap_or(false),
            "{name}: complete"
        );
    }
}

#[test]
fn answered_summaries_match_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["summary"].as_array().expect("summary cases") {
        let name = s(case, "name");
        let answer = &case["answer"];
        let question = AgentQuestionItem {
            id: s(answer, "questionId").to_string(),
            options: vec![
                AgentQuestionOption {
                    value: "inline".to_string(),
                    label: "Inline in the transcript".to_string(),
                    description: None,
                },
                AgentQuestionOption {
                    value: "composer".to_string(),
                    label: "Anchored above the composer".to_string(),
                    description: None,
                },
                AgentQuestionOption {
                    value: "modal".to_string(),
                    label: "Modal dialog".to_string(),
                    description: None,
                },
            ],
            ..Default::default()
        };

        let record = AnsweredQuestion {
            question,
            answer: AgentQuestionAnswer {
                question_id: s(answer, "questionId").to_string(),
                outcome: match s(answer, "outcome") {
                    "override" => AgentQuestionOutcome::Override,
                    "declined" => AgentQuestionOutcome::Declined,
                    _ => AgentQuestionOutcome::Selected,
                },
                values: strings(&answer["values"]),
                text: s(answer, "text").to_string(),
            },
        };

        assert_eq!(answered_question_summary(&record), s(case, "summary"), "{name}");
    }
}
