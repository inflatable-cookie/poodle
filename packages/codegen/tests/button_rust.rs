//! Tests for card 042 — the Button vertical slice, native half: the
//! `button-rust` target (R2 — a sibling of `button-ts`, which is
//! byte-frozen by b041's tests) emitting the self-contained Rust artifact
//! `poodle-render` consumes (R1/R1a — plain data, no `use` of any Poodle
//! crate, under `packages/render/src/generated/`), and the R3 vocabulary
//! contract the render reads it through.
//!
//! The card's required parity test is
//! [`render_artifact_matches_the_target_render`]: the expected artifact is
//! the target's render of the authored definition, never a hand-listed
//! expectation, and the committed render artifact must equal it byte-exact.
//! The R2 proof — "one definition change updates every artifact in one
//! `ir:build`" — is [`one_definition_change_moves_all_three_button_artifacts`]
//! (rename a state attribute, all three committed artifacts move in one
//! build) and is proven live across all four previews in the card's step 5.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use poodle_codegen::{generate, models, targets, GeneratedFile, GENERATOR_VERSION};

/// The fixture path, exactly as the Effigy selector passes it — also the
/// source path the artifact header carries.
const BUTTON_FIXTURE: &str = "packages/codegen/fixtures/button-model.json";

/// The committed render artifact (card 042 R1a: the artifact lives in the
/// package that ships it — `poodle-render` is the consumer, so it lands
/// under `packages/render/src/generated/`, not a preview and not codegen).
const RENDER_ARTIFACT: &str = "packages/render/src/generated/button.rs";

/// The web artifacts the same definition drives, at the post-review
/// locations (the b041 review moved emission into the component packages).
const SVELTE_ARTIFACT: &str = "packages/svelte/components/src/generated/button/index.ts";
const REACT_ARTIFACT: &str = "packages/react/components/src/generated/button/index.ts";

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    crate_dir().join("../..")
}

/// A scratch directory for one test, under the target dir cargo cleans.
fn scratch(name: &str) -> PathBuf {
    let path = crate_dir()
        .join("target")
        .join("button-rust-tests")
        .join(name);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("scratch dir");
    path
}

/// The button-rust target's render of the authored Button model, with the
/// fixture as the header's source path.
fn render_button_artifact() -> Vec<GeneratedFile> {
    let model = models::button::button_model();
    let findings = model.validate();
    assert!(
        findings.is_empty(),
        "the authored Button model validates clean: {findings:?}"
    );
    generate(
        &model,
        BUTTON_FIXTURE,
        &targets::button_rust::ButtonRustTarget,
    )
    .expect("button-rust target renders the authored model")
}

fn artifact_bytes(path: &str) -> Vec<u8> {
    fs::read(repo_root().join(path))
        .unwrap_or_else(|error| panic!("committed artifact {path} is readable: {error}"))
}

// ---------------------------------------------------------------------------
// The shared native artifact (the card's parity test, definition-derived)
// ---------------------------------------------------------------------------

#[test]
fn render_artifact_matches_the_target_render() {
    let files = render_button_artifact();
    assert_eq!(files.len(), 1, "one component renders one artifact");
    let rendered = &files[0];
    assert_eq!(
        rendered.path, "button.rs",
        "one file per component, named after the component id"
    );
    assert!(
        !rendered.contents.is_empty(),
        "the artifact is not an empty stub"
    );

    // The expected artifact is the definition's own render — derived, never
    // hand-listed. The render crate must carry exactly it.
    assert_eq!(
        artifact_bytes(RENDER_ARTIFACT),
        rendered.contents.as_bytes(),
        "the committed render artifact equals the definition's render"
    );

    // R1: the artifact is self-contained — no `use` of any Poodle crate,
    // the way `shell-rust`'s artifacts are (card 036 R1).
    assert!(
        !rendered.contents.contains("use poodle_")
            && !rendered.contents.contains("use crate::")
            && !rendered.contents.contains("poodle_ir")
            && !rendered.contents.contains("poodle_codegen"),
        "the artifact imports no Poodle crate (R1)"
    );
}

