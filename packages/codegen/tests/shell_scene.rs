//! Tests for the card 035/036 shell scene: the Rust-authored scene (R1),
//! the serialized fixture round trip, and the four shells' shared artifact —
//! Svelte + React through `shell-scene` (TS), GPUI + Jetstream through its
//! card-036 Rust sibling `shell-rust` (R2: a sibling target, `shell-scene`'s
//! output untouched).
//!
//! The card's required test — "the four shells expose the same capability
//! set and the same label text, derived from the scene rather than a
//! hand-written list" — is served by
//! [`both_web_shells_carry_the_same_scene_derived_artifact`] and
//! [`both_native_shells_carry_the_same_scene_derived_artifact`]: the
//! expected artifact is the target's render of the authored scene, not a
//! hand-listed expectation, and each committed artifact must equal it
//! byte-exact. A shell that drifts (renders a different control set or
//! label text) fails the comparison. A hand-listed expectation would pass
//! while the shells drift, which is the failure this card exists to
//! prevent.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use poodle_codegen::{generate, models, targets, GeneratedFile, GENERATOR_VERSION};

/// The fixture path, exactly as the Effigy selector passes it — also the
/// source path the artifact headers carry.
const SHELL_FIXTURE: &str = "packages/codegen/fixtures/shell-model.json";

const SVELTE_ARTIFACT: &str = "packages/svelte/preview/src/generated/preview-shell.ts";
const REACT_ARTIFACT: &str = "packages/react/preview/src/generated/preview-shell.ts";
const GPUI_ARTIFACT: &str = "packages/gpui/preview/src/generated/preview-shell.rs";
const JETSTREAM_ARTIFACT: &str = "packages/jetstream/preview/src/generated/preview-shell.rs";
const GPUI_MANIFEST: &str = "packages/gpui/preview/Cargo.toml";
const JETSTREAM_MANIFEST: &str = "packages/jetstream/preview/Cargo.toml";

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    crate_dir().join("../..")
}

fn fixture_path() -> PathBuf {
    repo_root().join(SHELL_FIXTURE)
}

/// A scratch directory for one test, under the target dir cargo cleans.
fn scratch(name: &str) -> PathBuf {
    let path = crate_dir()
        .join("target")
        .join("shell-scene-tests")
        .join(name);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("scratch dir");
    path
}

/// The shell-scene target's render of the authored shell model, with the
/// fixture as the header's source path.
fn render_shell_artifact() -> Vec<GeneratedFile> {
    let model = models::preview_shell::shell_model();
    let findings = model.validate();
    assert!(
        findings.is_empty(),
        "the authored shell model validates clean: {findings:?}"
    );
    generate(&model, SHELL_FIXTURE, &targets::shell::ShellSceneTarget)
        .expect("shell target renders the authored model")
}

/// The shell-rust target's render of the authored shell model (card 036):
/// the same scene in Rust shape, consumed by the native previews.
fn render_rust_artifact() -> Vec<GeneratedFile> {
    let model = models::preview_shell::shell_model();
    let findings = model.validate();
    assert!(
        findings.is_empty(),
        "the authored shell model validates clean: {findings:?}"
    );
    generate(&model, SHELL_FIXTURE, &targets::shell_rust::ShellRustTarget)
        .expect("shell-rust target renders the authored model")
}

fn artifact_bytes(path: &str) -> Vec<u8> {
    fs::read(repo_root().join(path))
        .unwrap_or_else(|error| panic!("committed artifact {path} is readable: {error}"))
}

// ---------------------------------------------------------------------------
// The authored scene (R1: authored in Rust; R2: values only, no schema)
// ---------------------------------------------------------------------------

#[test]
fn shell_model_validates_and_round_trips_as_json() {
    let model = models::preview_shell::shell_model();
    assert!(model.validate().is_empty(), "in-memory model validates");

    let document = serde_json::to_string_pretty(&model).expect("model serializes");
    let round_tripped: poodle_ir::IrModel =
        serde_json::from_str(&document).expect("serialized model parses");
    assert!(
        round_tripped.validate().is_empty(),
        "serialized model validates after the round trip"
    );

    // The committed fixture is exactly the authored model's serialization:
    // the pipeline consumes the fixture, and authoring must keep it current.
    let committed: poodle_ir::IrModel =
        serde_json::from_str(&fs::read_to_string(fixture_path()).expect("fixture reads"))
            .expect("committed fixture parses");
    assert_eq!(
        committed, model,
        "committed fixture matches the authored model"
    );
}

