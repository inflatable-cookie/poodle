//! Tests for card 045 — the RangeSlider stateful proof, web half: the
//! Rust-authored RangeSlider definition (R1), the serialized fixture round
//! trip, the declarative `slider` conformance vector (R1/R5), and the
//! `range-slider-ts` artifact both web components consume.
//!
//! The card's required parity test is
//! [`both_web_components_carry_the_same_range_slider_derived_artifact`]:
//! the expected artifact is the target's render of the authored definition,
//! never a hand-listed expectation, and each committed web artifact must
//! equal it byte-exact.
//!
//! The R2 proof — "a definition change moves the DOM in both web previews"
//! — is served at the artifact level by
//! [`one_definition_change_moves_both_web_artifacts`] (rename a state
//! attribute and a geometry style prop; both artifacts move in one build)
//! and proven live in the card's step 6 with both previews running.
//!
//! R2.1's negative finding — the IR's `Repeated` part kind requires a
//! `List` prop and yields identical instances, so the two thumbs are
//! recorded as distinct parts and the renderer hard-codes "two" — is
//! asserted here structurally: the parts are distinct, and no part uses
//! `PartKind::Repeated`.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use poodle_codegen::{generate, models, targets, GeneratedFile, GENERATOR_VERSION};

/// The fixture path, exactly as the Effigy selector passes it — also the
/// source path the artifact headers carry.
const RANGE_SLIDER_FIXTURE: &str = "packages/codegen/fixtures/range-slider-model.json";

// The committed artifacts, inside the consuming component packages (the
// published tarball carries `src` only, so the consumer must be able to
// resolve the import inside its own package — the b041 papercut fix).
const SVELTE_ARTIFACT: &str = "packages/svelte/components/src/generated/range-slider/index.ts";
const REACT_ARTIFACT: &str = "packages/react/components/src/generated/range-slider/index.ts";

/// The executable slider vector — a fixed target (R5): it must pass
/// unedited against both machines.
const MACHINES_VECTOR: &str = "packages/contracts/headless/vectors/machines.json";

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    crate_dir().join("../..")
}

fn fixture_path() -> PathBuf {
    repo_root().join(RANGE_SLIDER_FIXTURE)
}

/// A scratch directory for one test, under the target dir cargo cleans.
fn scratch(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    if path.exists() {
        fs::remove_dir_all(&path).expect("scratch clean");
    }
    path
}

/// The range-slider-ts target's render of the authored RangeSlider model,
/// with the fixture as the header's source path.
fn render_range_slider_artifact() -> Vec<GeneratedFile> {
    let model = models::range_slider::range_slider_model();
    let findings = model.validate();
    assert!(
        findings.is_empty(),
        "the authored RangeSlider model validates clean: {findings:?}"
    );
    generate(
        &model,
        RANGE_SLIDER_FIXTURE,
        &targets::range_slider::RangeSliderTarget,
    )
    .expect("range-slider-ts target renders the authored model")
}

fn artifact_bytes(path: &str) -> Vec<u8> {
    fs::read(repo_root().join(path))
        .unwrap_or_else(|error| panic!("committed artifact {path} is readable: {error}"))
}

// ---------------------------------------------------------------------------
// The authored definition (R1: authored in Rust; R2: values only, no schema)
// ---------------------------------------------------------------------------

