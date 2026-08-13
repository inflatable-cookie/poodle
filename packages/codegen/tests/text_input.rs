//! Tests for card 048 — the TextInput environment-boundary proof, web
//! half: the Rust-authored TextInput definition (R1), the serialized
//! fixture round trip, the typed capability boundary (R2), the declarative
//! `text-input` conformance vector (R1/R5), and the `text-input-ts`
//! artifact both web components consume.
//!
//! The card's required parity test is
//! [`both_web_components_carry_the_same_text_input_derived_artifact`]:
//! the expected artifact is the target's render of the authored definition,
//! never a hand-listed expectation, and each committed web artifact must
//! equal it byte-exact.
//!
//! The R2 proof — "a definition change moves the DOM in both web previews"
//! — is served at the artifact level by
//! [`one_definition_change_moves_both_web_artifacts`] (rename a state
//! attribute and a TXT-16 style prop; both artifacts move in one build)
//! and proven live in the card's step 6 with both previews running.
//!
//! The R2 boundary's shape — selection rides on `TextEditing` +
//! `Measurement`, there is no per-runtime ownership field, and the text
//! machine is not vector-pinned — is asserted here structurally
//! ([`text_input_vector_declares_the_hand_written_machine_semantics`] and
//! the definition-surface test) and answered in the batch log (R3).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use poodle_codegen::{generate, models, targets, GeneratedFile, GENERATOR_VERSION};

/// The fixture path, exactly as the Effigy selector passes it — also the
/// source path the artifact headers carry.
const TEXT_INPUT_FIXTURE: &str = "packages/codegen/fixtures/text-input-model.json";

// The committed artifacts, inside the consuming component packages (the
// published tarball carries `src` only, so the consumer must be able to
// resolve the import inside its own package — the b041 papercut fix).
const SVELTE_ARTIFACT: &str = "packages/svelte/components/src/generated/text-input/index.ts";
const REACT_ARTIFACT: &str = "packages/react/components/src/generated/text-input/index.ts";

/// The executable conformance-vector file — a fixed target (R5): the text
/// machine is **not** pinned there (no `text` key, GAP-01).
const MACHINES_VECTOR: &str = "packages/contracts/headless/vectors/machines.json";

/// The web component manifests — the card's required test: no
/// poodle-ir/poodle-codegen dependency may appear (R1; the artifact is
/// consumed from inside the same package).
const SVELTE_MANIFEST: &str = "packages/svelte/components/package.json";
const REACT_MANIFEST: &str = "packages/react/components/package.json";

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    crate_dir().join("../..")
}

fn fixture_path() -> PathBuf {
    repo_root().join(TEXT_INPUT_FIXTURE)
}

/// A scratch directory for one test, under the target dir cargo cleans.
fn scratch(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    if path.exists() {
        fs::remove_dir_all(&path).expect("scratch clean");
    }
    path
}

/// The text-input-ts target's render of the authored TextInput model,
/// with the fixture as the header's source path.
fn render_text_input_artifact() -> Vec<GeneratedFile> {
    let model = models::text_input::text_input_model();
    let findings = model.validate();
    assert!(
        findings.is_empty(),
        "the authored TextInput model validates clean: {findings:?}"
    );
    generate(
        &model,
        TEXT_INPUT_FIXTURE,
        &targets::text_input::TextInputTarget,
    )
    .expect("text-input-ts target renders the authored model")
}

fn artifact_bytes(path: &str) -> Vec<u8> {
    fs::read(repo_root().join(path))
        .unwrap_or_else(|error| panic!("committed artifact {path} is readable: {error}"))
}

// ---------------------------------------------------------------------------
// The authored definition (R1: authored in Rust; R2: values only, no schema)
// ---------------------------------------------------------------------------

