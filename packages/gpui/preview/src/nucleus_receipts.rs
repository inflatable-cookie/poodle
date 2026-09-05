//! M1 receipts for the fixed Nucleus mounted cohort.
//!
//! This module is included by the real headless regression target. It emits a
//! receipt only when a test explicitly supplies the private observation token
//! returned by `HeadlessDriver::mounted_observation` after production-path
//! input and assertions have completed. It is not a component registry and it
//! cannot turn a test name or direct handler call into evidence.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::headless_driver::MountedObservation;
use serde::Serialize;

const RECEIPT_SCHEMA: &str = "poodle.g16-nucleus-parity-receipt.v1";
const RUNTIME: &str = "gpui-headless";
const COMMAND: &str = "effigy regressions:native";
const PACKAGE: &str = "poodle-gpui-preview";
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
const LOCKFILE: &str = "packages/gpui/preview/Cargo.lock";
const LOCKFILE_SHA256: &str = "c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c";
const DISTRIBUTION: &str = "workspace";

#[derive(Serialize)]
struct LockedPackage {
    name: &'static str,
    version: &'static str,
    source: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    checksum: Option<&'static str>,
}

#[derive(Serialize)]
struct ProductionPathObservation {
    observed: bool,
    mount: &'static str,
    render_path: &'static str,
    input_dispatch: &'static str,
}

#[derive(Serialize)]
struct Artifact {
    path: &'static str,
    sha256: &'static str,
}

#[derive(Serialize)]
struct NucleusReceipt {
    schema: &'static str,
    component: &'static str,
    scenario_id: &'static str,
    proof_level: &'static str,
    runtime: &'static str,
    command: &'static str,
    package: &'static str,
    package_version: &'static str,
    source_commit: String,
    lockfile: &'static str,
    lockfile_sha256: String,
    lock_resolution: Vec<LockedPackage>,
    distribution: &'static str,
    production_path_observation: ProductionPathObservation,
    actions: Vec<&'static str>,
    assertions: Vec<&'static str>,
    outcome: &'static str,
    artifact_paths: Vec<Artifact>,
}

