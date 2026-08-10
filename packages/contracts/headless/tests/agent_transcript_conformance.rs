//! Cross-runtime conformance for the agent transcript: runs the shared vectors
//! in `vectors/agent-transcript.json` against the Rust mirror. The TypeScript
//! core runs the same vectors (packages/core/test/agent-transcript-conformance.test.ts).
//!
//! Grouping decides what the reader sees collapsed, so a divergence between web
//! and native would show up as the desktop build summarising a turn differently
//! from the web one — with nothing failing anywhere.

use serde_json::Value;

use poodle_headless::agent_transcript::*;

fn vectors() -> Value {
    let raw = include_str!("../vectors/agent-transcript.json");
    serde_json::from_str(raw).expect("vectors parse")
}

fn s<'a>(value: &'a Value, key: &str) -> &'a str {
    value.get(key).and_then(Value::as_str).unwrap_or("")
}

fn u(value: &Value, key: &str) -> u32 {
    value.get(key).and_then(Value::as_u64).unwrap_or(0) as u32
}

fn f(value: &Value, key: &str) -> f64 {
    value.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn item_from(value: &Value) -> TranscriptItem {
    match s(value, "kind") {
        "tool-call" => TranscriptItem::ToolCall(TranscriptToolCall {
            id: s(value, "id").to_string(),
            label: s(value, "label").to_string(),
            detail: value
                .get("detail")
                .and_then(Value::as_str)
                .map(str::to_string),
            status: ToolCallStatus::from_str(s(value, "status")),
            icon: value
                .get("icon")
                .and_then(Value::as_str)
                .map(str::to_string),
            output: value
                .get("output")
                .and_then(Value::as_str)
                .map(str::to_string),
        }),
        "changed-files" => TranscriptItem::ChangedFiles(TranscriptChangedFiles {
            id: s(value, "id").to_string(),
            files: value["files"]
                .as_array()
                .map(|entries| {
                    entries
                        .iter()
                        .map(|file| ChangedFile {
                            path: s(file, "path").to_string(),
                            additions: u(file, "additions"),
                            deletions: u(file, "deletions"),
                            status: None,
                        })
                        .collect()
                })
                .unwrap_or_default(),
        }),
        "activity" => TranscriptItem::Activity(TranscriptActivity {
            id: s(value, "id").to_string(),
            label: s(value, "label").to_string(),
            spinning: None,
        }),
        "decided-plan" => TranscriptItem::DecidedPlan(TranscriptDecidedPlan {
            id: s(value, "id").to_string(),
            plan: s(value, "plan").to_string(),
            status: poodle_headless::agent_plan::AgentPlanStatus::from_str_or_default(s(
                value, "status",
            )),
            decided_at: value
                .get("decidedAt")
                .and_then(Value::as_str)
                .map(str::to_string),
        }),
        "subagent-group" => TranscriptItem::SubagentGroup(TranscriptSubagentGroup {
            id: s(value, "id").to_string(),
            subagent: poodle_headless::agent_subagent::AgentSubagentItem {
                id: s(&value["subagent"], "id").to_string(),
                label: s(&value["subagent"], "label").to_string(),
                status: poodle_headless::agent_subagent::AgentSubagentStatus::from_str_or_default(
                    s(&value["subagent"], "status"),
                ),
                activity_line: value["subagent"]
                    .get("activityLine")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                summary: value["subagent"]
                    .get("summary")
                    .and_then(Value::as_str)
                    .map(str::to_string),
            },
            detail_lines: value
                .get("detailLines")
                .and_then(Value::as_array)
                .map(|lines| {
                    lines
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect()
                }),
        }),
        _ => TranscriptItem::Message(TranscriptMessage {
            id: s(value, "id").to_string(),
            role: match s(value, "role") {
                "user" => Some(TranscriptRole::User),
                _ => Some(TranscriptRole::Assistant),
            },
            markdown: s(value, "markdown").to_string(),
            is_streaming: value
                .get("isStreaming")
                .and_then(Value::as_bool)
                .unwrap_or(false),
        }),
    }
}

#[test]
fn grouping_matches_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["grouping"].as_array().expect("grouping cases") {
        let name = s(case, "name");
        let items: Vec<TranscriptItem> = case["items"]
            .as_array()
            .expect("items")
            .iter()
            .map(item_from)
            .collect();

        let blocks = group_transcript_items(&items);
        let expected = case["blocks"].as_array().expect("blocks");

        assert_eq!(
            blocks.len(),
            expected.len(),
            "{name}: block count differs ({:?})",
            blocks.iter().map(TranscriptBlock::kind).collect::<Vec<_>>()
        );

        for (block, want) in blocks.iter().zip(expected) {
            assert_eq!(block.kind(), s(want, "kind"), "{name}: block kind");
            assert_eq!(block.id(), s(want, "id"), "{name}: block id");

            match block {
                TranscriptBlock::ToolRun(run) => {
                    let want_ids: Vec<&str> = want["callIds"]
                        .as_array()
                        .expect("callIds")
                        .iter()
                        .map(|v| v.as_str().unwrap_or(""))
                        .collect();
                    let got_ids: Vec<&str> = run.calls.iter().map(|c| c.id.as_str()).collect();

                    assert_eq!(got_ids, want_ids, "{name}: run membership");
                    assert_eq!(
                        run.lead_call().map(|c| c.id.as_str()).unwrap_or(""),
                        s(want, "leadCallId"),
                        "{name}: collapsed run shows the newest call"
                    );
                    assert_eq!(
                        run.hidden_count() as u64,
                        want["hiddenCount"].as_u64().unwrap_or(0),
                        "{name}: hidden count"
                    );
                    assert_eq!(
                        run.status().as_str(),
                        s(want, "status"),
                        "{name}: run status"
                    );
                }
                TranscriptBlock::ChangedFiles(changed) => {
                    let totals = changed_files_totals(&changed.files);
                    let want_totals = &want["totals"];

                    assert_eq!(
                        totals.file_count as u64,
                        want_totals["fileCount"].as_u64().unwrap_or(0),
                        "{name}: file count"
                    );
                    assert_eq!(
                        totals.additions,
                        u(want_totals, "additions"),
                        "{name}: additions"
                    );
                    assert_eq!(
                        totals.deletions,
                        u(want_totals, "deletions"),
                        "{name}: deletions"
                    );
                }
                _ => {}
            }
        }
    }
}