#[test]
fn text_input_model_validates_and_round_trips_as_json() {
    let model = models::text_input::text_input_model();
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
fn text_input_definition_authors_the_full_contract_surface() {
    let model = models::text_input::text_input_model();
    let component = model
        .components
        .iter()
        .find(|component| component.id.as_str() == "text-input")
        .expect("the one component");

    // The card's 49-web-prop surface (the Svelte Props interface entries)
    // is accounted as: 41 non-callback props + the 8 callbacks. The
    // callbacks are declared as events (the Button/RangeSlider pattern);
    // onValueChange/submit/cancel/clear/focus/blur/selection-change are
    // typed, onValidationChange and onKeyDown have no EventKind and are
    // recorded as findings. Plus the 3 Rust-only props
    // (selectionStart/selectionEnd/isFocused, T §3 "Rust targets only").
    assert_eq!(
        component.props.len(),
        44,
        "41 web non-callback props + 3 Rust-only props (T §3; TXT-29)"
    );
    let web_props: Vec<&poodle_ir::Prop> = component
        .props
        .iter()
        .filter(|prop| {
            !prop.id.as_str().starts_with("selection") && prop.id.as_str() != "isFocused"
        })
        .collect();
    assert_eq!(web_props.len(), 41, "the non-callback web props");
    assert_eq!(
        component.events.len(),
        7,
        "the typed callback surface (T §5 + TXT-21)"
    );
    // The card's 49 = 41 props + 8 callbacks (7 typed + onKeyDown untyped).
    assert_eq!(
        41 + component.events.len() + 1,
        49,
        "41 props + 7 typed events + onKeyDown (native passthrough, no EventKind) = the \
         card's 49-web-prop surface"
    );
    assert_eq!(
        component
            .props
            .iter()
            .filter(|prop| prop.id.as_str() == "selectionStart"
                || prop.id.as_str() == "selectionEnd"
                || prop.id.as_str() == "isFocused")
            .count(),
        3,
        "the Rust-only trio is recorded as portable props (no rust-only flag exists)"
    );

    // The web-only surface (CROSS-03): native attributes + the validation
    // machinery + the snippets.
    let web_only: Vec<&str> = component
        .props
        .iter()
        .filter(|prop| prop.web_only)
        .map(|prop| prop.id.as_str())
        .collect();
    assert_eq!(
        web_only,
        vec![
            "autofocus",
            "spellcheck",
            "autocapitalize",
            "autocorrect",
            "enterKeyHint",
            "validate",
            "validationContext",
            "validationKey",
            "validationDebounce",
            "validateOnBlur",
            "list",
            "leading",
            "trailing",
        ],
        "the 13 web-only props (native attributes, validation machinery, snippets)"
    );

    // The expressible event surface: 7 of the 8 callbacks + the Rust-only
    // selection-change. onValidationChange/onKeyDown have no EventKind
    // (recorded finding, not typed dishonestly).
    assert_eq!(
        component.events.len(),
        7,
        "T §5 callbacks minus the two untyped"
    );
    assert!(
        component
            .events
            .iter()
            .any(|event| event.id.as_str() == "value-change" && event.name == "onValueChange"),
        "value-change is typed"
    );
    assert!(
        component
            .events
            .iter()
            .any(|event| event.id.as_str() == "clear"),
        "clear is typed (TXT-08)"
    );
    assert!(
        component
            .events
            .iter()
            .any(|event| event.id.as_str() == "selection-change"),
        "selection-change is typed for the Rust targets (TXT-21)"
    );
    assert!(
        !component
            .events
            .iter()
            .any(|event| event.name == "onValidationChange"),
        "onValidationChange has no EventKind — recorded as a finding (PayloadKind::\
         ValidationStatus exists but no kind uses it)"
    );

    // 4 emitted data-* attributes (TXT-18: three contract-documented, the
    // fourth corpus-documented) + 5 TXT-16 style props = 9.
    assert_eq!(
        component.attributes.len(),
        9,
        "the 4 data-* attributes + 5 TXT-16 style props"
    );
    let data_attributes: Vec<&str> = component
        .attributes
        .iter()
        .filter(|attribute| attribute.name.starts_with("data-"))
        .map(|attribute| attribute.name.as_str())
        .collect();
    assert_eq!(
        data_attributes,
        vec![
            "data-validation-state",
            "data-size",
            "data-density",
            "data-type",
        ],
        "the four emitted data-* names (TXT-18); three are contract-documented (the card's \
         count), data-type is corpus-documented"
    );
    assert_eq!(
        component
            .attributes
            .iter()
            .filter(|attribute| attribute.name.starts_with("--poodle-text-input-"))
            .count(),
        5,
        "the five TXT-16 style props"
    );
    assert_eq!(
        component.parts.len(),
        10,
        "the contract §2 ten-part anatomy"
    );
    assert!(
        component
            .parts
            .iter()
            .all(|part| !matches!(part.kind, poodle_ir::PartKind::Identified { .. })),
        "no identified family — the anatomy is one part per contract row (g13.018 R5)"
    );
    assert_eq!(
        component.recipe_hooks.len(),
        6,
        "the six recipe hooks of text-input.css (TXT-27)"
    );
    assert_eq!(
        model.shared_types.len(),
        9,
        "text-input-type/validation-state/input-mode/resize-direction/enter-key-hint/\
         autocorrect-mode + control-size/density/size-role"
    );

    // The R2 capability boundary: the six environment capabilities plus
    // the component-owned timers, each typed.
    let capabilities: Vec<String> = component
        .capabilities
        .iter()
        .map(|requirement| format!("{:?}", requirement.capability))
        .collect();
    assert_eq!(
        capabilities,
        vec![
            "Focus".to_owned(),
            "TextEditing".to_owned(),
            "Ime".to_owned(),
            "Clipboard".to_owned(),
            "Measurement".to_owned(),
            "Timers".to_owned(),
        ],
        "the typed capability boundary (R2) — selection rides on TextEditing + Measurement"
    );

    // The g13.018 three-way split (R3): every requirement declares all
    // four runtimes explicitly, web delegates (or provides), GPUI
    // implements, Jetstream is absent where it has no implementation —
    // absence is declared and reasoned, never inferred from silence.
    // The card's required test: TextInput states that Jetstream lacks
    // text editing.
    let split = |capability: poodle_ir::Capability| {
        let requirement = component
            .capabilities
            .iter()
            .find(|requirement| requirement.capability == capability)
            .unwrap_or_else(|| panic!("capability {capability:?} declared"));
        assert_eq!(
            requirement.runtimes.len(),
            4,
            "{capability:?}: every runtime listed explicitly (g13.018 R3)"
        );
        let rows = |provision: poodle_ir::CapabilityProvision| {
            requirement
                .runtimes
                .iter()
                .filter(|status| status.provision == provision)
                .map(|status| status.runtime)
                .collect::<Vec<_>>()
        };
        rows
    };
    let jetstream = poodle_ir::RuntimeTarget::Jetstream;
    let gpui = poodle_ir::RuntimeTarget::Gpui;
    // The card's headline: Jetstream lacks text editing, declared with a
    // reason.
    let text_editing_absent = split(poodle_ir::Capability::TextEditing)(
        poodle_ir::CapabilityProvision::Absent,
    );
    assert_eq!(
        text_editing_absent,
        vec![jetstream],
        "Jetstream lacks text editing — declared, not inferred (the card's required test)"
    );
    let text_editing_absent_row = component
        .capabilities
        .iter()
        .find(|requirement| requirement.capability == poodle_ir::Capability::TextEditing)
        .unwrap()
        .runtimes
        .iter()
        .find(|status| status.provision == poodle_ir::CapabilityProvision::Absent)
        .unwrap();
    assert!(
        !text_editing_absent_row.reason.trim().is_empty(),
        "the Jetstream absence carries a reason (g13.018 R3)"
    );
    // The rest of the split: web delegates editing/IME/clipboard/
    // measurement, GPUI provides them, Jetstream is absent; timers are
    // web-provided and native-absent.
    for capability in [
        poodle_ir::Capability::TextEditing,
        poodle_ir::Capability::Ime,
        poodle_ir::Capability::Clipboard,
        poodle_ir::Capability::Measurement,
    ] {
        let rows = split(capability);
        assert_eq!(
            rows(poodle_ir::CapabilityProvision::Delegated),
            vec![poodle_ir::RuntimeTarget::Svelte, poodle_ir::RuntimeTarget::React],
            "{capability:?}: the web runtimes delegate to the browser"
        );
        assert_eq!(
            rows(poodle_ir::CapabilityProvision::Provided),
            vec![gpui],
            "{capability:?}: GPUI implements it"
        );
        assert_eq!(
            rows(poodle_ir::CapabilityProvision::Absent),
            vec![jetstream],
            "{capability:?}: Jetstream is absent — the silent gap is declared"
        );
    }
    assert_eq!(
        split(poodle_ir::Capability::Timers)(poodle_ir::CapabilityProvision::Provided)
            .len(),
        2,
        "timers: both web runtimes own their setTimeout lifecycle"
    );
    assert_eq!(
        split(poodle_ir::Capability::Timers)(poodle_ir::CapabilityProvision::Absent)
            .len(),
        2,
        "timers: neither native has a timer surface (b049 measured)"
    );

    // Key prop defaults, byte-stable against the contract (R4).
    let prop = |name: &str| {
        component
            .props
            .iter()
            .find(|p| p.name == name)
            .unwrap_or_else(|| panic!("prop {name} authored"))
    };
    assert_eq!(prop("value").default, Some(poodle_ir::Value::Null));
    assert_eq!(
        prop("defaultValue").default,
        Some(poodle_ir::Value::string(""))
    );
    assert_eq!(prop("type").default, Some(poodle_ir::Value::member("text")));
    assert_eq!(
        prop("validationState").default,
        Some(poodle_ir::Value::member("none"))
    );
    assert_eq!(
        prop("validationDebounce").default,
        Some(poodle_ir::Value::number(300.0))
    );
    assert_eq!(
        prop("validateOnBlur").default,
        Some(poodle_ir::Value::boolean(true))
    );
    assert_eq!(
        prop("showClearButton").default,
        Some(poodle_ir::Value::boolean(true))
    );
    assert_eq!(
        prop("resize").default,
        Some(poodle_ir::Value::member("vertical"))
    );
    assert!(
        prop("validate").web_only,
        "the validator is web-only (CROSS-03)"
    );
    assert!(!prop("value").web_only, "the value is portable");

    // T §3 do-not-mix controlled pair (TXT-02) — unlike RangeSlider's
    // controlled-wins pair (b045 vocabulary note).
    assert_eq!(component.controlled_state.len(), 1);
    assert_eq!(
        component.controlled_state[0].rule,
        poodle_ir::ControlRule::DoNotMix
    );
    assert_eq!(component.controlled_state[0].controlled.as_str(), "value");
    assert_eq!(component.controlled_state[0].seed.as_str(), "defaultValue");

    // The size ladder carries the contract's §8 rungs (TXT-15).
    let size_axis = component.axes.size.as_ref().expect("size axis authored");
    assert_eq!(size_axis.ladder.len(), 5, "xs through xl");
    let density = component
        .axes
        .density
        .as_ref()
        .expect("density axis authored");
    assert_eq!(density.adjustments.len(), 2, "compact + comfortable");
    assert!(
        density.adjustments.iter().all(|adjustment| {
            adjustment
                .applies_to
                .as_ref()
                .is_some_and(|id| id.as_str() == "root")
                && adjustment.inline.is_some()
                && adjustment.block.is_some()
        }),
        "both density adjustments carry inline AND block padding on the root (T §8, TXT-15)"
    );
    assert!(
        component.axes.orientation.is_none(),
        "TextInput has no orientation axis"
    );

    // The web-only extensions: Jetstream clear-only (TXT-31) and the React
    // autocorrect omission (OBS-03).
    assert_eq!(component.extensions.len(), 2);
}

// ---------------------------------------------------------------------------
// R1/R5 — the declarative vector reference (the machine stays hand-written)
// ---------------------------------------------------------------------------

#[test]
fn text_input_vector_declares_the_hand_written_machine_semantics() {
    let model = models::text_input::text_input_model();
    let component = model
        .components
        .iter()
        .find(|component| component.id.as_str() == "text-input")
        .expect("the one component");

    // The component names the editing-model semantics through the vector
    // (R1/R5: the IR declares the machine, it does not absorb it).
    assert_eq!(
        component.conformance,
        vec![poodle_ir::Identifier::new("text-input")],
        "the editing-model semantics are pinned by the text-input vector (CROSS-18)"
    );

    let vector = model
        .conformance_vector("text-input")
        .expect("the text-input vector resolves in the model");
    // The runtimes that implement the shared edit model are the Rust
    // targets. The web runtimes have no TS text machine — the browser's
    // native editing honors the same semantics, which is the asymmetry the
    // milestone measures (R3); card 049 proves the native half.
    assert!(
        vector.applies_to.contains(&poodle_ir::RuntimeTarget::Gpui)
            && vector
                .applies_to
                .contains(&poodle_ir::RuntimeTarget::Jetstream),
        "the Rust targets implement the shared edit model"
    );
    assert!(
        !vector
            .applies_to
            .contains(&poodle_ir::RuntimeTarget::Svelte)
            && !vector.applies_to.contains(&poodle_ir::RuntimeTarget::React),
        "there is no TypeScript text machine (card R5; b047 baselined rs:text_input as \
         correctly different)"
    );
    assert!(!vector.steps.is_empty(), "the vector declares step intents");

    // R5: the executable vector file is the fixed target — and it carries
    // NO text key (GAP-01): the text machine is unit-test-pinned only.
    let vector_file =
        fs::read_to_string(repo_root().join(MACHINES_VECTOR)).expect("machines.json reads");
    let parsed: serde_json::Value =
        serde_json::from_str(&vector_file).expect("machines.json parses");
    assert!(
        parsed.get("text_input").is_none()
            && parsed.get("text").is_none()
            && parsed.get("text-input").is_none(),
        "machines.json carries no text vector (fixed target, R5) — the vector gap is a \
         finding for g13.008"
    );
}

// ---------------------------------------------------------------------------
// The shared web artifact (the card's parity test, definition-derived)
// ---------------------------------------------------------------------------

#[test]
fn both_web_components_carry_the_same_text_input_derived_artifact() {
    let files = render_text_input_artifact();
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
/// the four data-* attributes (names, forms, emission policies, value
/// domains), the five TXT-16 styleProps, and the recipe hooks — what
/// the components read instead of hard-coding.
#[test]
fn artifact_renders_parts_attributes_style_props_and_recipe_hooks() {
    let files = render_text_input_artifact();
    let contents = &files[0].contents;
    let model = models::text_input::text_input_model();
    let component = model
        .components
        .iter()
        .find(|component| component.id.as_str() == "text-input")
        .expect("the one component");

    // Parts: every part id carries its DOM class projection (T §2).
    for part in &component.parts {
        assert!(
            contents.contains(&format!("id: \"{}\"", part.id)),
            "artifact carries part '{}'",
            part.id
        );
    }
    assert!(contents.contains("className: \"poodle-text-input\""));
    assert!(contents.contains("className: \"poodle-text-input__field\""));
    assert!(contents.contains("className: \"poodle-text-input__control\""));
    assert!(contents.contains("className: \"poodle-text-input__clear\""));
    // The affix and affordance parts carry base + modifier classes.
    assert!(contents
        .contains("className: \"poodle-text-input__affix poodle-text-input__affix--prefix\""));
    assert!(contents
        .contains("className: \"poodle-text-input__affix poodle-text-input__affix--suffix\""));
    assert!(contents.contains(
        "className: \"poodle-text-input__affordance poodle-text-input__affordance--leading\""
    ));

    // Attributes: the 4 data-* names, forms, policies, and value domains.
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
        contents.contains("values: [\"none\", \"invalid\", \"valid\", \"pending\"]"),
        "validation-state domain"
    );
    assert!(
        contents.contains("values: [\"text\", \"multiline\", \"search\", \"slug\"]"),
        "type domain"
    );
    assert!(
        contents.contains("values: [\"xs\", \"sm\", \"md\", \"lg\", \"xl\"]"),
        "size domain"
    );
    assert!(
        contents.contains("values: [\"compact\", \"default\", \"comfortable\"]"),
        "density domain"
    );

    // Style props: the five TXT-16 custom properties with the visual
    // field each is fed by.
    for attribute in component
        .attributes
        .iter()
        .filter(|attribute| attribute.name.starts_with("--poodle-text-input-"))
    {
        assert!(
            contents.contains(&format!("name: \"{}\"", attribute.name)),
            "artifact carries the padding property '{}'",
            attribute.name
        );
    }
    assert!(contents.contains("source: \"controlPaddingStart\""));
    assert!(contents.contains("source: \"multilineBottomPadding\""));
    assert!(contents.contains("source: \"clearInsetInlineEnd\""));

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
// One Rust definition change updates both web artifacts in one `ir:build`
// (spec 063; the card's acceptance; the R2 proof at the artifact level)
// ---------------------------------------------------------------------------

#[test]
fn one_definition_change_moves_both_web_artifacts() {
    let mut changed = models::text_input::text_input_model();
    let component = changed
        .components
        .iter_mut()
        .find(|component| component.id.as_str() == "text-input")
        .expect("the one component");

    // One authored value change — the card's R2 proof, encoded: rename the
    // headline state attribute (`data-validation-state` →
    // `data-validation-level`).
    let validation_state = component
        .attributes
        .iter_mut()
        .find(|attribute| attribute.id.as_str() == "validation-state")
        .expect("the validation-state attribute");
    validation_state.name = "data-validation-level".to_owned();

    // And a TXT-16 style prop (`--poodle-text-input-control-padding-start`
    // → `--poodle-text-input-padding-start-v2`): the padding vocabulary
    // moves with the definition too.
    let padding_start = component
        .attributes
        .iter_mut()
        .find(|attribute| attribute.id.as_str() == "control-padding-start")
        .expect("the control-padding-start style prop");
    padding_start.name = "--poodle-text-input-padding-start-v2".to_owned();

    let files = generate(
        &changed,
        TEXT_INPUT_FIXTURE,
        &targets::text_input::TextInputTarget,
    )
    .expect("renders the changed definition");
    let contents = &files[0].contents;

    assert!(
        contents.contains("name: \"data-validation-level\""),
        "the renamed attribute lands in the artifact"
    );
    assert!(
        contents.contains("name: \"--poodle-text-input-padding-start-v2\""),
        "the renamed style prop lands in the artifact"
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
// text-input-ts)
// ---------------------------------------------------------------------------

#[test]
fn text_input_artifacts_fail_check_on_drift_and_check_never_writes() {
    let bin = env!("CARGO_BIN_EXE_poodle-codegen");
    let out = scratch("text-input-web-artifacts");
    // text-input-ts owns its nested root inside the shared generated/
    // dir.
    let root = out.join("generated").join("text-input");

    // A full write into the scratch, mirroring the Effigy selector.
    let status = Command::new(bin)
        .args([TEXT_INPUT_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "text-input-ts"])
        .current_dir(repo_root())
        .status()
        .expect("bin runs");
    assert!(status.success(), "text-input-ts write exits 0");
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
        .args([TEXT_INPUT_FIXTURE, "--out"])
        .arg(&out)
        .args(["--target", "text-input-ts", "--check"])
        .current_dir(repo_root())
        .status()
        .expect("check runs");
    assert!(!status.success(), "drift in a web artifact fails ir:check");
    assert_eq!(snapshot(&root), before, "check mode never mutates the tree");

    // The fixture itself is gated the same way: an authored-model change
    // with a stale committed fixture fails --author-text-input --check.
    let status = Command::new(bin)
        .args(["--author-text-input"])
        .arg(scratch("text-input-fixture-copy").join("text-input-model.json"))
        .current_dir(repo_root())
        .status()
        .expect("author runs");
    assert!(status.success(), "authoring writes the fixture");
}

// ---------------------------------------------------------------------------
// R1 — the web manifests carry no poodle-ir/poodle-codegen dependency
// (the card's required test; the artifact is consumed from inside the
// package that ships it, the b041 papercut fix)
// ---------------------------------------------------------------------------

#[test]
fn web_component_manifests_carry_no_poodle_ir_or_codegen_dependency() {
    for manifest in [SVELTE_MANIFEST, REACT_MANIFEST] {
        let contents =
            fs::read_to_string(repo_root().join(manifest)).expect("component manifest reads");
        assert!(
            !contents.contains("poodle-ir") && !contents.contains("poodle-codegen"),
            "{manifest} must not depend on poodle-ir or poodle-codegen (R1; the generated \
             artifact is consumed from inside the same package)"
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