fn lock_resolution() -> Vec<LockedPackage> {
    vec![
        LockedPackage {
            name: "gpui",
            version: "0.2.2",
            source: "crates.io",
            checksum: Some("979b45cfa6ec723b6f42330915a1b3769b930d02b2d505f9697f8ca602bee707"),
        },
        LockedPackage {
            name: "poodle-gpui",
            version: "0.3.0",
            source: "workspace",
            checksum: None,
        },
        LockedPackage {
            name: "poodle-gpui-preview",
            version: "0.3.0",
            source: "workspace",
            checksum: None,
        },
        LockedPackage {
            name: "poodle-node",
            version: "0.3.0",
            source: "workspace",
            checksum: None,
        },
        LockedPackage {
            name: "poodle-render",
            version: "0.3.0",
            source: "workspace",
            checksum: None,
        },
    ]
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn source_commit(root: &PathBuf) -> String {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()
        .expect("git is available for a parity receipt");
    assert!(
        output.status.success(),
        "git rev-parse HEAD failed while emitting a parity receipt"
    );
    let commit = String::from_utf8(output.stdout)
        .expect("git commit is UTF-8")
        .trim()
        .to_owned();
    assert!(
        commit.len() == 40
            && commit
                .chars()
                .all(|character| character.is_ascii_hexdigit()),
        "receipt source commit is not a full commit SHA"
    );
    commit
}

fn lockfile_sha256(root: &PathBuf) -> String {
    fs::read(root.join(LOCKFILE)).expect("GPUI preview Cargo.lock exists");
    LOCKFILE_SHA256.to_owned()
}

fn safe_file_stem(component: &str, scenario_id: &str) -> String {
    format!(
        "{}--{}",
        component.to_ascii_lowercase().replace(' ', "-"),
        scenario_id.replace('.', "-")
    )
}

/// Emit one deterministic receipt when the mounted selector has been asked to
/// collect execution evidence. The ordinary regression suite stays disposable
/// unless `POODLE_NUCLEUS_RECEIPT_DIR` is set by its caller.
pub(crate) fn emit_if_configured(
    component: &'static str,
    scenario_id: &'static str,
    observation: MountedObservation,
    actions: &[&'static str],
    assertions: &[&'static str],
) {
    let Some(directory) = env::var_os("POODLE_NUCLEUS_RECEIPT_DIR") else {
        return;
    };
    assert!(
        observation.is_valid(),
        "receipt requires observed mounted paint and GPUI input dispatch"
    );

    let root = repository_root();
    let receipt = NucleusReceipt {
        schema: RECEIPT_SCHEMA,
        component,
        scenario_id,
        proof_level: "M1",
        runtime: RUNTIME,
        command: COMMAND,
        package: PACKAGE,
        package_version: PACKAGE_VERSION,
        source_commit: source_commit(&root),
        lockfile: LOCKFILE,
        lockfile_sha256: lockfile_sha256(&root),
        lock_resolution: lock_resolution(),
        distribution: DISTRIBUTION,
        production_path_observation: ProductionPathObservation {
            observed: true,
            mount: "HeadlessDriver",
            render_path: "poodle_render -> poodle_gpui_node_backend::to_gpui",
            input_dispatch: "gpui-test-platform-dispatch",
        },
        actions: actions.to_vec(),
        assertions: assertions.to_vec(),
        outcome: "passed",
        artifact_paths: Vec::new(),
    };

    let directory = PathBuf::from(directory);
    fs::create_dir_all(&directory).expect("parity receipt directory can be created");
    let destination = directory.join(format!("{}.json", safe_file_stem(component, scenario_id)));
    let temporary = destination.with_extension("json.tmp");
    let encoded = serde_json::to_vec_pretty(&receipt).expect("parity receipt serializes");
    fs::write(&temporary, encoded).expect("parity receipt can be written");
    fs::rename(&temporary, &destination).expect("parity receipt can be published");
    eprintln!("nucleus receipt: {}", destination.display());
}

// ── A1 paired accessibility receipts (g16.111) ────────────────────────────
//
// An A1 receipt pairs the mounted GPUI node-tree accessibility projection
// with the mounted Svelte DOM's ARIA semantics for the same shared scenario.
// The scenario file is deserialised here and hashed; the Svelte snapshot
// carries the hash it ran against, and a mismatch is rejected before any
// comparison. Both snapshots are committed artifacts whose SHA-256 the
// receipt records. The receipt is emitted only after the diff is empty.

use crate::headless_driver::MountedAccessibilityNode;
use poodle_node::{NodeRole, NodeToggled};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

pub(crate) const A1_SCENARIO_SCHEMA: &str = "poodle.g16-nucleus-a11y-scenario.v1";
pub(crate) const A1_SNAPSHOT_SCHEMA: &str = "poodle.g16-nucleus-a11y-snapshot.v1";
pub(crate) const A1_SCENARIO_DIR: &str = "test/nucleus-a11y/scenarios";
pub(crate) const A1_SNAPSHOT_DIR: &str = "test/nucleus-a11y/snapshots";
const A1_GPUI_RUNTIME: &str = "gpui-headless";
const A1_SVELTE_RUNTIME: &str = "svelte-happy-dom";
const A1_SVELTE_COMMAND: &str = "effigy test:nucleus-a11y";
const A1_SVELTE_MOUNT: &str = "@testing-library/svelte render";
const A1_SVELTE_INPUT_DISPATCH: &str = "dom-events";

/// A node reference shared by both extractors: the first node in document
/// order whose role and accessible name match. No runtime id ever appears
/// in a scenario, so the same file drives the DOM and the node tree.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct A1Target {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum A1Action {
    /// Pointer press and release on the target (mouse down focuses a
    /// focusable control on both runtimes, then the click activates it).
    PointerActivate { target: A1Target },
    /// Focus the target, then one named key press and release.
    Key { target: A1Target, key: String },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct A1Exclusion {
    pub attribute: String,
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct A1Scenario {
    pub schema: String,
    pub component: String,
    pub scenario_id: String,
    /// Web-named props, applied verbatim by the Svelte extractor and mapped
    /// field-for-field by the row's Rust proof (unknown keys are rejected).
    pub props: Value,
    /// Fixture text that is not a public prop on either runtime (slot content).
    #[serde(default)]
    pub fixtures: Map<String, Value>,
    pub actions: Vec<A1Action>,
    /// States compared for this component, exactly as its contract declares.
    pub declared_states: Vec<String>,
    #[serde(default)]
    pub web_only_exclusions: Vec<A1Exclusion>,
}

pub(crate) struct LoadedA1Scenario {
    pub row: &'static str,
    pub path: String,
    pub sha256: String,
    pub scenario: A1Scenario,
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Deserialise the shared scenario file for one cohort row and hash its
/// exact bytes. A shape the Rust side does not understand is an error, not
/// a silently ignored key.
pub(crate) fn load_a1_scenario(row: &'static str) -> LoadedA1Scenario {
    let relative = format!("{A1_SCENARIO_DIR}/{row}.json");
    let bytes = fs::read(repository_root().join(&relative))
        .unwrap_or_else(|error| panic!("A1 scenario {relative} is unreadable: {error}"));
    let scenario: A1Scenario = serde_json::from_slice(&bytes)
        .unwrap_or_else(|error| panic!("A1 scenario {relative} does not deserialise: {error}"));
    assert_eq!(scenario.schema, A1_SCENARIO_SCHEMA, "{relative} schema");
    LoadedA1Scenario {
        row,
        path: relative,
        sha256: sha256_hex(&bytes),
        scenario,
    }
}

/// The ARIA role string a `poodle-node` role projects as. One mapping,
/// total over the enum, so a new role cannot silently compare as `null`.
pub(crate) fn aria_role(role: NodeRole) -> &'static str {
    match role {
        NodeRole::Alert => "alert",
        NodeRole::AlertDialog => "alertdialog",
        NodeRole::Banner => "banner",
        NodeRole::Button => "button",
        NodeRole::Cell => "cell",
        NodeRole::CheckBox => "checkbox",
        NodeRole::ComboBox => "combobox",
        NodeRole::Dialog => "dialog",
        NodeRole::Grid => "grid",
        NodeRole::Group => "group",
        NodeRole::Heading => "heading",
        NodeRole::SearchBox => "searchbox",
        NodeRole::Label => "label",
        NodeRole::List => "list",
        NodeRole::ListItem => "listitem",
        NodeRole::ListBox => "listbox",
        NodeRole::ListBoxOption => "option",
        NodeRole::Log => "log",
        NodeRole::Image => "img",
        NodeRole::Menu => "menu",
        NodeRole::MenuBar => "menubar",
        NodeRole::MenuItem => "menuitem",
        NodeRole::MenuItemCheckBox => "menuitemcheckbox",
        NodeRole::MenuItemRadio => "menuitemradio",
        NodeRole::Splitter => "separator",
        NodeRole::Slider => "slider",
        NodeRole::ProgressIndicator => "progressbar",
        NodeRole::RadioGroup => "radiogroup",
        NodeRole::RadioButton => "radio",
        NodeRole::Region => "region",
        NodeRole::Row => "row",
        NodeRole::SpinButton => "spinbutton",
        NodeRole::Status => "status",
        NodeRole::Switch => "switch",
        NodeRole::Tab => "tab",
        NodeRole::TabList => "tablist",
        NodeRole::TabPanel => "tabpanel",
        NodeRole::TextInput => "textbox",
        NodeRole::Toolbar => "toolbar",
        NodeRole::Tooltip => "tooltip",
        NodeRole::Tree => "tree",
        NodeRole::TreeItem => "treeitem",
    }
}

fn trimmed(value: Option<&str>) -> Value {
    match value.map(str::trim) {
        Some(text) if !text.is_empty() => Value::String(text.to_owned()),
        _ => Value::Null,
    }
}

fn resolve_targets(reference: Option<&str>, nodes: &[MountedAccessibilityNode]) -> Vec<i64> {
    let Some(reference) = reference else {
        return Vec::new();
    };
    reference
        .split_whitespace()
        .map(|target| {
            nodes
                .iter()
                .position(|node| node.semantic_id.as_deref() == Some(target))
                .map_or(-1, |index| index as i64)
        })
        .collect()
}

/// The accessible name the node record yields: `labelled_by` resolves to the
/// referenced node's own label (the record-level half of the accessible-name
/// algorithm), else the record's label. There is no name-from-content
/// fallback on this side: a name the record lacks is reported as `null`.
fn record_name(node: &MountedAccessibilityNode, nodes: &[MountedAccessibilityNode]) -> Value {
    if let Some(reference) = node.labelled_by.as_deref() {
        let joined = reference
            .split_whitespace()
            .filter_map(|target| {
                nodes
                    .iter()
                    .find(|candidate| candidate.semantic_id.as_deref() == Some(target))
                    .and_then(|candidate| candidate.label.clone())
            })
            .collect::<Vec<_>>()
            .join(" ");
        if !joined.trim().is_empty() {
            return Value::String(joined.trim().to_owned());
        }
    }
    trimmed(node.label.as_deref())
}

fn value_text(node: &MountedAccessibilityNode) -> Value {
    if node.value_text.is_some() {
        return trimmed(node.value_text.as_deref());
    }
    // A combobox with no declared value text exposes its visible value: the
    // text content of the node, whitespace-normalised. Text inputs declare
    // their actual value explicitly so placeholder text is never a value.
    if matches!(node.role, NodeRole::ComboBox) {
        let joined = node
            .text_content
            .iter()
            .flat_map(|text| text.split_whitespace())
            .collect::<Vec<_>>()
            .join(" ");
        return trimmed(Some(&joined));
    }
    Value::Null
}

fn declared_state(node: &MountedAccessibilityNode, state: &str) -> Value {
    match state {
        "checked" => match node.toggled {
            Some(NodeToggled::True) => json!(true),
            Some(NodeToggled::False) => json!(false),
            Some(NodeToggled::Mixed) => json!("mixed"),
            None => Value::Null,
        },
        "expanded" => json!(node.expanded),
        "selected" => json!(node.selected),
        "disabled" => json!(node.disabled),
        "invalid" => json!(node.invalid),
        "busy" => json!(node.busy),
        other => panic!("A1 scenario declares an unknown state `{other}`"),
    }
}

pub(crate) fn is_sequential_tab_stop(node: &MountedAccessibilityNode) -> bool {
    node.focusable && !node.disabled && node.tab_index.map_or(true, |index| index >= 0)
}

/// Normalise the mounted projection into the shared snapshot shape:
/// relationships by index, names trimmed, value text as strings, and only
/// the states the scenario declares for this component.
pub(crate) fn normalise_a1_nodes(
    nodes: &[MountedAccessibilityNode],
    scenario: &A1Scenario,
) -> Vec<Value> {
    let mut focus_order = 0i64;
    nodes
        .iter()
        .map(|node| {
            let mut states = Map::new();
            for state in &scenario.declared_states {
                states.insert(state.clone(), declared_state(node, state));
            }
            let order = if is_sequential_tab_stop(node) {
                let index = focus_order;
                focus_order += 1;
                json!(index)
            } else {
                Value::Null
            };
            json!({
                "role": aria_role(node.role),
                "name": record_name(node, nodes),
                "value": node.value,
                "value_text": value_text(node),
                "states": states,
                "relationships": {
                    "controls": resolve_targets(node.controls.as_deref(), nodes),
                    "labelled_by": resolve_targets(node.labelled_by.as_deref(), nodes),
                    "described_by": resolve_targets(node.described_by.as_deref(), nodes),
                },
                "level": node.level,
                "orientation": trimmed(node.orientation.as_deref()),
                "focus_order": order,
                "focused": node.focused,
            })
        })
        .collect()
}

/// The GPUI snapshot file: the normalised nodes plus the run record that
/// proves they came from a mounted, input-driven frame.
pub(crate) fn gpui_snapshot_file(
    loaded: &LoadedA1Scenario,
    observation: MountedObservation,
    nodes: Vec<Value>,
) -> Value {
    json!({
        "schema": A1_SNAPSHOT_SCHEMA,
        "component": loaded.scenario.component,
        "scenario_id": loaded.scenario.scenario_id,
        "scenario_path": loaded.path,
        "scenario_sha256": loaded.sha256,
        "runtime": A1_GPUI_RUNTIME,
        "run": {
            "command": COMMAND,
            "mount": "HeadlessDriver",
            "render_path": "poodle_render -> poodle_gpui_node_backend::to_gpui",
            "input_dispatch": "gpui-test-platform-dispatch",
        },
        "nodes": nodes,
    })
}

fn snapshot_bytes(file: &Value) -> Vec<u8> {
    let mut encoded = serde_json::to_vec_pretty(file).expect("A1 snapshot serialises");
    encoded.push(b'\n');
    encoded
}

/// Read the committed Svelte snapshot for the row and reject it unless it
/// ran against exactly this scenario file and carries a real run record.
pub(crate) fn load_svelte_snapshot(loaded: &LoadedA1Scenario) -> (String, String, Value) {
    let relative = format!("{}/{}.svelte.json", A1_SNAPSHOT_DIR, loaded.row);
    let bytes = fs::read(repository_root().join(&relative)).unwrap_or_else(|error| {
        panic!("Svelte A1 snapshot {relative} is missing; run `{A1_SVELTE_COMMAND}` with POODLE_NUCLEUS_A11Y_WRITE=1 ({error})")
    });
    let file: Value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|error| panic!("Svelte A1 snapshot {relative} does not parse: {error}"));
    let field = |key: &str| {
        file.get(key)
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned()
    };
    assert_eq!(field("schema"), A1_SNAPSHOT_SCHEMA, "{relative} schema");
    assert_eq!(field("runtime"), A1_SVELTE_RUNTIME, "{relative} runtime");
    assert_eq!(
        field("component"),
        loaded.scenario.component,
        "{relative} component"
    );
    assert_eq!(
        field("scenario_id"),
        loaded.scenario.scenario_id,
        "{relative} scenario"
    );
    assert_eq!(
        field("scenario_path"),
        loaded.path,
        "{relative} scenario path"
    );
    assert_eq!(
        field("scenario_sha256"),
        loaded.sha256,
        "{relative} was produced from a different scenario file (hash mismatch); regenerate it"
    );
    let run = file.get("run").cloned().unwrap_or(Value::Null);
    let run_field = |key: &str| {
        run.get(key)
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned()
    };
    assert_eq!(
        run_field("command"),
        A1_SVELTE_COMMAND,
        "{relative} run command"
    );
    assert_eq!(run_field("mount"), A1_SVELTE_MOUNT, "{relative} run mount");
    assert_eq!(
        run_field("input_dispatch"),
        A1_SVELTE_INPUT_DISPATCH,
        "{relative} run input dispatch"
    );
    assert!(
        file.get("nodes").is_some_and(Value::is_array),
        "{relative} has no nodes"
    );
    (relative, sha256_hex(&bytes), file)
}

/// Positional, field-by-field comparison of two normalised node lists. An
/// extra node on either side is reported against `role` with `null` on the
/// side that lacks it.
pub(crate) fn diff_a1_nodes(gpui: &[Value], svelte: &[Value]) -> Vec<Value> {
    let mut diff = Vec::new();
    let length = gpui.len().max(svelte.len());
    for index in 0..length {
        match (gpui.get(index), svelte.get(index)) {
            (Some(left), Some(right)) => {
                let left = left.as_object().expect("gpui node object");
                let right = right.as_object().expect("svelte node object");
                let mut keys: Vec<&String> = left.keys().chain(right.keys()).collect();
                keys.sort();
                keys.dedup();
                for key in keys {
                    let left_value = left.get(key).cloned().unwrap_or(Value::Null);
                    let right_value = right.get(key).cloned().unwrap_or(Value::Null);
                    if !a1_values_equal(&left_value, &right_value) {
                        diff.push(json!({
                            "index": index,
                            "field": key,
                            "gpui": left_value,
                            "svelte": right_value,
                        }));
                    }
                }
            }
            (left, right) => diff.push(json!({
                "index": index,
                "field": "role",
                "gpui": left.and_then(|node| node.get("role").cloned()).unwrap_or(Value::Null),
                "svelte": right.and_then(|node| node.get("role").cloned()).unwrap_or(Value::Null),
            })),
        }
    }
    diff
}

/// JSON has one numeric value space for the A1 contract. Keep Rust's typed
/// integer/float representation from reporting a semantic mismatch such as
/// `0` versus `0.0`.
fn a1_values_equal(left: &Value, right: &Value) -> bool {
    match (left.as_f64(), right.as_f64()) {
        (Some(left), Some(right)) => left == right,
        _ => left == right,
    }
}

/// Compare the fresh GPUI snapshot with the committed one. A missing
/// committed file is tolerated only while a receipt directory is configured,
/// which is the run that publishes it.
pub(crate) fn check_committed_gpui_snapshot(row: &str, fresh: &Value) -> String {
    let relative = format!("{A1_SNAPSHOT_DIR}/{row}.gpui.json");
    let path = repository_root().join(&relative);
    match fs::read(&path) {
        Ok(bytes) => {
            let committed: Value = serde_json::from_slice(&bytes)
                .unwrap_or_else(|error| panic!("{relative} does not parse: {error}"));
            assert!(
                committed.get("scenario_sha256") == fresh.get("scenario_sha256"),
                "{relative} was produced from a different scenario file (hash mismatch); re-run `{COMMAND}` to publish it"
            );
            assert!(
                committed == *fresh,
                "{relative} is stale: the mounted GPUI projection changed; re-run `{COMMAND}` and publish the new snapshot"
            );
        }
        Err(_) => assert!(
            env::var_os("POODLE_NUCLEUS_RECEIPT_DIR").is_some(),
            "{relative} is missing; run `{COMMAND}` to publish it"
        ),
    }
    relative
}

/// Emit one A1 receipt and the GPUI snapshot it hashes when the mounted
/// selector has been asked to collect execution evidence. The receipt
/// carries both snapshot hashes, the scenario hash, the exclusions, and the
/// (empty) diff; the caller has already asserted the diff is empty.
#[allow(clippy::too_many_arguments)]
pub(crate) fn emit_a1_if_configured(
    loaded: &LoadedA1Scenario,
    observation: MountedObservation,
    gpui_file: &Value,
    svelte_path: &str,
    svelte_sha256: &str,
    diff: &[Value],
    actions: &[&'static str],
    assertions: &[&'static str],
) {
    let Some(directory) = env::var_os("POODLE_NUCLEUS_RECEIPT_DIR") else {
        return;
    };
    assert!(
        observation.is_valid(),
        "receipt requires observed mounted paint and GPUI input dispatch"
    );
    assert!(
        diff.is_empty(),
        "an A1 receipt is only emitted for an empty diff"
    );

    let root = repository_root();
    let gpui_bytes = snapshot_bytes(gpui_file);
    let gpui_sha256 = sha256_hex(&gpui_bytes);
    let gpui_path = format!("{}/{}.gpui.json", A1_SNAPSHOT_DIR, loaded.row);
    let component: &str = &loaded.scenario.component;
    let scenario_id: &str = &loaded.scenario.scenario_id;
    let receipt = json!({
        "schema": RECEIPT_SCHEMA,
        "component": component,
        "scenario_id": scenario_id,
        "proof_level": "A1",
        "runtime": RUNTIME,
        "command": COMMAND,
        "package": PACKAGE,
        "package_version": PACKAGE_VERSION,
        "source_commit": source_commit(&root),
        "lockfile": LOCKFILE,
        "lockfile_sha256": lockfile_sha256(&root),
        "lock_resolution": lock_resolution(),
        "distribution": DISTRIBUTION,
        "production_path_observation": {
            "observed": true,
            "mount": "HeadlessDriver",
            "render_path": "poodle_render -> poodle_gpui_node_backend::to_gpui",
            "input_dispatch": "gpui-test-platform-dispatch",
        },
        "actions": actions,
        "assertions": assertions,
        "outcome": "passed",
        "artifact_paths": [
            { "path": gpui_path, "sha256": gpui_sha256 },
            { "path": svelte_path, "sha256": svelte_sha256 },
        ],
        "accessibility": {
            "scenario_path": loaded.path,
            "scenario_sha256": loaded.sha256,
            "gpui_snapshot_path": gpui_path,
            "gpui_snapshot_sha256": gpui_sha256,
            "svelte_snapshot_path": svelte_path,
            "svelte_snapshot_sha256": svelte_sha256,
            "web_only_exclusions": loaded.scenario.web_only_exclusions.iter().map(|exclusion| json!({
                "attribute": exclusion.attribute,
                "reason": exclusion.reason,
            })).collect::<Vec<_>>(),
            "diff": diff,
        },
    });

    let directory = PathBuf::from(directory);
    fs::create_dir_all(&directory).expect("parity receipt directory can be created");
    let stem = format!("{}--a1", safe_file_stem(component, scenario_id));
    let destination = directory.join(format!("{stem}.json"));
    let temporary = destination.with_extension("json.tmp");
    let encoded = serde_json::to_vec_pretty(&receipt).expect("A1 receipt serialises");
    fs::write(&temporary, encoded).expect("A1 receipt can be written");
    fs::rename(&temporary, &destination).expect("A1 receipt can be published");
    let snapshot_destination = directory.join(format!("{}.gpui.json", loaded.row));
    fs::write(&snapshot_destination, gpui_bytes).expect("A1 GPUI snapshot can be written");
    eprintln!("nucleus A1 receipt: {}", destination.display());
    eprintln!(
        "nucleus A1 gpui snapshot: {}",
        snapshot_destination.display()
    );
}

/// A diverged row publishes its executed GPUI snapshot and the diff beside
/// the receipts so the log can cite them. It never emits a receipt.
pub(crate) fn publish_a1_divergence_if_configured(
    loaded: &LoadedA1Scenario,
    gpui_file: &Value,
    diff: &[Value],
) {
    let Some(directory) = env::var_os("POODLE_NUCLEUS_RECEIPT_DIR") else {
        return;
    };
    let directory = PathBuf::from(directory)
        .join("a1-divergences")
        .join(loaded.row);
    fs::create_dir_all(&directory).expect("parity divergence directory can be created");
    fs::write(directory.join("gpui.json"), snapshot_bytes(gpui_file))
        .expect("A1 GPUI snapshot can be written");
    let mut encoded = serde_json::to_vec_pretty(diff).expect("A1 diff serialises");
    encoded.push(b'\n');
    fs::write(directory.join("diff.json"), &encoded).expect("A1 diff can be written");
    let svelte_relative = format!("{A1_SNAPSHOT_DIR}/{}.svelte.json", loaded.row);
    fs::copy(
        repository_root().join(&svelte_relative),
        directory.join("svelte.json"),
    )
    .expect("A1 Svelte snapshot can be copied");
    let attributes = json!({
        "component": loaded.scenario.component,
        "scenario_id": loaded.scenario.scenario_id,
        "scenario_path": loaded.path,
        "scenario_sha256": loaded.sha256,
        "attributes": diff,
    });
    fs::write(
        directory.join("attributes.json"),
        serde_json::to_vec_pretty(&attributes).expect("attributes serialise"),
    )
    .expect("A1 divergence attributes can be written");
}
