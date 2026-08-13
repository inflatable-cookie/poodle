//! Machine-interface emission (g14-b004, spec 064 mechanism 1).
//!
//! The schema is not an `IrModel`. These tests cover load/validate, byte-
//! identical double generation, drift detection that names the machine, and
//! the CLI's select-only path.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use poodle_codegen::machine_interfaces::{self, SCHEMA_VERSION};
use poodle_codegen::targets::{machine_rust, machine_ts};
use poodle_codegen::{check_outputs, write_outputs, DriftKind};

const SCHEMA: &str = "packages/contracts/headless/machine-interfaces.json";

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

fn schema_path() -> PathBuf {
    repo_root().join(SCHEMA)
}

fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("machine-interfaces").join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir creates");
    dir
}

fn load() -> poodle_codegen::machine_interfaces::Document {
    machine_interfaces::load_and_validate(&schema_path()).expect("committed schema loads")
}

#[test]
fn committed_schema_loads_four_pilots() {
    let document = load();
    assert_eq!(document.schema_version, SCHEMA_VERSION);
    let ids: Vec<&str> = document.machines.iter().map(|m| m.id.as_str()).collect();
    assert_eq!(ids, ["hover", "menu", "modal", "popover"]);
}

#[test]
fn double_generation_is_byte_identical() {
    let document = load();
    let ts_a = machine_ts::render(&document, SCHEMA);
    let ts_b = machine_ts::render(&document, SCHEMA);
    assert_eq!(ts_a, ts_b);
    let rs_a = machine_rust::render(&document, SCHEMA);
    let rs_b = machine_rust::render(&document, SCHEMA);
    assert_eq!(rs_a, rs_b);
    assert_eq!(ts_a.len(), 4);
    assert_eq!(rs_a.len(), 4);
}

#[test]
fn check_is_clean_against_what_the_emitter_just_wrote() {
    let document = load();
    let files = machine_ts::render(&document, SCHEMA);
    let root = scratch("clean");
    write_outputs(&root, &files).expect("write");
    let report = check_outputs(&root, &files).expect("check");
    assert!(report.is_clean(), "{}", report.message());
}

#[test]
fn planted_schema_divergence_names_the_machine() {
    let original = fs::read_to_string(schema_path()).expect("schema reads");
    let files = machine_ts::render(&load(), SCHEMA);
    let root = scratch("plant");
    write_outputs(&root, &files).expect("write original artifacts");

    let planted = original.replace(
        "{ \"ts\": \"closing\", \"rs\": \"Closing\" }",
        "{ \"ts\": \"closing\", \"rs\": \"Closing\" }, { \"ts\": \"planted\", \"rs\": \"Planted\" }",
    );
    assert_ne!(original, planted, "the plant is a real edit");
    let planted_path = scratch("plant-schema").join("machine-interfaces.json");
    fs::write(&planted_path, planted).expect("write planted schema");
    let planted_doc = machine_interfaces::load_and_validate(&planted_path).expect("planted loads");
    let planted_files = machine_ts::render(&planted_doc, SCHEMA);
    let report = check_outputs(&root, &planted_files).expect("check");
    assert!(!report.is_clean(), "planted state must drift");
    let message = report.message();
    assert!(
        message.contains("hover.ts"),
        "gate names the machine via its artifact: {message}"
    );
    assert!(
        report.drifted.iter().any(|(path, kind)| {
            path.ends_with("hover.ts") && *kind == DriftKind::Content
        }),
        "hover is content drift: {:?}",
        report.drifted
    );
}

#[test]
fn cli_check_fails_on_planted_hover_and_names_it() {
    let document = load();
    let files = machine_ts::render(&document, SCHEMA);
    let out = scratch("cli-plant-out");
    let root = out.join(machine_ts::OUTPUT_ROOT);
    write_outputs(&root, &files).expect("write");

    let original = fs::read_to_string(schema_path()).expect("schema reads");
    let planted = original.replace(
        "{ \"ts\": \"closing\", \"rs\": \"Closing\" }",
        "{ \"ts\": \"closing\", \"rs\": \"Closing\" }, { \"ts\": \"planted\", \"rs\": \"Planted\" }",
    );
    let schema = scratch("cli-plant-schema").join("machine-interfaces.json");
    fs::write(&schema, planted).expect("write planted");

    let bin = env!("CARGO_BIN_EXE_poodle-codegen");
    let output = Command::new(bin)
        .args([
            "--machine-interfaces",
            schema.to_str().expect("utf-8"),
            "--out",
            out.to_str().expect("utf-8"),
            "--target",
            "machine-ts",
            "--check",
        ])
        .current_dir(repo_root())
        .output()
        .expect("bin runs");
    assert!(!output.status.success(), "planted schema fails the gate");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("hover.ts"),
        "stderr names the machine: {stderr}"
    );
    assert!(!stderr.contains("panicked"), "no panic");
}

#[test]
fn malformed_schema_is_a_clean_error() {
    let dir = scratch("malformed");
    let bad = dir.join("bad.json");
    fs::write(&bad, "{ this is not json").expect("write garbage");
    let err = machine_interfaces::load_and_validate(&bad).expect_err("malformed fails");
    let message = err.to_string();
    assert!(
        message.contains("not valid machine-interface JSON"),
        "{message}"
    );
}

#[test]
fn machine_targets_are_not_in_the_ir_registry() {
    assert!(poodle_codegen::targets::by_id("machine-ts").is_none());
    assert!(poodle_codegen::targets::by_id("machine-rust").is_none());
    let ids: Vec<&str> = poodle_codegen::targets::selectable()
        .iter()
        .map(|target| target.id())
        .collect();
    assert!(!ids.contains(&"machine-ts"));
    assert!(!ids.contains(&"machine-rust"));
}

#[test]
fn committed_artifacts_match_the_emitter() {
    let document = load();
    let ts = machine_ts::render(&document, SCHEMA);
    let rs = machine_rust::render(&document, SCHEMA);
    let ts_root = repo_root().join("packages/core/src").join(machine_ts::OUTPUT_ROOT);
    let rs_root = repo_root()
        .join("packages/contracts/headless/src")
        .join(machine_rust::OUTPUT_ROOT);
    let ts_report = check_outputs(&ts_root, &ts).expect("ts check");
    let rs_report = check_outputs(&rs_root, &rs).expect("rs check");
    assert!(
        ts_report.is_clean(),
        "TypeScript machine interfaces drifted:\n{}",
        ts_report.message()
    );
    assert!(
        rs_report.is_clean(),
        "Rust machine interfaces drifted:\n{}",
        rs_report.message()
    );
}
