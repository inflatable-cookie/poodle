//! Tests for card 046 — the RangeSlider stateful proof, native half: the
//! `range-slider-rust` target (R2 — a sibling of `range-slider-ts`, which
//! is byte-frozen by b045's tests) emitting the self-contained Rust
//! artifact `poodle-render` consumes (R1/R1a — plain data, no `use` of any
//! Poodle crate, under `packages/render/src/generated/`), and the R3
//! vocabulary contract the render reads it through.
//!
//! The card's required parity test is
//! [`render_artifact_matches_the_target_render`]: the expected artifact is
//! the target's render of the authored definition, never a hand-listed
//! expectation, and the committed render artifact must equal it byte-exact.
//! The R2 proof — "one definition change updates every artifact in one
//! `ir:build`" — is
//! [`one_definition_change_moves_all_three_range_slider_artifacts`]
//! (rename a state attribute and a geometry hook, all three committed
//! artifacts move in one build) and is proven live across all four previews
//! in the card's step 6.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use poodle_codegen::{generate, models, targets, GeneratedFile, GENERATOR_VERSION};

/// The fixture path, exactly as the Effigy selector passes it — also the
/// source path the artifact header carries.
const RANGE_SLIDER_FIXTURE: &str = "packages/codegen/fixtures/range-slider-model.json";

/// The committed render artifact (card 046 R1a: the artifact lives in the
/// package that ships it — `poodle-render` is the consumer, so it lands
/// under `packages/render/src/generated/`, not a preview and not codegen).
/// It sits in its own nested root (`generated/range-slider/`) — the
/// disjoint-roots layout card 041 established for shared `generated/`
/// directories: the top level belongs to `button-rust`, whose orphan sweep
/// would delete a sibling's file.
const RENDER_ARTIFACT: &str = "packages/render/src/generated/range-slider/index.rs";

/// The web artifacts the same definition drives (card 045 locations).
const SVELTE_ARTIFACT: &str = "packages/svelte/components/src/generated/range-slider/index.ts";
const REACT_ARTIFACT: &str = "packages/react/components/src/generated/range-slider/index.ts";

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    crate_dir().join("../..")
}

