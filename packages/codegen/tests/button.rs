//! Tests for card 041 — the Button vertical slice, web half: the
//! Rust-authored Button definition (R1), the serialized fixture round trip,
//! and the `button-ts` artifact both web components consume.
//!
//! The card's required parity test is
//! [`both_web_components_carry_the_same_button_derived_artifact`]: the
//! expected artifact is the target's render of the authored definition,
//! never a hand-listed expectation, and each committed web artifact must
//! equal it byte-exact. A component that drifts (different attribute names,
//! part classes, or recipe hooks) fails the comparison.
//!
//! The R2 proof — "a definition change moves the DOM in both web previews"
//! — is served at the artifact level by
//! [`one_definition_change_moves_both_web_artifacts`] (rename a state
//! attribute, both artifacts move in one build) and proven live in the
//! card's step 7 with both previews running.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use poodle_codegen::{generate, models, targets, GeneratedFile, GENERATOR_VERSION};

/// The fixture path, exactly as the Effigy selector passes it — also the
/// source path the artifact headers carry.
const BUTTON_FIXTURE: &str = "packages/codegen/fixtures/button-model.json";

const SVELTE_ARTIFACT: &str = "packages/svelte/preview/src/generated/button/index.ts";
const REACT_ARTIFACT: &str = "packages/react/preview/src/generated/button/index.ts";

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    crate_dir().join("../..")
}

fn fixture_path() -> PathBuf {
    repo_root().join(BUTTON_FIXTURE)
}

/// A scratch directory for one test, under the target dir cargo cleans.
fn scratch(name: &str) -> PathBuf {
    let path = crate_dir().join("target").join("button-tests").join(name);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("scratch dir");
    path
}

/// The button-ts target's render of the authored Button model, with the
/// fixture as the header's source path.
fn render_button_artifact() -> Vec<GeneratedFile> {
    let model = models::button::button_model();
    let findings = model.validate();
    assert!(
        findings.is_empty(),
        "the authored Button model validates clean: {findings:?}"
    );
    generate(&model, BUTTON_FIXTURE, &targets::button::ButtonTarget)
        .expect("button-ts target renders the authored model")
}

fn artifact_bytes(path: &str) -> Vec<u8> {
    fs::read(repo_root().join(path))
        .unwrap_or_else(|error| panic!("committed artifact {path} is readable: {error}"))
}

// ---------------------------------------------------------------------------
// The authored definition (R1: authored in Rust; R2: values only, no schema)
// ---------------------------------------------------------------------------