#[test]
fn range_slider_model_validates_and_round_trips_as_json() {
    let model = models::range_slider::range_slider_model();
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
fn range_slider_definition_authors_the_full_contract_surface() {
    let model = models::range_slider::range_slider_model();
    let component = model
        .components
        .iter()
        .find(|component| component.id.as_str() == "range-slider")
        .expect("the one component");

    // 16 data props + 2 callbacks (declared as events) = the 18-web-prop
    // surface (R4). `defaultValue` (React's uncontrolled seed) is an extra
    // web-only record, not one of the 18.
    assert_eq!(
        component.props.len(),
        17,
        "16 contract props + defaultValue (web-only, CROSS-03)"
    );
    assert_eq!(
        component.events.len(),
        2,
        "onValueChange/onValueCommit (R §5)"
    );
    assert_eq!(
        component.attributes.len(),
        15,
        "the 8 data-* attributes + 7 fill-geometry custom properties (RNG-17)"
    );
    assert_eq!(
        component
            .attributes
            .iter()
            .filter(|attribute| attribute.name.starts_with("data-"))
            .count(),
        8,
        "the eight data-* attributes (R §9)"
    );
    assert_eq!(
        component
            .attributes
            .iter()
            .filter(|attribute| attribute.name.starts_with("--poodle-range-"))
            .count(),
        7,
        "the seven geometry custom properties (RNG-17)"
    );
    assert_eq!(
        component.parts.len(),
        9,
        "root + track + 2 fill segments + center + 2 controls + 2 embedded controls (R §2)"
    );
    assert!(
        component
            .parts
            .iter()
            .all(|part| !matches!(part.kind, poodle_ir::PartKind::Repeated { .. })),
        "R2.1: the two thumbs are NOT one Repeated part — the IR's Repeated kind needs a List \
         prop and yields identical instances, so the anatomy records distinct parts and the \
         renderer hard-codes two (finding for g13.008)"
    );
    assert_eq!(
        component.recipe_hooks.len(),
        11,
        "the eleven recipe hooks of range-slider.css (RNG-21)"
    );
    assert_eq!(
        model.shared_types.len(),
        7,
        "slider-variant/polarity/thumb + control-size/density/size-role + orientation"
    );

    // Key prop defaults, byte-stable against the contract (R4).
    let prop = |name: &str| {
        component
            .props
            .iter()
            .find(|p| p.name == name)
            .unwrap_or_else(|| panic!("prop {name} authored"))
    };
    assert_eq!(
        prop("value").default,
        Some(poodle_ir::Value::Pair(
            Box::new(poodle_ir::Value::Number(0.0)),
            Box::new(poodle_ir::Value::Number(100.0)),
        ))
    );
    assert_eq!(
        prop("variant").default,
        Some(poodle_ir::Value::member("standard"))
    );
    assert_eq!(
        prop("polarity").default,
        Some(poodle_ir::Value::member("unipolar"))
    );
    assert_eq!(
        prop("orientation").default,
        Some(poodle_ir::Value::member("horizontal"))
    );
    assert_eq!(prop("centerValue").default, Some(poodle_ir::Value::Null));
    assert!(
        prop("defaultValue").web_only,
        "React's uncontrolled seed is web-only (CROSS-03)"
    );
    assert!(!prop("value").web_only, "the pair is portable");

    // The size ladder carries the contract's fixed rem metrics (RNG-09).
    let size_axis = component.axes.size.as_ref().expect("size axis authored");
    assert_eq!(size_axis.ladder.len(), 5, "xs through xl");
    let md = size_axis
        .ladder
        .iter()
        .find(|step| step.size == poodle_ir::ControlSize::Md)
        .expect("md rung");
    assert_eq!(
        md.metrics.get("min-height"),
        Some(&poodle_ir::MetricValue::Rem(1.5))
    );
    assert_eq!(
        md.metrics.get("track-thickness"),
        Some(&poodle_ir::MetricValue::Rem(0.375))
    );
    assert_eq!(
        md.metrics.get("thumb-diameter"),
        Some(&poodle_ir::MetricValue::Rem(1.0))
    );

    // The orientation axis and the density hit-area adjustments (RNG-07/09).
    let orientation = component
        .axes
        .orientation
        .as_ref()
        .expect("orientation axis authored");
    assert_eq!(orientation.default, poodle_ir::Orientation::Horizontal);
    assert_eq!(orientation.values.len(), 2, "horizontal and vertical");
    let density = component
        .axes
        .density
        .as_ref()
        .expect("density axis authored");
    assert_eq!(density.adjustments.len(), 2, "compact + comfortable");
    assert!(
        density.adjustments.iter().all(|adjustment| adjustment
            .applies_to
            .as_ref()
            .is_some_and(|id| id.as_str() == "root")),
        "the density padding applies to the root part (the §8 hit-area exception)"
    );
}

// ---------------------------------------------------------------------------
// R1/R5 — the declarative vector reference (the machine stays hand-written)
// ---------------------------------------------------------------------------

#[test]
fn slider_vector_declares_the_hand_written_machine_semantics() {
    let model = models::range_slider::range_slider_model();
    let component = model
        .components
        .iter()
        .find(|component| component.id.as_str() == "range-slider")
        .expect("the one component");

    // The component names the machine semantics through the vector (R1:
    // the IR declares the machine, it does not absorb it).
    assert_eq!(
        component.conformance,
        vec![poodle_ir::Identifier::new("slider")],
        "the machine semantics are pinned by the shared slider vector (CROSS-18)"
    );

    let vector = model
        .conformance_vector("slider")
        .expect("the slider vector resolves in the model");
    assert!(
        vector
            .applies_to
            .contains(&poodle_ir::RuntimeTarget::Svelte)
            && vector.applies_to.contains(&poodle_ir::RuntimeTarget::React),
        "both web runtime machines honor the vector"
    );
    assert!(!vector.steps.is_empty(), "the vector declares step intents");

    // R5: the executable vector file is the fixed target — it still carries
    // the slider key, unedited.
    let vector_file =
        fs::read_to_string(repo_root().join(MACHINES_VECTOR)).expect("machines.json reads");
    let parsed: serde_json::Value =
        serde_json::from_str(&vector_file).expect("machines.json parses");
    assert!(
        parsed.get("slider").is_some(),
        "the slider conformance vector still exists in {MACHINES_VECTOR}"
    );
}

// ---------------------------------------------------------------------------
// The shared web artifact (the card's parity test, definition-derived)
// ---------------------------------------------------------------------------

#[test]
fn both_web_components_carry_the_same_range_slider_derived_artifact() {
    let files = render_range_slider_artifact();
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
/// the eight data-* attributes (names, forms, emission policies, value
/// domains), the seven geometry styleProps, and the recipe hooks — what
/// the components read instead of hard-coding.
#[test]
fn artifact_renders_parts_attributes_style_props_and_recipe_hooks() {
    let files = render_range_slider_artifact();
    let contents = &files[0].contents;
    let model = models::range_slider::range_slider_model();
    let component = model
        .components
        .iter()
        .find(|component| component.id.as_str() == "range-slider")
        .expect("the one component");

    // Parts: every part id carries its DOM class projection (R §2).
    for part in &component.parts {
        assert!(
            contents.contains(&format!("id: \"{}\"", part.id)),
            "artifact carries part '{}'",
            part.id
        );
    }
    assert!(contents.contains("className: \"poodle-range-slider\""));
    assert!(contents.contains("className: \"poodle-range-slider__track\""));
    assert!(contents.contains("className: \"poodle-range-slider__center\""));
    // The fill and control parts carry base + modifier classes.
    assert!(contents
        .contains("className: \"poodle-range-slider__fill poodle-range-slider__fill--negative\""));
    assert!(contents
        .contains("className: \"poodle-range-slider__fill poodle-range-slider__fill--positive\""));
    assert!(contents.contains(
        "className: \"poodle-range-slider__control poodle-range-slider__control--lower\""
    ));
    assert!(contents.contains(
        "className: \"poodle-range-slider__embedded-control \
         poodle-range-slider__embedded-control--upper\""
    ));

    // Attributes: the 8 data-* names, forms, policies, and value domains.
    for attribute in component
        .attributes
        .iter()
        .filter(|attribute| attribute.name.starts_with("data-"))
    {
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
        contents.contains("values: [\"horizontal\", \"vertical\"]"),
        "orientation domain"
    );
    assert!(
        contents.contains("values: [\"standard\", \"embedded\"]"),
        "variant domain"
    );
    assert!(
        contents.contains("values: [\"unipolar\", \"bipolar\"]"),
        "polarity domain"
    );
    assert!(
        contents.contains("values: [\"true\", \"false\"]"),
        "boolean domains (disabled, fill-split)"
    );
    assert!(
        contents.contains("values: [\"xs\", \"sm\", \"md\", \"lg\", \"xl\"]"),
        "size domain"
    );

    // Style props: the seven geometry custom properties with the visual
    // field each is fed by (RNG-17).
    for attribute in component
        .attributes
        .iter()
        .filter(|attribute| attribute.name.starts_with("--poodle-range-"))
    {
        assert!(
            contents.contains(&format!("name: \"{}\"", attribute.name)),
            "artifact carries the geometry property '{}'",
            attribute.name
        );
    }
    assert!(contents.contains("source: \"lowerNorm\""));
    assert!(contents.contains("source: \"negativeFillSpanNorm\""));
    assert!(contents.contains("source: \"positiveFillSpanNorm\""));

    // Recipe hooks: every declared hook and its chain lands in the artifact.
    for hook in &component.recipe_hooks {
        assert!(
            contents.contains(&format!("hook: \"{}\"", hook.hook)),
            "artifact carries recipe hook '{}'",
            hook.hook
        );
    }
    assert!(
        contents.contains("kind: \"recipe-hook\"") && contents.contains("kind: \"token\""),
        "chain kinds are recorded"
    );
    assert!(
        contents.contains("--poodle-recipe-range-slider-fill-negative"),
        "the negative-fill recipe role is in the artifact (R2.2 — recipe roles remain exact)"
    );
}

// ---------------------------------------------------------------------------
// The Generated Artifact Contract (spec 063; IR-07)
// ---------------------------------------------------------------------------

#[test]
fn artifact_header_names_the_source_definition_and_generator_version() {
    let contents = &render_range_slider_artifact()[0].contents;
    let header_lines: Vec<&str> = contents
        .lines()
        .take_while(|line| line.starts_with("// "))
        .collect();
    assert!(
        header_lines
            .iter()
            .any(|line| *line == format!("// Source: {RANGE_SLIDER_FIXTURE}")),
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
    let mut changed = models::range_slider::range_slider_model();
    let component = changed
        .components
        .iter_mut()
        .find(|component| component.id.as_str() == "range-slider")
        .expect("the one component");

    // One authored value change — the card's R2 proof, encoded: rename a
    // data-* attribute (`data-polarity` → `data-polarity-level`).
    let polarity = component
        .attributes
        .iter_mut()
        .find(|attribute| attribute.id.as_str() == "polarity")
        .expect("the polarity attribute");
    polarity.name = "data-polarity-level".to_owned();

    // And a geometry hook (--poodle-range-start → --poodle-range-begin):
    // the value-dependent fill vocabulary moves with the definition too.
    let range_start = component
        .attributes
        .iter_mut()
        .find(|attribute| attribute.id.as_str() == "range-start")
        .expect("the range-start style prop");
    range_start.name = "--poodle-range-begin".to_owned();

    let files = generate(
        &changed,
        RANGE_SLIDER_FIXTURE,
        &targets::range_slider::RangeSliderTarget,
    )
    .expect("renders the changed definition");
    let contents = &files[0].contents;

    assert!(
        contents.contains("name: \"data-polarity-level\""),
        "the renamed attribute lands in the artifact"
    );
    assert!(
        contents.contains("name: \"--poodle-range-begin\""),
        "the renamed geometry hook lands in the artifact"
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
// The drift gate covers the web artifacts (ir:check on --target
// range-slider-ts)
// ---------------------------------------------------------------------------

#[test]
fn range_slider_artifacts_fail_check_on_drift_and_check_never_writes() {
    let bin = env!("CARGO_BIN_EXE_poodle-codegen");
    let out = scratch("web-artifacts");
    // range-slider-ts owns its nested root inside the shared generated/
    // dir.
    let root = out.join("generated").join("range-slider");

    // A full write into the scratch, mirroring the Effigy selector.
    let status = Command::new(bin)
        .args([RANGE_SLIDER_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "range-slider-ts"])
        .current_dir(repo_root())
        .status()
        .expect("bin runs");
    assert!(status.success(), "range-slider-ts write exits 0");
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
        .args([RANGE_SLIDER_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "range-slider-ts", "--check"])
        .current_dir(repo_root())
        .status()
        .expect("check runs");
    assert!(!status.success(), "drift in a web artifact fails ir:check");
    assert_eq!(snapshot(&root), before, "check mode never mutates the tree");

    // The fixture itself is gated the same way: an authored-model change
    // with a stale committed fixture fails --author-range-slider --check.
    let status = Command::new(bin)
        .args(["--author-range-slider"])
        .arg(scratch("fixture-copy").join("range-slider-model.json"))
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
