//! Tests for card 049 — the TextInput environment-boundary proof, native
//! half: the `text-input-rust` target (R1 — a sibling of `text-input-ts`,
//! which is byte-frozen by b048's tests) emitting the self-contained Rust
//! artifact `poodle-render` consumes (R1/R1a — plain data, no `use` of any
//! Poodle crate, under `packages/render/src/generated/`), and the R3
//! vocabulary contract the render reads it through.
//!
//! The card's required parity test is
//! [`render_artifact_matches_the_target_render`]: the expected artifact is
//! the target's render of the authored definition, never a hand-listed
//! expectation, and the committed render artifact must equal it byte-exact.
//! The four-runtime proof — "one definition change reaches all four
//! runtimes" — is
//! [`one_definition_change_moves_all_three_text_input_artifacts`] (rename
//! a state attribute and a TXT-16 style prop, all three committed
//! artifacts move in one build) and is proven live across all four
//! previews in the card's step 7.
//!
//! R2/R3's subject — the typed capability boundary — is carried into the
//! artifact as the `capabilities` list
//! ([`render_artifact_carries_the_rendered_vocabulary`]): the render
//! wires the edit handlers the declaration names, and the list is
//! component-scoped, never per-runtime
//! ([`a_capability_gap_is_visible_in_the_artifact_and_moves_the_render`]).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use poodle_codegen::{generate, models, targets, GeneratedFile, GENERATOR_VERSION};

/// The fixture path, exactly as the Effigy selector passes it — also the
/// source path the artifact header carries.
const TEXT_INPUT_FIXTURE: &str = "packages/codegen/fixtures/text-input-model.json";

/// The committed render artifact (card 049 R1a: the artifact lives in the
/// package that ships it — `poodle-render` is the consumer, so it lands
/// under `packages/render/src/generated/`, not a preview and not codegen).
/// It sits in its own nested root (`generated/text-input/`) — the
/// disjoint-roots layout card 041 established for shared `generated/`
/// directories: the top level belongs to `button-rust`, whose orphan sweep
/// would delete a sibling's file.
const RENDER_ARTIFACT: &str = "packages/render/src/generated/text-input/index.rs";

/// The web artifacts the same definition drives (card 048 locations).
const SVELTE_ARTIFACT: &str = "packages/svelte/components/src/generated/text-input/index.ts";
const REACT_ARTIFACT: &str = "packages/react/components/src/generated/text-input/index.ts";

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    crate_dir().join("../..")
}

/// A scratch directory for one test, under the target dir cargo cleans.
fn scratch(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    if path.exists() {
        fs::remove_dir_all(&path).expect("scratch clean");
    }
    path
}

/// The text-input-rust target's render of the authored TextInput model,
/// with the fixture as the header's source path.
fn render_text_input_artifact() -> Vec<GeneratedFile> {
    let model = models::text_input::text_input_model();
    generate(
        &model,
        TEXT_INPUT_FIXTURE,
        &targets::text_input_rust::TextInputRustTarget,
    )
    .expect("text-input-rust renders the authored model")
}

fn artifact_bytes(path: &str) -> Vec<u8> {
    fs::read(repo_root().join(path)).expect("committed artifact reads")
}

// ---------------------------------------------------------------------------
// The shared native artifact (the card's parity test, definition-derived)
// ---------------------------------------------------------------------------