#[test]
fn windowing_matches_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["windowing"].as_array().expect("windowing cases") {
        let name = s(case, "name");
        let heights: Vec<f64> = case["heights"]
            .as_array()
            .expect("heights")
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0))
            .collect();

        let got = transcript_window(
            &heights,
            f(case, "estimated"),
            f(case, "scrollTop"),
            f(case, "viewport"),
            case["overscan"].as_u64().unwrap_or(0) as usize,
        );
        let want = &case["window"];

        assert_eq!(
            got.start_index as u64,
            want["startIndex"].as_u64().unwrap_or(0),
            "{name}: startIndex"
        );
        assert_eq!(
            got.end_index as u64,
            want["endIndex"].as_u64().unwrap_or(0),
            "{name}: endIndex"
        );
        assert_eq!(got.offset_y, f(want, "offsetY"), "{name}: offsetY");
        assert_eq!(
            got.total_height,
            f(want, "totalHeight"),
            "{name}: totalHeight"
        );
    }
}

#[test]
fn bottom_anchoring_matches_the_shared_vectors() {
    let vectors = vectors();

    for case in vectors["pinned"].as_array().expect("pinned cases") {
        let name = s(case, "name");

        assert_eq!(
            is_pinned_to_bottom(
                f(case, "scrollTop"),
                f(case, "scrollHeight"),
                f(case, "clientHeight"),
                f(case, "threshold"),
            ),
            case["pinned"].as_bool().unwrap_or(false),
            "{name}"
        );
    }
}

fn tree_vectors() -> Value {
    let raw = include_str!("../vectors/changed-file-tree.json");
    serde_json::from_str(raw).expect("tree vectors parse")
}

fn node_value(node: &ChangedFileNode) -> Value {
    serde_json::json!({
        "path": node.path,
        "label": node.label,
        "isDirectory": node.is_directory,
        "additions": node.additions,
        "deletions": node.deletions,
        "children": node.children.iter().map(node_value).collect::<Vec<_>>(),
    })
}

/// Folding decides what the expanded card looks like, and a divergence would
/// show as the desktop build drawing an indentation staircase where the web one
/// draws a single collapsed row.
#[test]
fn changed_file_tree_matches_the_shared_vectors() {
    for case in tree_vectors().as_array().expect("tree cases") {
        let name = s(case, "name");
        let files: Vec<ChangedFile> = case["files"]
            .as_array()
            .expect("files")
            .iter()
            .map(|f| ChangedFile {
                path: s(f, "path").to_string(),
                additions: u(f, "additions"),
                deletions: u(f, "deletions"),
                status: None,
            })
            .collect();

        let got = Value::Array(
            build_changed_file_tree(&files)
                .iter()
                .map(node_value)
                .collect(),
        );
        assert_eq!(got, case["tree"], "{name}: tree folding diverged");

        let scopes = Value::Array(
            changed_file_scopes(&files)
                .into_iter()
                .map(|(name, count)| serde_json::json!({ "name": name, "fileCount": count }))
                .collect(),
        );
        assert_eq!(scopes, case["scopes"], "{name}: scopes diverged");
    }
}