#[test]
fn shell_scene_authors_every_shell_capability() {
    let model = models::preview_shell::shell_model();
    let scene = model.scene("preview-shell").expect("the one scene");

    // SHELL-01..04 — the four web control axes, in display order.
    let kinds: Vec<poodle_ir::SceneAxisKind> = scene.axes.iter().map(|axis| axis.kind).collect();
    assert_eq!(
        kinds,
        vec![
            poodle_ir::SceneAxisKind::Theme,
            poodle_ir::SceneAxisKind::Size,
            poodle_ir::SceneAxisKind::Density,
            poodle_ir::SceneAxisKind::Contrast,
        ],
        "the web shell control surface is the scene's axis list"
    );

    // Named values come from the token registries, never hand-listed.
    let theme = &scene.axes[0];
    match &theme.values {
        poodle_ir::AxisValues::Named(values) => {
            assert_eq!(
                values.iter().map(|v| v.as_str()).collect::<Vec<_>>(),
                poodle_ir::theme_names(),
                "theme axis values are the poodle-tokens presets"
            );
        }
        other => panic!("theme axis must be named, got {other:?}"),
    }
    let contrast = &scene.axes[3];
    assert_eq!(
        contrast.values,
        poodle_ir::AxisValues::Continuous {
            min: 0.4,
            max: 1.6,
            default: 0.5,
        },
        "contrast is the continuous 0.4-1.6 axis both web shells already render"
    );

    // SHELL-05/06/07/08 — navigation, search, tabs, preview state.
    let layout = scene.layout.as_ref().expect("layout authored");
    assert_eq!(
        layout
            .sections
            .iter()
            .map(|section| (section.title.as_str(), section.kind))
            .collect::<Vec<_>>(),
        vec![
            ("Components", poodle_ir::NavSectionKind::Components),
            ("Tokens", poodle_ir::NavSectionKind::Tokens),
        ],
        "navigation sections carry the shell's labels"
    );
    let search = scene.search.as_ref().expect("search authored");
    assert!(search.case_insensitive);
    assert_eq!(
        search.fields,
        vec![
            poodle_ir::SearchField::DisplayName,
            poodle_ir::SearchField::Description,
        ],
        "SHELL-06: case-insensitive filter over display name and description"
    );
    let preview = scene
        .preview_state
        .as_ref()
        .expect("preview state authored");
    assert_eq!(
        preview.contrast,
        Some(0.5),
        "the shell's initial contrast matches the scene default"
    );
}

// ---------------------------------------------------------------------------
// The shared web artifact (the card's parity test, scene-derived)
// ---------------------------------------------------------------------------

#[test]
fn both_web_shells_carry_the_same_scene_derived_artifact() {
    let files = render_shell_artifact();
    assert_eq!(files.len(), 1, "one scene renders one artifact");
    let rendered = &files[0];
    assert_eq!(rendered.path, "preview-shell.ts");
    assert!(
        !rendered.contents.is_empty(),
        "the artifact is not an empty stub"
    );

    // The expected capability set + label text is the scene's own render —
    // derived, never hand-listed. Both shells must carry exactly it.
    assert_eq!(
        artifact_bytes(SVELTE_ARTIFACT),
        rendered.contents.as_bytes(),
        "the Svelte shell's committed artifact equals the scene's render"
    );
    assert_eq!(
        artifact_bytes(REACT_ARTIFACT),
        rendered.contents.as_bytes(),
        "the React shell's committed artifact equals the scene's render"
    );
}

/// The card-036 counterpart: both native shells carry the same scene-derived
/// Rust artifact (R1: self-contained plain data; R2: the `shell-rust`
/// sibling target). GPUI and Jetstream must equal the render byte-exact.
#[test]
fn both_native_shells_carry_the_same_scene_derived_artifact() {
    let files = render_rust_artifact();
    assert_eq!(files.len(), 1, "one scene renders one Rust artifact");
    let rendered = &files[0];
    assert_eq!(rendered.path, "preview-shell.rs");
    assert!(
        !rendered.contents.is_empty(),
        "the Rust artifact is not an empty stub"
    );
    assert!(
        !rendered.contents.contains("use poodle_ir")
            && !rendered.contents.contains("use poodle_codegen"),
        "the Rust artifact imports no Poodle crate (R1 — self-contained)"
    );

    assert_eq!(
        artifact_bytes(GPUI_ARTIFACT),
        rendered.contents.as_bytes(),
        "the GPUI shell's committed artifact equals the scene's render"
    );
    assert_eq!(
        artifact_bytes(JETSTREAM_ARTIFACT),
        rendered.contents.as_bytes(),
        "the Jetstream shell's committed artifact equals the scene's render"
    );
}