/// The R3 vocabulary: the artifact carries the variant/tone/density member
/// lists, the parts (with DOM classes), the eleven state attributes
/// (names, forms, emission policies, value domains), and the recipe hooks
/// — what the render reads instead of hard-coding.
#[test]
fn render_artifact_carries_the_rendered_vocabulary() {
    let contents = &render_button_artifact()[0].contents;
    let model = models::button::button_model();
    let component = model
        .components
        .iter()
        .find(|component| component.id.as_str() == "button")
        .expect("the one component");

    // The shared-type member lists the render discriminates on (R3).
    assert!(
        contents.contains("variants: &[\"primary\", \"secondary\", \"ghost\"]"),
        "variant vocabulary"
    );
    assert!(
        contents.contains("tones: &[\"default\", \"danger\", \"success\", \"warning\"]"),
        "tone vocabulary"
    );
    assert!(
        contents.contains("densities: &[\"compact\", \"default\", \"comfortable\"]"),
        "density vocabulary"
    );

    // Parts: every part id carries its DOM class projection (B §2).
    for part in &component.parts {
        assert!(
            contents.contains(&format!("id: \"{}\"", part.id)),
            "artifact carries part '{}'",
            part.id
        );
    }
    assert!(contents.contains("dom_class: \"poodle-button\""));
    assert!(contents.contains("dom_class: \"poodle-button__spinner\""));
    assert!(contents.contains("dom_class: \"poodle-button__label\""));
    assert!(contents.contains("dom_class: \"poodle-button__chevron\""));
    assert_eq!(
        contents
            .matches("dom_class: \"poodle-button__icon\"")
            .count(),
        2,
        "both icon parts share the icon span class (B §2)"
    );

    // Attributes: names, forms, emission policies, and value domains.
    for attribute in &component.attributes {
        assert!(
            contents.contains(&format!("id: \"{}\"", attribute.id)),
            "artifact carries attribute '{}'",
            attribute.id
        );
        assert!(
            contents.contains(&format!("name: \"{}\"", attribute.name)),
            "artifact carries the DOM name '{}' of '{}'",
            attribute.name,
            attribute.id
        );
    }
    // The value domains the render's treatments discriminate on (R3).
    assert!(
        contents.contains("values: Some(&[\"primary\", \"secondary\", \"ghost\"])"),
        "variant domain"
    );
    assert!(
        contents.contains("values: Some(&[\"danger\", \"success\", \"warning\"])"),
        "tone domain omits the default"
    );
    assert!(
        contents.contains("values: Some(&[\"xs\", \"sm\", \"md\", \"lg\", \"xl\"])"),
        "size domain"
    );
    assert!(
        contents.contains("values: Some(&[\"content\"])"),
        "fit domain omits the default"
    );
    assert!(
        contents.contains("values: Some(&[\"true\", \"false\"])"),
        "boolean domains"
    );
    assert!(
        contents.contains("form: \"presence-only\""),
        "presence-only form is recorded"
    );

    // Recipe hooks: every declared hook and its chain lands in the artifact.
    for hook in &component.recipe_hooks {
        assert!(
            contents.contains(&format!("hook: \"{}\"", hook.hook)),
            "artifact carries recipe hook '{}'",
            hook.hook
        );
    }
    assert!(
        contents.contains("kind: \"recipe-hook\"")
            && contents.contains("kind: \"component-variable\"")
            && contents.contains("kind: \"token\""),
        "chain kinds are recorded"
    );
}

// ---------------------------------------------------------------------------
// The Generated Artifact Contract (spec 063; IR-07)
// ---------------------------------------------------------------------------

#[test]
fn artifact_header_names_the_source_definition_and_generator_version() {
    let contents = &render_button_artifact()[0].contents;
    let header_lines: Vec<&str> = contents
        .lines()
        .take_while(|line| line.starts_with("// "))
        .collect();
    assert!(
        header_lines
            .iter()
            .any(|line| *line == format!("// Source: {BUTTON_FIXTURE}")),
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
            .any(|line| *line == format!("// IR schema version: {}", poodle_ir::IR_SCHEMA_VERSION)),
        "header carries the IR schema version: {header_lines:?}"
    );
}

