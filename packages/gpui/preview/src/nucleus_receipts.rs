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
const DISTRIBUTION: &str = "workspace";

/// The canonical resolution order the receipt contract fixes: the external
/// dependency first, then the four workspace packages. The emitter derives
/// exactly these entries, in this order, from the lockfile bytes.
const LOCKED_PACKAGES: [&str; 5] = [
    "gpui",
    "poodle-gpui",
    "poodle-gpui-preview",
    "poodle-node",
    "poodle-render",
];

/// The one selected package that is not a workspace member: its receipt entry
/// carries the normalized `crates.io` source and the lockfile checksum.
const REGISTRY_PACKAGE: &str = "gpui";

#[derive(Debug, PartialEq, Serialize)]
struct LockedPackage {
    name: String,
    version: String,
    source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    checksum: Option<String>,
}

/// The lock identity a receipt publishes, derived from the exact lockfile
/// bytes. Neither the SHA-256 nor a Poodle release version is copied into this
/// source, so a lockstep release never has to edit the emitter.
#[derive(Debug)]
struct LockProvenance {
    lockfile_sha256: String,
    lock_resolution: Vec<LockedPackage>,
}

/// The receipt-relevant fields of one `[[package]]` block. `name` and
/// `version` are required; `source` and `checksum` are optional in the file and
/// validated against the package's expected form at resolution time.
#[derive(Default)]
struct ParsedLockPackage {
    name: Option<String>,
    version: Option<String>,
    source: Option<String>,
    checksum: Option<String>,
}

impl ParsedLockPackage {
    /// A block without a name or version is malformed, not skippable: a lock
    /// the emitter cannot read exactly is not evidence.
    fn require(self) -> Result<Self, String> {
        let name = self
            .name
            .as_deref()
            .ok_or_else(|| "Cargo.lock has a [[package]] block without a name".to_owned())?;
        let version = self
            .version
            .as_deref()
            .ok_or_else(|| format!("Cargo.lock package {name} has no version"))?;
        if version.is_empty() {
            return Err(format!("Cargo.lock package {name} has an empty version"));
        }
        Ok(self)
    }
}

/// Parse a Cargo.lock field line (`key = "value"`). Lines the receipt does not
/// resolve (dependency array items, blank lines) return `None`.
fn lock_field(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(" = ")?;
    let value = value.strip_prefix('"')?.strip_suffix('"')?;
    if key.is_empty()
        || !key
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
    {
        return None;
    }
    Some((key, value))
}