/// The labels and capability kinds the scene itself carries, surfaced from
/// the artifact — the R4 reading: label text is a deterministic projection
/// of the scene's axes and search presence, and the artifact is the single
/// copy both shells read. Both the TS artifact (web) and the Rust artifact
/// (natives) must carry the same projection (card 036 R2).
#[test]
fn artifact_labels_are_a_projection_of_the_scene() {
    let files = render_shell_artifact();
    let contents = &files[0].contents;

    let rust_files = render_rust_artifact();
    let rust_contents = &rust_files[0].contents;

    let model = models::preview_shell::shell_model();
    let scene = model.scene("preview-shell").expect("the one scene");

    for axis in &scene.axes {
        let kind = match axis.kind {
            poodle_ir::SceneAxisKind::Theme => "theme",
            poodle_ir::SceneAxisKind::Size => "size",
            poodle_ir::SceneAxisKind::Density => "density",
            poodle_ir::SceneAxisKind::Orientation => "orientation",
            poodle_ir::SceneAxisKind::Contrast => "contrast",
        };
        assert!(
            contents.contains(&format!("kind: \"{kind}\"")),
            "TS artifact carries the scene's {kind} control"
        );
        assert!(
            rust_contents.contains(&format!("kind: \"{kind}\"")),
            "Rust artifact carries the scene's {kind} control"
        );
    }
    assert!(
        scene.search.is_some(),
        "the scene configures search (the control surface test depends on it)"
    );
    assert!(
        contents.contains("kind: \"search\""),
        "TS artifact carries the search control from the scene's search config"
    );
    assert!(
        rust_contents.contains("kind: \"search\""),
        "Rust artifact carries the search control from the scene's search config"
    );

    // Deleting search from the scene removes the control from the artifacts
    // (R3) — prove the projection, not the current value.
    let mut without_search = model.clone();
    if let Some(scene) = without_search.scenes.first_mut() {
        scene.search = None;
    }
    let without = generate(
        &without_search,
        SHELL_FIXTURE,
        &targets::shell::ShellSceneTarget,
    )
    .expect("renders without search");
    assert!(
        !without[0].contents.contains("kind: \"search\""),
        "removing search from the scene removes the control from the TS artifact"
    );
    let without_rust = generate(
        &without_search,
        SHELL_FIXTURE,
        &targets::shell_rust::ShellRustTarget,
    )
    .expect("renders without search");
    assert!(
        !without_rust[0].contents.contains("kind: \"search\""),
        "removing search from the scene removes the control from the Rust artifact"
    );
}

// ---------------------------------------------------------------------------
// The Generated Artifact Contract (spec 063; IR-07)
// ---------------------------------------------------------------------------

