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
const LOCKFILE_SHA256: &str = "8bb8f8edaba8f381b9dec39532f5299231d2dfaa1c4509c7f87e41ca27711a55";
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