/// A scratch directory for one test, under the target dir cargo cleans.
fn scratch(name: &str) -> PathBuf {
    let dir = crate_dir()
        .join("target")
        .join(format!("range-slider-rust-{name}"));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// The range-slider-rust target's render of the authored RangeSlider model,
/// with the fixture as the header's source path.
fn render_range_slider_artifact() -> Vec<GeneratedFile> {
    let model = models::range_slider::range_slider_model();
    generate(
        &model,
        RANGE_SLIDER_FIXTURE,
        &targets::range_slider_rust::RangeSliderRustTarget,
    )
    .expect("range-slider-rust renders the authored model")
}

fn artifact_bytes(path: &str) -> Vec<u8> {
    fs::read(repo_root().join(path)).expect("committed artifact reads")
}

// ---------------------------------------------------------------------------
// The shared native artifact (the card's parity test, definition-derived)
// ---------------------------------------------------------------------------

#[test]
fn render_artifact_matches_the_target_render() {
    let rendered = &render_range_slider_artifact()[0];
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
/// the nine parts (with the DOM class projection), the eight data-*
/// attributes (names, forms, emission policies, value domains), the seven
/// RNG-17 geometry hooks with their source fields, and the recipe hooks —
/// what the render reads instead of hard-coding.
#[test]
fn render_artifact_carries_the_rendered_vocabulary() {
    let contents = &render_range_slider_artifact()[0].contents;
    let model = models::range_slider::range_slider_model();
    let component = model
        .components
        .iter()
        .find(|component| component.id.as_str() == "range-slider")
        .expect("the one component");

    // The shared-type member lists the render discriminates on (R3).
    assert!(
        contents.contains("variants: &[\"standard\", \"embedded\"]"),
        "variant vocabulary"
    );
    assert!(
        contents.contains("polarities: &[\"unipolar\", \"bipolar\"]"),
        "polarity vocabulary"
    );
    assert!(
        contents.contains("sizes: &[\"xs\", \"sm\", \"md\", \"lg\", \"xl\"]"),
        "size vocabulary"
    );
    assert!(
        contents.contains("densities: &[\"compact\", \"default\", \"comfortable\"]"),
        "density vocabulary"
    );

    // Parts: every part id carries its DOM class projection (R §2).
    for part in &component.parts {
        assert!(
            contents.contains(&format!("id: \"{}\"", part.id)),
            "artifact carries part '{}'",
            part.id
        );
    }
    assert!(contents.contains("dom_class: \"poodle-range-slider\""));
    assert!(contents.contains("dom_class: \"poodle-range-slider__track\""));
    assert!(contents.contains("dom_class: \"poodle-range-slider__center\""));
    // The fill and control parts carry base + modifier classes.
    assert!(contents
        .contains("dom_class: \"poodle-range-slider__fill poodle-range-slider__fill--negative\""));
    assert!(contents
        .contains("dom_class: \"poodle-range-slider__fill poodle-range-slider__fill--positive\""));
    assert!(contents.contains(
        "dom_class: \"poodle-range-slider__control poodle-range-slider__control--lower\""
    ));
    assert!(contents.contains(
        "dom_class: \"poodle-range-slider__control poodle-range-slider__control--upper\""
    ));
    // The embedded-control class pairs are over the 100-column line, so the
    // emitter wraps them with a backslash continuation; fold those back to
    // the single-class-string form before asserting.
    let folded = fold_continuations(contents);
    assert!(folded.contains(
        "dom_class: \"poodle-range-slider__embedded-control \
         poodle-range-slider__embedded-control--lower\""
    ));
    assert!(folded.contains(
        "dom_class: \"poodle-range-slider__embedded-control \
         poodle-range-slider__embedded-control--upper\""
    ));

    // Attributes: the eight data-* names, forms, emission policies, and
    // value domains.
    let data_attributes: Vec<_> = component
        .attributes
        .iter()
        .filter(|attribute| !attribute.name.starts_with("--"))
        .collect();
    assert_eq!(data_attributes.len(), 8, "the eight data-* attributes");
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
        contents.contains("values: Some(&[\"horizontal\", \"vertical\"])"),
        "orientation domain"
    );
    assert!(
        contents.contains("values: Some(&[\"standard\", \"embedded\"])"),
        "variant domain"
    );
    assert!(
        contents.contains("values: Some(&[\"unipolar\", \"bipolar\"])"),
        "polarity domain"
    );
    assert!(
        contents.contains("values: Some(&[\"true\", \"false\"])"),
        "boolean domains (disabled, fill-split)"
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
        contents.contains("form: \"valued\"") && contents.contains("emission: \"always\""),
        "all eight attributes are valued and always-emitted"
    );

    // Style props — the seven RNG-17 geometry hooks with the VisualState
    // field that feeds each (R2/R3).
    let style_props: Vec<_> = component
        .attributes
        .iter()
        .filter(|attribute| attribute.name.starts_with("--"))
        .collect();
    assert_eq!(style_props.len(), 7, "the seven geometry hooks");
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
    assert!(contents.contains("source: \"lowerNorm\""));
    assert!(contents.contains("source: \"upperNorm\""));
    assert!(contents.contains("source: \"centerNorm\""));
    assert!(contents.contains("source: \"negativeFillStartNorm\""));
    assert!(contents.contains("source: \"negativeFillSpanNorm\""));
    assert!(contents.contains("source: \"positiveFillStartNorm\""));
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
        "chain kinds are recorded (hook → terminal token; the RNG-21 shape)"
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
// One Rust definition change updates every RangeSlider artifact in one
// `ir:build` (spec 063; the card's acceptance; the R2 proof at the artifact
// level — both the attribute channel and the geometry-hook channel)
// ---------------------------------------------------------------------------

#[test]
fn one_definition_change_moves_all_three_range_slider_artifacts() {
    let mut changed = models::range_slider::range_slider_model();
    // One authored value change — the card's R2 proof, encoded: rename a
    // state attribute (`data-variant` → `data-variant-level`, the value
    // the card's step 6 changes live across all four previews) and a
    // geometry hook (`--poodle-range-positive-span` →
    // `--poodle-range-positive-width`, the RNG-17 fill channel).
    let component = changed
        .components
        .iter_mut()
        .find(|component| component.id.as_str() == "range-slider")
        .expect("the one component");
    let variant = component
        .attributes
        .iter_mut()
        .find(|attribute| attribute.id.as_str() == "variant")
        .expect("the variant attribute");
    variant.name = "data-variant-level".to_owned();
    let positive_span = component
        .attributes
        .iter_mut()
        .find(|attribute| attribute.id.as_str() == "range-positive-span")
        .expect("the positive-span geometry hook");
    positive_span.name = "--poodle-range-positive-width".to_owned();

    // One definition change renders every RangeSlider artifact differently.
    let rust = generate(
        &changed,
        RANGE_SLIDER_FIXTURE,
        &targets::range_slider_rust::RangeSliderRustTarget,
    )
    .expect("renders the changed definition");
    assert!(
        rust[0].contents.contains("name: \"data-variant-level\""),
        "the renamed attribute lands in the render artifact"
    );
    assert!(
        rust[0]
            .contents
            .contains("name: \"--poodle-range-positive-width\""),
        "the renamed geometry hook lands in the render artifact"
    );
    assert_ne!(
        rust[0].contents.as_bytes(),
        artifact_bytes(RENDER_ARTIFACT).as_slice(),
        "the render artifact would move in one rebuild"
    );

    let web = generate(
        &changed,
        RANGE_SLIDER_FIXTURE,
        &targets::range_slider::RangeSliderTarget,
    )
    .expect("renders the changed definition");
    assert!(
        web[0].contents.contains("name: \"data-variant-level\"")
            && web[0]
                .contents
                .contains("name: \"--poodle-range-positive-width\""),
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
// R1 asserted, not just avoided: the render manifest gains no dependency
// ---------------------------------------------------------------------------

#[test]
fn render_manifest_carries_no_poodle_ir_or_codegen_dependency() {
    let manifest = fs::read_to_string(repo_root().join("packages/render/Cargo.toml"))
        .expect("render manifest reads");
    assert!(
        !manifest.contains("poodle-ir") && !manifest.contains("poodle-codegen"),
        "poodle-render must not depend on poodle-ir or poodle-codegen (b003 R2; card 046 R1)"
    );
    assert!(
        manifest.contains("poodle-node") && manifest.contains("poodle-specs"),
        "the manifest still carries the existing render dependencies"
    );
}

// ---------------------------------------------------------------------------
// The drift gate covers the render artifact (ir:check on --target
// range-slider-rust)
// ---------------------------------------------------------------------------

#[test]
fn range_slider_rust_artifacts_fail_check_on_drift_and_check_never_writes() {
    let bin = env!("CARGO_BIN_EXE_poodle-codegen");
    let out = scratch("render-artifacts");
    let root = out.join("generated/range-slider");

    // A full write into the scratch, mirroring the Effigy selector.
    let status = Command::new(bin)
        .args([RANGE_SLIDER_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "range-slider-rust"])
        .current_dir(repo_root())
        .status()
        .expect("bin runs");
    assert!(status.success(), "range-slider-rust write exits 0");
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
        .args([RANGE_SLIDER_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "range-slider-rust", "--check"])
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

/// Folds the emitter's backslash-continuation literals (over-long values
/// wrapped onto a second line) back to the single string they denote, so
/// vocabulary assertions can name the joined value.
fn fold_continuations(contents: &str) -> String {
    let mut out = String::with_capacity(contents.len());
    let mut chars = contents.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' && chars.peek() == Some(&'\n') {
            // A line-continuation backslash is not part of the string; the
            // newline and its leading whitespace are skipped (the space
            // before the backslash is already in the literal).
            chars.next();
            while chars.peek().is_some_and(|c| c.is_whitespace()) {
                chars.next();
            }
            continue;
        }
        out.push(ch);
    }
    out
}

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
