//! Update-status conformance: the shared vectors in `vectors/update.json`,
//! written against the TypeScript core (`packages/core/test/update-conformance
//! .test.ts`). The Rust mirror must reproduce every expectation exactly: view
//! state, tone, title, body, actions and notice, and the message and retry of
//! every rejection code.

use poodle_headless::update::*;
use serde_json::Value;

fn vectors() -> Value {
    serde_json::from_str(include_str!("../vectors/update.json")).expect("vectors parse")
}

fn rejection_code(name: &str) -> UpdateRejectionCode {
    match name {
        "staleAuthority" => UpdateRejectionCode::StaleAuthority,
        "noOffer" => UpdateRejectionCode::NoOffer,
        "unavailable" => UpdateRejectionCode::Unavailable,
        "channelMismatch" => UpdateRejectionCode::ChannelMismatch,
        "unreachable" => UpdateRejectionCode::Unreachable,
        "signatureRejected" => UpdateRejectionCode::SignatureRejected,
        "notWritable" => UpdateRejectionCode::NotWritable,
        "malformedArtifact" => UpdateRejectionCode::MalformedArtifact,
        "installFailed" => UpdateRejectionCode::InstallFailed,
        other => panic!("unknown rejection code in vectors: {other}"),
    }
}

fn action(name: &str) -> UpdateStatusAction {
    match name {
        "check" => UpdateStatusAction::Check,
        "install" => UpdateStatusAction::Install,
        "defer" => UpdateStatusAction::Defer,
        "downloadAgain" => UpdateStatusAction::DownloadAgain,
        other => panic!("unknown action in vectors: {other}"),
    }
}

fn availability(value: &Value) -> UpdateAvailabilityProjection {
    match value["state"].as_str().expect("availability state") {
        "offer" => UpdateAvailabilityProjection::Offer {
            version: value["version"].as_str().unwrap().to_string(),
            reason: OfferReason::Staged,
            notes: value["notes"].as_str().map(str::to_string),
        },
        "unavailableForTarget" => UpdateAvailabilityProjection::UnavailableForTarget {
            version: value["version"].as_str().unwrap().to_string(),
        },
        other => panic!("availability state not in vectors yet: {other}"),
    }
}

fn input(value: &Value) -> UpdateStatusInput {
    let status = match value["status"]["kind"].as_str().unwrap_or("idle") {
        "idle" => UpdateControllerStatus::Idle,
        "ready" => UpdateControllerStatus::Ready,
        "loading" => UpdateControllerStatus::Loading,
        other => panic!("controller status not in vectors yet: {other}"),
    };
    UpdateStatusInput {
        status,
        availability: value.get("availability").map(availability),
        progress: None,
        deferral: None,
        last_rejection: value["lastRejection"].as_str().map(rejection_code),
        last_rejection_detail: value["lastRejectionDetail"].as_str().map(str::to_string),
        ahead_of_channel: None,
        channel: None,
        installed_version: None,
    }
}

#[test]
fn views_match_the_shared_vectors() {
    let doc = vectors();
    let cases = doc["views"].as_array().expect("views array");
    assert!(!cases.is_empty());
    for case in cases {
        let id = case["id"].as_str().unwrap();
        let expect = &case["expect"];
        let view = update_status_view(input(&case["input"]));

        assert_eq!(view.state.as_str(), expect["state"], "{id}: state");
        assert_eq!(view.tone.as_str(), expect["tone"], "{id}: tone");
        assert_eq!(view.title, expect["title"], "{id}: title");
        assert_eq!(view.body.as_deref(), expect["body"].as_str(), "{id}: body");

        let actions: Vec<&str> = view
            .actions
            .iter()
            .map(|a| match a {
                UpdateStatusAction::Check => "check",
                UpdateStatusAction::Install => "install",
                UpdateStatusAction::Defer => "defer",
                UpdateStatusAction::DownloadAgain => "downloadAgain",
            })
            .collect();
        let expected_actions: Vec<&str> = expect["actions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert_eq!(actions, expected_actions, "{id}: actions");

        if expect["notice"].is_null() {
            assert!(view.notice.is_none(), "{id}: notice should be absent");
        } else {
            let notice = view.notice.as_ref().unwrap_or_else(|| panic!("{id}: notice"));
            let tone = match expect["notice"]["tone"].as_str().unwrap() {
                "neutral" => UpdateStatusNoticeTone::Neutral,
                "danger" => UpdateStatusNoticeTone::Danger,
                other => panic!("{id}: unknown notice tone {other}"),
            };
            assert_eq!(notice.tone, tone, "{id}: notice tone");
            assert_eq!(
                notice.message,
                expect["notice"]["message"].as_str().unwrap(),
                "{id}: notice message"
            );
            let retry = expect["notice"]["retry"].as_str().map(action);
            assert_eq!(notice.retry, retry, "{id}: notice retry");
        }
    }
}

#[test]
fn rejections_match_the_shared_vectors() {
    let doc = vectors();
    let cases = doc["rejections"].as_array().expect("rejections array");
    assert_eq!(cases.len(), 9, "every rejection code has a vector row");
    for case in cases {
        let code = rejection_code(case["code"].as_str().unwrap());
        assert_eq!(
            update_rejection_message(code),
            case["message"].as_str().unwrap(),
            "{code:?}: message"
        );
        let retry = case["retry"].as_str().map(action);
        assert_eq!(update_rejection_retry(code), retry, "{code:?}: retry");
    }
}