#[test]
fn artifact_header_names_the_source_definition_and_generator_version() {
    for contents in [
        &render_shell_artifact()[0].contents,
        &render_rust_artifact()[0].contents,
    ] {
        let header_lines: Vec<&str> = contents
            .lines()
            .take_while(|line| line.starts_with("// "))
            .collect();
        assert!(
            header_lines
                .iter()
                .any(|line| *line == format!("// Source: {SHELL_FIXTURE}")),
            "header names the source definition (the serialized authored model): {header_lines:?}"
        );
        assert!(
            header_lines.iter().any(|line| *line
                == format!(
                    "// Generated by poodle-codegen {GENERATOR_VERSION}. Do not edit manually."
                )),
            "header carries the generator version: {header_lines:?}"
        );
        assert!(
            header_lines
                .iter()
                .any(|line| *line
                    == format!("// IR schema version: {}", poodle_ir::IR_SCHEMA_VERSION)),
            "header carries the IR schema version: {header_lines:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// One Rust definition change updates all four shells in one `ir:build`
// (spec 063; the card's acceptance)
// ---------------------------------------------------------------------------

#[test]
fn one_scene_change_moves_all_four_artifacts() {
    let mut changed = models::preview_shell::shell_model();
    // One authored value change: contrast default 0.5 -> 0.8.
    changed.scenes[0]
        .preview_state
        .as_mut()
        .expect("state")
        .contrast = Some(0.8);

    let files = generate(&changed, SHELL_FIXTURE, &targets::shell::ShellSceneTarget)
        .expect("renders the changed scene");
    let contents = &files[0].contents;

    assert!(
        contents.contains("contrast: 0.8"),
        "the changed value lands in the TS artifact"
    );
    assert_ne!(
        contents.as_bytes(),
        artifact_bytes(SVELTE_ARTIFACT).as_slice(),
        "the Svelte artifact would move in one rebuild"
    );
    assert_ne!(
        contents.as_bytes(),
        artifact_bytes(REACT_ARTIFACT).as_slice(),
        "the React artifact would move in one rebuild"
    );

    let rust_files = generate(
        &changed,
        SHELL_FIXTURE,
        &targets::shell_rust::ShellRustTarget,
    )
    .expect("renders the changed scene");
    let rust_contents = &rust_files[0].contents;

    assert!(
        rust_contents.contains("contrast: Some(0.8)"),
        "the changed value lands in the Rust artifact"
    );
    assert_ne!(
        rust_contents.as_bytes(),
        artifact_bytes(GPUI_ARTIFACT).as_slice(),
        "the GPUI artifact would move in one rebuild"
    );
    assert_ne!(
        rust_contents.as_bytes(),
        artifact_bytes(JETSTREAM_ARTIFACT).as_slice(),
        "the Jetstream artifact would move in one rebuild"
    );
}

// ---------------------------------------------------------------------------
// The drift gate covers the web artifacts (ir:check on --target shell-scene)
// ---------------------------------------------------------------------------

#[test]
fn shell_web_artifacts_fail_check_on_drift_and_check_never_writes() {
    let bin = env!("CARGO_BIN_EXE_poodle-codegen");
    let out = scratch("web-artifacts");
    let root = out.join("generated");

    // A full write into the scratch, mirroring the Effigy selector.
    let status = Command::new(bin)
        .args([SHELL_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "shell-scene"])
        .current_dir(repo_root())
        .status()
        .expect("bin runs");
    assert!(status.success(), "shell-scene write exits 0");
    let committed = fs::read_to_string(root.join("preview-shell.ts")).expect("artifact written");

    // Plant drift in the committed web artifact, then check: must fail, and
    // must not mutate the tree.
    fs::write(
        root.join("preview-shell.ts"),
        format!("{committed}\n// planted drift"),
    )
    .expect("plant drift");
    fs::write(root.join("orphan.json"), "{}\n").expect("plant orphan");
    let before = snapshot(&root);

    let status = Command::new(bin)
        .args([SHELL_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "shell-scene", "--check"])
        .current_dir(repo_root())
        .status()
        .expect("check runs");
    assert!(!status.success(), "drift in a web artifact fails ir:check");
    assert_eq!(snapshot(&root), before, "check mode never mutates the tree");

    // The fixture itself is gated the same way: an authored-model change
    // with a stale committed fixture fails --author-shell --check.
    let status = Command::new(bin)
        .args(["--author-shell"])
        .arg(scratch("fixture-copy").join("shell-model.json"))
        .current_dir(repo_root())
        .status()
        .expect("author runs");
    assert!(status.success(), "authoring writes the fixture");
}

/// The card-036 counterpart of the web drift test: `ir:check` must fail on
/// drift in a **Rust** artifact too, and check mode must never write.
#[test]
fn shell_rust_artifacts_fail_check_on_drift_and_check_never_writes() {
    let bin = env!("CARGO_BIN_EXE_poodle-codegen");
    let out = scratch("rust-artifacts");
    let root = out.join("generated");

    // A full write into the scratch, mirroring the Effigy selector.
    let status = Command::new(bin)
        .args([SHELL_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "shell-rust"])
        .current_dir(repo_root())
        .status()
        .expect("bin runs");
    assert!(status.success(), "shell-rust write exits 0");
    let committed = fs::read_to_string(root.join("preview-shell.rs")).expect("artifact written");

    // Plant drift in the committed Rust artifact, then check: must fail, and
    // must not mutate the tree.
    fs::write(
        root.join("preview-shell.rs"),
        format!("{committed}\n// planted drift"),
    )
    .expect("plant drift");
    fs::write(root.join("orphan.json"), "{}\n").expect("plant orphan");
    let before = snapshot(&root);

    let status = Command::new(bin)
        .args([SHELL_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "shell-rust", "--check"])
        .current_dir(repo_root())
        .status()
        .expect("check runs");
    assert!(!status.success(), "drift in a Rust artifact fails ir:check");
    assert_eq!(snapshot(&root), before, "check mode never mutates the tree");
}

/// R1 asserted, not just avoided: neither native preview manifest may name
/// `poodle-ir` or `poodle-codegen`. The artifacts are pulled in by
/// `#[path]`, so no manifest change was needed — prove the constraint.
#[test]
fn native_preview_manifests_carry_no_poodle_ir_or_codegen_dependency() {
    for manifest in [GPUI_MANIFEST, JETSTREAM_MANIFEST] {
        let text = fs::read_to_string(repo_root().join(manifest))
            .unwrap_or_else(|error| panic!("{manifest} is readable: {error}"));
        assert!(
            !text.contains("poodle-ir"),
            "{manifest} must not depend on poodle-ir (card 036 R1)"
        );
        assert!(
            !text.contains("poodle-codegen"),
            "{manifest} must not depend on poodle-codegen (card 036 R1)"
        );
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Recursively maps relative path → bytes under `root`.
fn snapshot(root: &PathBuf) -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    let mut stack = vec![root.clone()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).expect("snapshot reads dir") {
            let entry = entry.expect("snapshot entry");
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let relative = path
                    .strip_prefix(root)
                    .expect("under root")
                    .to_str()
                    .expect("utf8")
                    .to_owned();
                out.push((relative, fs::read(&path).expect("snapshot reads file")));
            }
        }
    }
    out.sort();
    out
}