/// Split the lockfile into `[[package]]` blocks and read the four fields the
/// receipt resolves. Everything before the first package table is Cargo's
/// preamble and is ignored; a later table that is not `[[package]]` ends the
/// package section. A block missing a name or version, or repeating a field,
/// is rejected rather than silently dropped.
fn parse_lock_packages(text: &str) -> Result<Vec<ParsedLockPackage>, String> {
    let mut packages = Vec::new();
    let mut block: Option<ParsedLockPackage> = None;
    for line in text.lines() {
        let line = line.trim_end();
        if line == "[[package]]" {
            if let Some(previous) = block.take() {
                packages.push(previous);
            }
            block = Some(ParsedLockPackage::default());
            continue;
        }
        let Some(package) = block.as_mut() else {
            continue;
        };
        if line.starts_with('[') {
            if let Some(previous) = block.take() {
                packages.push(previous);
            }
            continue;
        }
        let Some((key, value)) = lock_field(line) else {
            continue;
        };
        let slot = match key {
            "name" => &mut package.name,
            "version" => &mut package.version,
            "source" => &mut package.source,
            "checksum" => &mut package.checksum,
            _ => continue,
        };
        if slot.is_some() {
            return Err(format!(
                "Cargo.lock repeats the `{key}` field in one package block"
            ));
        }
        *slot = Some(value.to_owned());
    }
    if let Some(last) = block.take() {
        packages.push(last);
    }
    let mut parsed = Vec::with_capacity(packages.len());
    for package in packages {
        parsed.push(package.require()?);
    }
    Ok(parsed)
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// Derive the published lock identity from the exact lockfile bytes. The lock
/// is evidence, so each selected package must appear exactly once with a
/// complete, expected source form. Missing, duplicate, incomplete,
/// unexpected-source, or checksum-less registry entries fail closed; nothing
/// falls back to a stale literal.
fn lock_provenance(lock_bytes: &[u8]) -> Result<LockProvenance, String> {
    let text =
        std::str::from_utf8(lock_bytes).map_err(|_| "Cargo.lock is not valid UTF-8".to_owned())?;
    let packages = parse_lock_packages(text)?;
    let mut lock_resolution = Vec::with_capacity(LOCKED_PACKAGES.len());
    for name in LOCKED_PACKAGES {
        let mut matches = packages
            .iter()
            .filter(|package| package.name.as_deref() == Some(name));
        let Some(package) = matches.next() else {
            return Err(format!("Cargo.lock is missing the required package {name}"));
        };
        if matches.next().is_some() {
            return Err(format!(
                "Cargo.lock declares the required package {name} more than once"
            ));
        }
        let version = package
            .version
            .clone()
            .expect("a parsed package has a version");
        let (source, checksum) = if name == REGISTRY_PACKAGE {
            let source = package
                .source
                .as_deref()
                .ok_or_else(|| format!("Cargo.lock package {name} has no source"))?;
            if !source.starts_with("registry+") {
                return Err(format!(
                    "Cargo.lock package {name} has unexpected source {source}"
                ));
            }
            let checksum = package
                .checksum
                .as_deref()
                .ok_or_else(|| format!("Cargo.lock package {name} has no checksum"))?;
            if !is_sha256(checksum) {
                return Err(format!("Cargo.lock package {name} has an invalid checksum"));
            }
            ("crates.io", Some(checksum.to_owned()))
        } else {
            if package.source.is_some() {
                return Err(format!(
                    "Cargo.lock workspace package {name} declares a source"
                ));
            }
            if package.checksum.is_some() {
                return Err(format!(
                    "Cargo.lock workspace package {name} declares a checksum"
                ));
            }
            ("workspace", None)
        };
        lock_resolution.push(LockedPackage {
            name: name.to_owned(),
            version,
            source: source.to_owned(),
            checksum,
        });
    }
    Ok(LockProvenance {
        lockfile_sha256: sha256_hex(lock_bytes),
        lock_resolution,
    })
}

/// Read the committed lockfile once and derive its receipt provenance. The
/// exact bytes are hashed; the resolution comes from those same bytes.
fn lock_provenance_from_disk(root: &PathBuf) -> LockProvenance {
    let bytes = fs::read(root.join(LOCKFILE))
        .unwrap_or_else(|error| panic!("GPUI preview {LOCKFILE} is unreadable: {error}"));
    lock_provenance(&bytes).unwrap_or_else(|error| {
        panic!("GPUI preview {LOCKFILE} is not usable receipt provenance: {error}")
    })
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
    let provenance = lock_provenance_from_disk(&root);
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
        lockfile_sha256: provenance.lockfile_sha256,
        lock_resolution: provenance.lock_resolution,
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
    /// Host-owned append that must not move focus. The row's proof maps the
    /// item into the production transcript host before the final snapshot.
    ProgrammaticAppend { item: Value },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct A1Exclusion {
    pub attribute: String,
    pub reason: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct A1Capture {
    pub width: u32,
    pub height: u32,
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
    /// Fixed logical capture viewport for the cohort fixture kind. The A1
    /// extractors carry this field but do not use it for DOM layout.
    pub capture: A1Capture,
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
    assert!(
        scenario.capture.width > 0,
        "{relative} capture width must be positive"
    );
    assert!(
        scenario.capture.height > 0,
        "{relative} capture height must be positive"
    );
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
    let provenance = lock_provenance_from_disk(&root);
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
        "lockfile_sha256": provenance.lockfile_sha256,
        "lock_resolution": provenance.lock_resolution,
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

/// Focused laws for the lock-provenance helper. The clockwork release conflict
/// this task removes was a hard-coded hash and four hard-coded versions, so
/// these laws read the committed lock, a planted release lock, changed bytes,
/// and every rejected package shape. They deliberately embed no current Poodle
/// version: a lockstep release changes the lockfile, not this source.
#[cfg(test)]
mod receipt_lock_tests {
    use super::*;

    fn committed_lock_text() -> String {
        fs::read_to_string(repository_root().join(LOCKFILE))
            .expect("the committed GPUI preview Cargo.lock is readable")
    }

    /// A well-formed lock identical to the committed one apart from the four
    /// workspace package versions, which carry the planted release version.
    /// This is the disposable versioned lock a release will produce.
    fn planted_lock(version: &str) -> String {
        let mut lock = committed_lock_text();
        for name in &LOCKED_PACKAGES[1..] {
            let block = block_of(&lock, name).to_owned();
            let line = matching_line(&block, name, "version").to_owned();
            lock = replace_in_block(&lock, name, &line, &format!("version = \"{version}\""));
        }
        lock
    }

    /// The exact `[[package]]` block for one package, from its table header to
    /// the next table header.
    fn block_of<'a>(lock: &'a str, name: &str) -> &'a str {
        let marker = format!("[[package]]\nname = \"{name}\"");
        let start = lock
            .find(&marker)
            .unwrap_or_else(|| panic!("fixture lock has no {name} package block"));
        let after = start + marker.len();
        let end = lock[after..]
            .find("[[package]]")
            .map_or(lock.len(), |offset| after + offset);
        &lock[start..end]
    }

    fn matching_line<'a>(block: &'a str, name: &str, field: &str) -> &'a str {
        block
            .lines()
            .find(|line| line.starts_with(&format!("{field} = ")))
            .unwrap_or_else(|| panic!("{name} block has no {field} field"))
    }

    fn replace_in_block(lock: &str, name: &str, from: &str, to: &str) -> String {
        let block = block_of(lock, name);
        let mutated = block.replacen(from, to, 1);
        assert_ne!(mutated, block, "fixture replacement did not apply");
        lock.replacen(block, &mutated, 1)
    }

    fn remove_field(lock: &str, name: &str, field: &str) -> String {
        let block = block_of(lock, name);
        let line = matching_line(block, name, field);
        replace_in_block(lock, name, &format!("{line}\n"), "")
    }

    fn derived(bytes: &[u8]) -> LockProvenance {
        lock_provenance(bytes).expect("the fixture lock derives provenance")
    }

    fn entry<'a>(provenance: &'a LockProvenance, name: &str) -> &'a LockedPackage {
        provenance
            .lock_resolution
            .iter()
            .find(|package| package.name == name)
            .unwrap_or_else(|| panic!("resolution has no {name}"))
    }

    fn rejection(lock: &str) -> String {
        lock_provenance(lock.as_bytes()).expect_err("the malformed lock must be rejected")
    }

    #[test]
    fn receipt_lock_provenance_is_derived_from_the_committed_lock() {
        let lock = committed_lock_text();
        let provenance = derived(lock.as_bytes());
        assert_eq!(provenance.lockfile_sha256, sha256_hex(lock.as_bytes()));
        let names: Vec<&str> = provenance
            .lock_resolution
            .iter()
            .map(|package| package.name.as_str())
            .collect();
        assert_eq!(names, LOCKED_PACKAGES);
        assert_eq!(entry(&provenance, REGISTRY_PACKAGE).source, "crates.io");
        assert!(
            entry(&provenance, REGISTRY_PACKAGE)
                .checksum
                .as_deref()
                .is_some_and(|checksum| is_sha256(checksum)),
            "the registry package carries a SHA-256 checksum"
        );
        for name in &LOCKED_PACKAGES[1..] {
            let package = entry(&provenance, name);
            assert_eq!(package.source, "workspace");
            assert_eq!(package.checksum, None);
        }
        // Versions are read from the lock, never embedded: every entry equals
        // its block's own version line, and the preview package equals the
        // Cargo.toml version compiled into this evidence target.
        for name in LOCKED_PACKAGES {
            let block = block_of(&lock, name);
            let version_line = matching_line(block, name, "version");
            assert_eq!(
                entry(&provenance, name).version,
                version_line
                    .trim_start_matches("version = \"")
                    .trim_end_matches('"')
            );
        }
        assert_eq!(
            entry(&provenance, "poodle-gpui-preview").version,
            env!("CARGO_PKG_VERSION")
        );
    }

    #[test]
    fn receipt_lock_hash_tracks_changed_bytes() {
        let lock = committed_lock_text();
        let baseline = derived(lock.as_bytes());
        let mut changed_bytes = lock.into_bytes();
        changed_bytes[0] = b'!';
        let changed = derived(&changed_bytes);
        assert_eq!(changed.lockfile_sha256, sha256_hex(&changed_bytes));
        assert_ne!(changed.lockfile_sha256, baseline.lockfile_sha256);
        assert_eq!(changed.lock_resolution, baseline.lock_resolution);
    }

    #[test]
    fn receipt_lock_derives_a_planted_release_version() {
        let current = committed_lock_text();
        let baseline = derived(current.as_bytes());
        let planted = planted_lock("0.4.0");
        assert_ne!(planted, current);
        let provenance = derived(planted.as_bytes());
        assert_eq!(provenance.lockfile_sha256, sha256_hex(planted.as_bytes()));
        assert_ne!(provenance.lockfile_sha256, baseline.lockfile_sha256);
        for name in &LOCKED_PACKAGES[1..] {
            assert_eq!(entry(&provenance, name).version, "0.4.0");
        }
        // The external entry is untouched: only the Poodle release moved.
        assert_eq!(
            entry(&provenance, REGISTRY_PACKAGE).version,
            entry(&baseline, REGISTRY_PACKAGE).version
        );
    }

    #[test]
    fn receipt_lock_rejects_a_missing_package() {
        let planted = planted_lock("0.4.0");
        let block = block_of(&planted, "poodle-node");
        let removed = planted.replacen(block, "", 1);
        let error = rejection(&removed);
        assert!(error.contains("missing"), "{error}");
        assert!(error.contains("poodle-node"), "{error}");
    }

    #[test]
    fn receipt_lock_rejects_a_duplicate_package() {
        let planted = planted_lock("0.4.0");
        let duplicated = format!("{planted}{}", block_of(&planted, "poodle-gpui"));
        let error = rejection(&duplicated);
        assert!(error.contains("more than once"), "{error}");
        assert!(error.contains("poodle-gpui"), "{error}");
    }

    #[test]
    fn receipt_lock_rejects_an_incomplete_package() {
        let planted = planted_lock("0.4.0");
        let incomplete = remove_field(&planted, "poodle-render", "version");
        let error = rejection(&incomplete);
        assert!(error.contains("poodle-render"), "{error}");
        assert!(error.contains("version"), "{error}");
    }

    #[test]
    fn receipt_lock_rejects_a_workspace_package_with_a_source() {
        let planted = planted_lock("0.4.0");
        let mutated = replace_in_block(
            &planted,
            "poodle-gpui",
            "name = \"poodle-gpui\"\n",
            "name = \"poodle-gpui\"\nsource = \"registry+https://example.invalid/index\"\n",
        );
        let error = rejection(&mutated);
        assert!(error.contains("workspace package"), "{error}");
        assert!(error.contains("source"), "{error}");
    }

    #[test]
    fn receipt_lock_rejects_a_workspace_package_with_a_checksum() {
        let planted = planted_lock("0.4.0");
        let mutated = replace_in_block(
            &planted,
            "poodle-node",
            "name = \"poodle-node\"\n",
            &format!(
                "name = \"poodle-node\"\nchecksum = \"{}\"\n",
                "a".repeat(64)
            ),
        );
        let error = rejection(&mutated);
        assert!(error.contains("workspace package"), "{error}");
        assert!(error.contains("checksum"), "{error}");
    }

    #[test]
    fn receipt_lock_rejects_a_registry_package_without_a_source() {
        let planted = planted_lock("0.4.0");
        let mutated = remove_field(&planted, REGISTRY_PACKAGE, "source");
        let error = rejection(&mutated);
        assert!(error.contains(REGISTRY_PACKAGE), "{error}");
        assert!(error.contains("no source"), "{error}");
    }

    #[test]
    fn receipt_lock_rejects_an_unexpected_registry_source() {
        let planted = planted_lock("0.4.0");
        let block = block_of(&planted, REGISTRY_PACKAGE);
        let source_line = matching_line(block, REGISTRY_PACKAGE, "source");
        let mutated = replace_in_block(
            &planted,
            REGISTRY_PACKAGE,
            source_line,
            "source = \"git+https://example.invalid/gpui\"",
        );
        let error = rejection(&mutated);
        assert!(error.contains("unexpected source"), "{error}");
    }

    #[test]
    fn receipt_lock_rejects_a_registry_package_without_a_checksum() {
        let planted = planted_lock("0.4.0");
        let mutated = remove_field(&planted, REGISTRY_PACKAGE, "checksum");
        let error = rejection(&mutated);
        assert!(error.contains(REGISTRY_PACKAGE), "{error}");
        assert!(error.contains("no checksum"), "{error}");
    }

    #[test]
    fn receipt_lock_rejects_a_malformed_checksum() {
        let planted = planted_lock("0.4.0");
        let block = block_of(&planted, REGISTRY_PACKAGE);
        let checksum_line = matching_line(block, REGISTRY_PACKAGE, "checksum");
        let mutated = replace_in_block(
            &planted,
            REGISTRY_PACKAGE,
            checksum_line,
            "checksum = \"not-a-sha256\"",
        );
        let error = rejection(&mutated);
        assert!(error.contains("invalid checksum"), "{error}");
    }
}
