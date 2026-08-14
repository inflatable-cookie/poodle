//! Preview-catalogue emission (g14.025).
//!
//! The manifest is not an `IrModel`. These tests cover load/validate, planted
//! classification failures, byte-identical double generation, and stale
//! TypeScript / Rust check-mode drift.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use poodle_codegen::catalogue::{self, SCHEMA_VERSION};
use poodle_codegen::targets::{catalogue_rust, catalogue_ts};
use poodle_codegen::{check_outputs, write_outputs, DriftKind};

const MANIFEST: &str = "packages/codegen/fixtures/preview-catalogue.json";

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    crate_dir()
        .parent()
        .expect("packages/codegen has a parent")
        .parent()
        .expect("packages has a parent")
        .to_path_buf()
}

fn manifest_path() -> PathBuf {
    repo_root().join(MANIFEST)
}

fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("preview-catalogue")
        .join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir creates");
    dir
}

fn load() -> catalogue::Document {
    catalogue::load_and_validate(&manifest_path()).expect("manifest loads and validates")
}

#[test]
fn manifest_classifies_every_current_entry_once() {
    let document = load();
    assert_eq!(document.schema_version, SCHEMA_VERSION);
    assert!(!document.components.is_empty());
    let slugs: std::collections::BTreeSet<&str> = document
        .components
        .iter()
        .map(|component| component.slug.as_str())
        .collect();
    assert_eq!(slugs.len(), document.components.len());
    assert!(catalogue::validate(&document).is_empty());
}

#[test]
fn double_generation_is_byte_identical() {
    let document = load();
    let ts_a = catalogue_ts::render(&document, MANIFEST);
    let ts_b = catalogue_ts::render(&document, MANIFEST);
    let rs_a = catalogue_rust::render(&document, MANIFEST);
    let rs_b = catalogue_rust::render(&document, MANIFEST);
    assert_eq!(ts_a, ts_b);
    assert_eq!(rs_a, rs_b);
}

#[test]
fn planted_missing_classification_fails_load() {
    let original = fs::read_to_string(manifest_path()).expect("manifest reads");
    let planted = original.replacen("\"family\": \"actions-selection\"", "\"family\": \"\"", 1);
    assert_ne!(original, planted);
    let path = scratch("missing-classification").join("preview-catalogue.json");
    fs::write(&path, planted).expect("write planted");
    let error = catalogue::load_and_validate(&path).expect_err("empty family fails");
    let message = error.to_string();
    assert!(
        message.contains("missing classification") || message.contains("unknown family"),
        "{message}"
    );
}

#[test]
fn planted_duplicate_slug_fails_load() {
    let original = fs::read_to_string(manifest_path()).expect("manifest reads");
    let needle = "\"slug\": \"button\"";
    let planted = original.replacen(needle, "\"slug\": \"icon-button\"", 1);
    assert_ne!(original, planted);
    let path = scratch("duplicate-slug").join("preview-catalogue.json");
    fs::write(&path, planted).expect("write planted");
    let error = catalogue::load_and_validate(&path).expect_err("duplicate slug fails");
    assert!(
        error.to_string().contains("duplicate slug 'icon-button'"),
        "{}",
        error
    );
}

#[test]
fn planted_invalid_family_fails_load() {
    let original = fs::read_to_string(manifest_path()).expect("manifest reads");
    let planted = original.replacen(
        "\"family\": \"actions-selection\"",
        "\"family\": \"miscellaneous\"",
        1,
    );
    assert_ne!(original, planted);
    let path = scratch("invalid-family").join("preview-catalogue.json");
    fs::write(&path, planted).expect("write planted");
    let error = catalogue::load_and_validate(&path).expect_err("invalid family fails");
    assert!(
        error.to_string().contains("unknown family 'miscellaneous'"),
        "{}",
        error
    );
}

#[test]
fn check_detects_stale_typescript() {
    let document = load();
    let files = catalogue_ts::render(&document, MANIFEST);
    let root = scratch("stale-ts");
    write_outputs(&root, &files).expect("write");
    let mut drifted = files.clone();
    drifted[0].contents.push_str("// planted\n");
    let report = check_outputs(&root, &drifted).expect("check");
    assert!(!report.is_clean());
    assert!(
        report
            .drifted
            .iter()
            .any(|(path, kind)| path.ends_with("catalogue.ts") && *kind == DriftKind::Content),
        "{:?}",
        report.drifted
    );
}

#[test]
fn check_detects_stale_rust() {
    let document = load();
    let files = catalogue_rust::render(&document, MANIFEST);
    let root = scratch("stale-rs");
    write_outputs(&root, &files).expect("write");
    let mut drifted = files.clone();
    drifted[0].contents.push_str("// planted\n");
    let report = check_outputs(&root, &drifted).expect("check");
    assert!(!report.is_clean());
    assert!(
        report
            .drifted
            .iter()
            .any(|(path, kind)| path.ends_with("catalogue.rs") && *kind == DriftKind::Content),
        "{:?}",
        report.drifted
    );
}

#[test]
fn cli_check_fails_on_stale_typescript() {
    let document = load();
    let files = catalogue_ts::render(&document, MANIFEST);
    let out = scratch("cli-stale-out");
    let root = out.join(catalogue_ts::OUTPUT_ROOT);
    write_outputs(&root, &files).expect("write");
    fs::write(root.join("catalogue.ts"), "// stale\n").expect("plant stale bytes");

    let bin = env!("CARGO_BIN_EXE_poodle-codegen");
    let output = Command::new(bin)
        .args([
            "--catalogue",
            MANIFEST,
            "--out",
            out.to_str().expect("utf-8"),
            "--target",
            "catalogue-ts",
            "--check",
        ])
        .current_dir(repo_root())
        .output()
        .expect("bin runs");
    assert!(!output.status.success(), "stale TS fails the gate");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("catalogue.ts"),
        "stderr names the artifact: {stderr}"
    );
}

#[test]
fn catalogue_targets_are_not_in_the_ir_registry() {
    assert!(poodle_codegen::targets::by_id("catalogue-ts").is_none());
    assert!(poodle_codegen::targets::by_id("catalogue-rust").is_none());
}

#[test]
fn committed_artifacts_match_the_emitter() {
    let document = load();
    let ts = catalogue_ts::render(&document, MANIFEST);
    let rs = catalogue_rust::render(&document, MANIFEST);
    let ts_root = repo_root()
        .join("packages/svelte/preview/src")
        .join(catalogue_ts::OUTPUT_ROOT);
    let rs_root = repo_root()
        .join("packages/gpui/preview/src")
        .join(catalogue_rust::OUTPUT_ROOT);
    let ts_report = check_outputs(&ts_root, &ts).expect("ts check");
    let rs_report = check_outputs(&rs_root, &rs).expect("rs check");
    assert!(ts_report.is_clean(), "{}", ts_report.message());
    assert!(rs_report.is_clean(), "{}", rs_report.message());
    let react_root = repo_root()
        .join("packages/react/preview/src")
        .join(catalogue_ts::OUTPUT_ROOT);
    let jet_root = repo_root()
        .join("packages/jetstream/preview/src")
        .join(catalogue_rust::OUTPUT_ROOT);
    assert!(check_outputs(&react_root, &ts).expect("react").is_clean());
    assert!(check_outputs(&jet_root, &rs).expect("jetstream").is_clean());
}