// ---------------------------------------------------------------------------
// One Rust definition change updates every Button artifact in one `ir:build`
// (spec 063; the card's acceptance; the R2 proof at the artifact level)
// ---------------------------------------------------------------------------

#[test]
fn one_definition_change_moves_all_three_button_artifacts() {
    let mut changed = models::button::button_model();
    // One authored value change — the card's R2 proof, encoded: rename a
    // state attribute (`data-tone` → `data-tone-level`), the same value
    // the card's step 5 changes live across all four previews.
    let component = changed
        .components
        .iter_mut()
        .find(|component| component.id.as_str() == "button")
        .expect("the one component");
    let tone = component
        .attributes
        .iter_mut()
        .find(|attribute| attribute.id.as_str() == "tone")
        .expect("the tone attribute");
    tone.name = "data-tone-level".to_owned();

    // One definition change renders every Button artifact differently.
    let rust = generate(
        &changed,
        BUTTON_FIXTURE,
        &targets::button_rust::ButtonRustTarget,
    )
    .expect("renders the changed definition");
    assert!(
        rust[0].contents.contains("name: \"data-tone-level\""),
        "the renamed attribute lands in the render artifact"
    );
    assert_ne!(
        rust[0].contents.as_bytes(),
        artifact_bytes(RENDER_ARTIFACT).as_slice(),
        "the render artifact would move in one rebuild"
    );

    let web = generate(&changed, BUTTON_FIXTURE, &targets::button::ButtonTarget)
        .expect("renders the changed definition");
    assert!(
        web[0].contents.contains("name: \"data-tone-level\""),
        "the renamed attribute lands in the web artifact"
    );
    assert_ne!(
        web[0].contents.as_bytes(),
        artifact_bytes(SVELTE_ARTIFACT).as_slice(),
        "the Svelte artifact would move in one rebuild"
    );
    assert_ne!(
        web[0].contents.as_bytes(),
        artifact_bytes(REACT_ARTIFACT).as_slice(),
        "the React artifact would move in one rebuild"
    );
}

// ---------------------------------------------------------------------------
// R1 asserted, not just avoided: the render manifest gains no dependency
// ---------------------------------------------------------------------------

#[test]
fn render_manifest_carries_no_poodle_ir_or_codegen_dependency() {
    let manifest = fs::read_to_string(repo_root().join("packages/render/Cargo.toml"))
        .expect("render manifest reads");
    assert!(
        !manifest.contains("poodle-ir") && !manifest.contains("poodle-codegen"),
        "poodle-render must not depend on poodle-ir or poodle-codegen (b003 R2; card 042 R1)"
    );
    assert!(
        manifest.contains("poodle-node") && manifest.contains("poodle-specs"),
        "the manifest still carries the existing render dependencies"
    );
}

// ---------------------------------------------------------------------------
// The drift gate covers the render artifact (ir:check on --target button-rust)
// ---------------------------------------------------------------------------

#[test]
fn button_rust_artifacts_fail_check_on_drift_and_check_never_writes() {
    let bin = env!("CARGO_BIN_EXE_poodle-codegen");
    let out = scratch("render-artifacts");
    let root = out.join("generated");

    // A full write into the scratch, mirroring the Effigy selector.
    let status = Command::new(bin)
        .args([BUTTON_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "button-rust"])
        .current_dir(repo_root())
        .status()
        .expect("bin runs");
    assert!(status.success(), "button-rust write exits 0");
    let committed = fs::read_to_string(root.join("button.rs")).expect("artifact written");

    // Plant drift in the committed render artifact, then check: must fail,
    // and must not mutate the tree.
    fs::write(
        root.join("button.rs"),
        format!("{committed}\n// planted drift"),
    )
    .expect("plant drift");
    fs::write(root.join("orphan.json"), "{}\n").expect("plant orphan");
    let before = snapshot(&root);

    let status = Command::new(bin)
        .args([BUTTON_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "button-rust", "--check"])
        .current_dir(repo_root())
        .status()
        .expect("check runs");
    assert!(
        !status.success(),
        "drift in a render artifact fails ir:check"
    );
    assert_eq!(snapshot(&root), before, "check mode never mutates the tree");
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