#[test]
fn render_artifact_matches_the_target_render() {
    let rendered = &render_text_input_artifact()[0];
    assert_eq!(
        rendered.path, "index.rs",
        "one file per component model, under the target's nested root"
    );
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

/// The R3 vocabulary: the artifact carries the shared-type member lists,
/// the ten parts (with the DOM class projection), the four data-*
/// attributes (names, forms, emission policies, value domains), the five
/// TXT-16 padding hooks with their source fields, the recipe hooks, and
/// the typed capability boundary — what the render reads instead of
/// hard-coding.
#[test]
fn render_artifact_carries_the_rendered_vocabulary() {
    let contents = &render_text_input_artifact()[0].contents;
    let model = models::text_input::text_input_model();
    let component = model
        .components
        .iter()
        .find(|component| component.id.as_str() == "text-input")
        .expect("the one component");

    // The shared-type member lists the render discriminates on (R3).
    assert!(
        contents.contains("types: &[\"text\", \"multiline\", \"search\", \"slug\"]"),
        "type vocabulary (the data-type domain)"
    );
    assert!(
        contents.contains("validation_states: &[\"none\", \"invalid\", \"valid\", \"pending\"]"),
        "validation-state vocabulary (the data-validation-state domain)"
    );
    assert!(
        contents.contains("sizes: &[\"xs\", \"sm\", \"md\", \"lg\", \"xl\"]"),
        "size vocabulary (the size ladder)"
    );
    assert!(
        contents.contains("densities: &[\"compact\", \"default\", \"comfortable\"]"),
        "density vocabulary (the density adjustments)"
    );

    // The typed capability boundary (R2/R3) — the serde names in
    // declaration order. The render wires the edit handlers these name.
    assert!(
        contents.contains(
            "capabilities: &[\n        \"focus\",\n        \"text-editing\",\n        \"ime\",\n        \"clipboard\",\n        \"measurement\",\n        \"timers\",\n    ],"
        ),
        "the six environment capabilities + timers, in the authored order (b048's table)"
    );

    // Parts: every part id carries its DOM class projection (T §2).
    for part in &component.parts {
        assert!(
            contents.contains(&format!("id: \"{}\"", part.id)),
            "artifact carries part '{}'",
            part.id
        );
    }
    assert!(contents.contains("dom_class: \"poodle-text-input\""));
    assert!(contents.contains("dom_class: \"poodle-text-input__field\""));
    assert!(contents.contains("dom_class: \"poodle-text-input__control\""));
    assert!(contents.contains("dom_class: \"poodle-text-input__clear\""));
    // The affix and affordance parts carry base + modifier classes.
    assert!(contents
        .contains("dom_class: \"poodle-text-input__affix poodle-text-input__affix--prefix\""));
    assert!(contents
        .contains("dom_class: \"poodle-text-input__affix poodle-text-input__affix--suffix\""));
    assert!(contents.contains(
        "dom_class: \"poodle-text-input__affordance poodle-text-input__affordance--leading\""
    ));

    // Attributes: the four data-* names, forms, emission policies, and
    // value domains.
    let data_attributes: Vec<_> = component
        .attributes
        .iter()
        .filter(|attribute| !attribute.name.starts_with("--"))
        .collect();
    assert_eq!(
        data_attributes.len(),
        4,
        "the four emitted data-* attributes"
    );
    for attribute in &data_attributes {
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
    assert!(
        contents.contains("values: Some(&[\"none\", \"invalid\", \"valid\", \"pending\"])"),
        "validation-state domain"
    );
    assert!(
        contents.contains("values: Some(&[\"xs\", \"sm\", \"md\", \"lg\", \"xl\"])"),
        "size domain"
    );
    assert!(
        contents.contains("values: Some(&[\"compact\", \"default\", \"comfortable\"])"),
        "density domain"
    );
    assert!(
        contents.contains("values: Some(&[\"text\", \"multiline\", \"search\", \"slug\"])"),
        "type domain"
    );
    assert!(
        contents.contains("form: \"valued\"") && contents.contains("emission: \"always\""),
        "all four attributes are valued and always-emitted"
    );

    // Style props — the five TXT-16 padding hooks with the VisualState
    // field that feeds each (R2/R3).
    let style_props: Vec<_> = component
        .attributes
        .iter()
        .filter(|attribute| attribute.name.starts_with("--"))
        .collect();
    assert_eq!(style_props.len(), 5, "the five TXT-16 padding hooks");
    for attribute in &style_props {
        assert!(
            contents.contains(&format!("id: \"{}\"", attribute.id)),
            "artifact carries style prop '{}'",
            attribute.id
        );
        assert!(
            contents.contains(&format!("name: \"{}\"", attribute.name)),
            "artifact carries the hook name '{}'",
            attribute.name
        );
    }
    assert!(contents.contains("source: \"controlPaddingStart\""));
    assert!(contents.contains("source: \"controlPaddingEnd\""));
    assert!(contents.contains("source: \"multilineBottomPadding\""));
    assert!(contents.contains("source: \"clearInsetInlineEnd\""));
    assert!(contents.contains("source: \"trailingInsetInlineEnd\""));

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
            && contents.contains("kind: \"token\"")
            && contents.contains("kind: \"component-variable\""),
        "chain kinds are recorded, including the focus-fill component-variable chain"
    );
}

// ---------------------------------------------------------------------------
// The Generated Artifact Contract (spec 063; IR-07)
// ---------------------------------------------------------------------------

#[test]
fn artifact_header_names_the_source_definition_and_generator_version() {
    let contents = &render_text_input_artifact()[0].contents;
    let header_lines: Vec<&str> = contents
        .lines()
        .take_while(|line| line.starts_with("// "))
        .collect();
    assert!(
        header_lines
            .iter()
            .any(|line| *line == format!("// Source: {TEXT_INPUT_FIXTURE}")),
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
// One Rust definition change updates every TextInput artifact in one
// `ir:build` (spec 063; the card's acceptance; the R2 proof at the artifact
// level — both the attribute channel and the padding-hook channel)
// ---------------------------------------------------------------------------

#[test]
fn one_definition_change_moves_all_three_text_input_artifacts() {
    let mut changed = models::text_input::text_input_model();
    // One authored value change — the card's step-7 proof, encoded: rename
    // a state attribute (`data-validation-state` → `data-validation-level`,
    // the value the card changes live across all four previews) and a
    // TXT-16 padding hook (`--poodle-text-input-control-padding-start` →
    // `--poodle-text-input-padding-start-v2`, the style-prop channel).
    let component = changed
        .components
        .iter_mut()
        .find(|component| component.id.as_str() == "text-input")
        .expect("the one component");
    let validation_state = component
        .attributes
        .iter_mut()
        .find(|attribute| attribute.id.as_str() == "validation-state")
        .expect("the validation-state attribute");
    validation_state.name = "data-validation-level".to_owned();
    let padding_start = component
        .attributes
        .iter_mut()
        .find(|attribute| attribute.id.as_str() == "control-padding-start")
        .expect("the control-padding-start style prop");
    padding_start.name = "--poodle-text-input-padding-start-v2".to_owned();

    // One definition change renders every TextInput artifact differently.
    let rust = generate(
        &changed,
        TEXT_INPUT_FIXTURE,
        &targets::text_input_rust::TextInputRustTarget,
    )
    .expect("renders the changed definition");
    assert!(
        rust[0].contents.contains("name: \"data-validation-level\""),
        "the renamed attribute lands in the render artifact"
    );
    assert!(
        rust[0]
            .contents
            .contains("name: \"--poodle-text-input-padding-start-v2\""),
        "the renamed padding hook lands in the render artifact"
    );
    assert_ne!(
        rust[0].contents.as_bytes(),
        artifact_bytes(RENDER_ARTIFACT).as_slice(),
        "the render artifact would move in one rebuild"
    );

    let web = generate(
        &changed,
        TEXT_INPUT_FIXTURE,
        &targets::text_input::TextInputTarget,
    )
    .expect("renders the changed definition");
    assert!(
        web[0].contents.contains("name: \"data-validation-level\"")
            && web[0]
                .contents
                .contains("name: \"--poodle-text-input-padding-start-v2\""),
        "the renamed vocabulary lands in the web artifact"
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
// R2/R3 at the artifact level — the capability boundary is carried, and a
// gap is visible as a component-wide drop, never a per-runtime one
// ---------------------------------------------------------------------------

#[test]
fn a_capability_gap_is_visible_in_the_artifact_and_moves_the_render() {
    // R3's question, encoded: if a runtime lacked the text-editing
    // capability, the IR would express it as a definition-wide drop (the
    // only mechanism there is) — the artifact's capabilities list shrinks,
    // and the render artifact moves, which the shared render honours for
    // EVERY runtime. There is no way to express "Jetstream only".
    let mut changed = models::text_input::text_input_model();
    let component = changed
        .components
        .iter_mut()
        .find(|component| component.id.as_str() == "text-input")
        .expect("the one component");
    component
        .capabilities
        .retain(|requirement| requirement.capability != poodle_ir::Capability::TextEditing);

    let rendered = &generate(
        &changed,
        TEXT_INPUT_FIXTURE,
        &targets::text_input_rust::TextInputRustTarget,
    )
    .expect("renders the changed definition")[0];

    assert!(
        !rendered
            .contents
            .contains("capabilities: &[\n        \"text-editing\"")
            && rendered
                .contents
                .contains("capabilities: &[\n        \"focus\",\n        \"ime\""),
        "the dropped capability is visible in the artifact's list"
    );
    assert_ne!(
        rendered.contents.as_bytes(),
        artifact_bytes(RENDER_ARTIFACT).as_slice(),
        "a capability gap moves the render artifact"
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
        "poodle-render must not depend on poodle-ir or poodle-codegen (b003 R2; card 049 R1)"
    );
    assert!(
        manifest.contains("poodle-node") && manifest.contains("poodle-specs"),
        "the manifest still carries the existing render dependencies"
    );
}

// ---------------------------------------------------------------------------
// The drift gate covers the render artifact (ir:check on --target
// text-input-rust)
// ---------------------------------------------------------------------------

#[test]
fn text_input_rust_artifacts_fail_check_on_drift_and_check_never_writes() {
    let bin = env!("CARGO_BIN_EXE_poodle-codegen");
    let out = scratch("text-input-rust-artifacts");
    let root = out.join("generated/text-input");

    // A full write into the scratch, mirroring the Effigy selector.
    let status = Command::new(bin)
        .args([TEXT_INPUT_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "text-input-rust"])
        .current_dir(repo_root())
        .status()
        .expect("bin runs");
    assert!(status.success(), "text-input-rust write exits 0");
    let committed = fs::read_to_string(root.join("index.rs")).expect("artifact written");

    // Plant drift in the committed render artifact, then check: must fail,
    // and must not mutate the tree.
    fs::write(
        root.join("index.rs"),
        format!("{committed}\n// planted drift"),
    )
    .expect("plant drift");
    fs::write(root.join("orphan.json"), "{}\n").expect("plant orphan");
    let before = snapshot(&out);

    let status = Command::new(bin)
        .args([TEXT_INPUT_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "text-input-rust", "--check"])
        .current_dir(repo_root())
        .status()
        .expect("check runs");
    assert!(
        !status.success(),
        "drift in a render artifact fails ir:check"
    );
    assert_eq!(snapshot(&out), before, "check mode never mutates the tree");
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