#[test]
fn button_model_validates_and_round_trips_as_json() {
    let model = models::button::button_model();
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
fn button_definition_authors_the_full_contract_surface() {
    let model = models::button::button_model();
    let component = model
        .components
        .iter()
        .find(|component| component.id.as_str() == "button")
        .expect("the one component");

    // 30 data/snippet props + 4 events = the 34-web-prop surface (R3). The
    // IR records the callbacks as events (CROSS-05), not props.
    assert_eq!(
        component.props.len(),
        30,
        "27 data props + 3 snippet slots (children/leading/trailing)"
    );
    assert_eq!(
        component.events.len(),
        4,
        "onClick/onFocus/onBlur/onPressedChange"
    );
    assert_eq!(
        component.attributes.len(),
        11,
        "the eleven data-* attributes (BTN-18)"
    );
    assert_eq!(
        component.parts.len(),
        6,
        "root + spinner + leading icon + label + trailing icon + chevron (B §2)"
    );
    assert_eq!(
        component.recipe_hooks.len(),
        76,
        "the distinct recipe hooks of button.css (BTN-22)"
    );
    assert_eq!(
        model.shared_types.len(),
        9,
        "the shared enumerated types the props reference"
    );

    // Key prop defaults, byte-stable against the contract (R3).
    let prop = |name: &str| {
        component
            .props
            .iter()
            .find(|p| p.name == name)
            .unwrap_or_else(|| panic!("prop {name} authored"))
    };
    assert_eq!(
        prop("variant").default,
        Some(poodle_ir::Value::member("secondary"))
    );
    assert_eq!(
        prop("tone").default,
        Some(poodle_ir::Value::member("default"))
    );
    assert_eq!(
        prop("fit").default,
        Some(poodle_ir::Value::member("default"))
    );
    assert_eq!(
        prop("disabled").default,
        Some(poodle_ir::Value::boolean(false))
    );
    assert_eq!(
        prop("loading").default,
        Some(poodle_ir::Value::boolean(false))
    );
    assert_eq!(prop("pressed").default, Some(poodle_ir::Value::Null));
    assert!(
        prop("type").web_only,
        "the form family is web-only (CROSS-03)"
    );
    assert!(prop("form").web_only);
    assert!(prop("formmethod").web_only);
    assert!(!prop("tone").web_only, "tone is portable");

    // The size ladder carries the contract's fixed rem metrics (BTN-23).
    let size_axis = component.axes.size.as_ref().expect("size axis authored");
    assert_eq!(size_axis.ladder.len(), 5, "xs through xl");
    let md = size_axis
        .ladder
        .iter()
        .find(|step| step.size == poodle_ir::ControlSize::Md)
        .expect("md rung");
    assert_eq!(
        md.metrics.get("height"),
        Some(&poodle_ir::MetricValue::Rem(2.25))
    );
    assert_eq!(
        md.metrics.get("min-width"),
        Some(&poodle_ir::MetricValue::Rem(5.0))
    );
    assert_eq!(
        md.metrics.get("font-size"),
        Some(&poodle_ir::MetricValue::Rem(0.8125))
    );
}

// ---------------------------------------------------------------------------
// The shared web artifact (the card's parity test, definition-derived)
// ---------------------------------------------------------------------------

#[test]
fn both_web_components_carry_the_same_button_derived_artifact() {
    let files = render_button_artifact();
    assert_eq!(files.len(), 1, "one component renders one artifact");
    let rendered = &files[0];
    assert_eq!(
        rendered.path, "index.ts",
        "the artifact is the nested root's index"
    );
    assert!(
        !rendered.contents.is_empty(),
        "the artifact is not an empty stub"
    );

    // The expected artifact is the definition's own render — derived, never
    // hand-listed. Both web components must carry exactly it.
    assert_eq!(
        artifact_bytes(SVELTE_ARTIFACT),
        rendered.contents.as_bytes(),
        "the Svelte component's committed artifact equals the definition's render"
    );
    assert_eq!(
        artifact_bytes(REACT_ARTIFACT),
        rendered.contents.as_bytes(),
        "the React component's committed artifact equals the definition's render"
    );
}

/// The R2 vocabulary: the artifact carries the parts (with DOM classes),
/// the eleven state attributes (names, forms, emission policies, value
/// domains), and the recipe hooks — what the components read instead of
/// hard-coding.
#[test]
fn artifact_renders_parts_attributes_and_recipe_hooks() {
    let files = render_button_artifact();
    let contents = &files[0].contents;
    let model = models::button::button_model();
    let component = model
        .components
        .iter()
        .find(|component| component.id.as_str() == "button")
        .expect("the one component");

    // Parts: every part id carries its DOM class projection (B §2).
    for part in &component.parts {
        assert!(
            contents.contains(&format!("id: \"{}\"", part.id)),
            "artifact carries part '{}'",
            part.id
        );
    }
    assert!(contents.contains("className: \"poodle-button\""));
    assert!(contents.contains("className: \"poodle-button__spinner\""));
    assert!(contents.contains("className: \"poodle-button__label\""));
    assert!(contents.contains("className: \"poodle-button__chevron\""));
    // Leading and trailing icons collapse onto the shared icon span class.
    assert_eq!(
        contents
            .matches("className: \"poodle-button__icon\"")
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
    // The value domains the components' DOM can carry (R2).
    assert!(
        contents.contains("values: [\"primary\", \"secondary\", \"ghost\"]"),
        "variant domain"
    );
    assert!(
        contents.contains("values: [\"danger\", \"success\", \"warning\"]"),
        "tone domain omits the default"
    );
    assert!(
        contents.contains("values: [\"xs\", \"sm\", \"md\", \"lg\", \"xl\"]"),
        "size domain"
    );
    assert!(
        contents.contains("values: [\"content\"]"),
        "fit domain omits the default"
    );
    assert!(
        contents.contains("values: [\"true\", \"false\"]"),
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
            && contents.contains("kind: \"component-variable\""),
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
// One Rust definition change updates both web artifacts in one `ir:build`
// (spec 063; the card's acceptance; the R2 proof at the artifact level)
// ---------------------------------------------------------------------------

#[test]
fn one_definition_change_moves_both_web_artifacts() {
    let mut changed = models::button::button_model();
    // One authored value change — the card's R2 proof, encoded: rename a
    // state attribute (`data-tone` → `data-tone-level`).
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

    let files = generate(&changed, BUTTON_FIXTURE, &targets::button::ButtonTarget)
        .expect("renders the changed definition");
    let contents = &files[0].contents;

    assert!(
        contents.contains("name: \"data-tone-level\""),
        "the renamed attribute lands in the artifact"
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
}

// ---------------------------------------------------------------------------
// The drift gate covers the web artifacts (ir:check on --target button-ts)
// ---------------------------------------------------------------------------

#[test]
fn button_artifacts_fail_check_on_drift_and_check_never_writes() {
    let bin = env!("CARGO_BIN_EXE_poodle-codegen");
    let out = scratch("web-artifacts");
    // button-ts owns its nested root inside the shared generated/ dir.
    let root = out.join("generated").join("button");

    // A full write into the scratch, mirroring the Effigy selector.
    let status = Command::new(bin)
        .args([BUTTON_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "button-ts"])
        .current_dir(repo_root())
        .status()
        .expect("bin runs");
    assert!(status.success(), "button-ts write exits 0");
    let committed = fs::read_to_string(root.join("index.ts")).expect("artifact written");

    // Plant drift in the committed web artifact, then check: must fail, and
    // must not mutate the tree.
    fs::write(
        root.join("index.ts"),
        format!("{committed}\n// planted drift"),
    )
    .expect("plant drift");
    fs::write(root.join("orphan.json"), "{}\n").expect("plant orphan");
    let before = snapshot(&root);

    let status = Command::new(bin)
        .args([BUTTON_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "button-ts", "--check"])
        .current_dir(repo_root())
        .status()
        .expect("check runs");
    assert!(!status.success(), "drift in a web artifact fails ir:check");
    assert_eq!(snapshot(&root), before, "check mode never mutates the tree");

    // The fixture itself is gated the same way: an authored-model change
    // with a stale committed fixture fails --author-button --check.
    let status = Command::new(bin)
        .args(["--author-button"])
        .arg(scratch("fixture-copy").join("button-model.json"))
        .current_dir(repo_root())
        .status()
        .expect("author runs");
    assert!(status.success(), "authoring writes the fixture");
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
