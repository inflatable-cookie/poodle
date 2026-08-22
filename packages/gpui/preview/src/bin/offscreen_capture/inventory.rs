//! g15.046/g15.047 — Button visual fixture inventory: the one Rust loader,
//! validator, and typed decoder.
//!
//! This module parses the same checked-in bytes as its TypeScript sibling,
//! `test/visual/fixtures/button-visual-inventory.ts`. Neither language is
//! generated from the other and there are no bindings between them: the
//! shared thing is one JSON file, and the cost of that choice is the small
//! duplicated roster below.
//!
//! It was extracted from `tests/visual_fixture_inventory.rs` in g15.047 so the
//! offscreen capture target and the inventory test target consume the same
//! parser — there is no third copy. Both include it by path; both provide a
//! crate-root `presentation_axes` module for the theme / control-size domain
//! (the same domain authority the capture CLI parses against).
//!
//! The module is deliberately free of gpui types: it must compile in the
//! inventory test target without the `capture` feature. No rendering happens
//! here — this is data parsing and validation only.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context as _, Result, bail};
use poodle_specs::{ButtonTone, ButtonVariant, ControlDensity};
use serde_json::{Map, Value};

use crate::presentation_axes::{ControlSize, ThemePreset};

/// Path of the one canonical file, relative to this crate's manifest.
const INVENTORY_RELATIVE_PATH: &str = "../../../test/visual/fixtures/button-visual-inventory.json";

/// The same string the TypeScript loader asserts. Button-local by design.
pub const INVENTORY_SCHEMA: &str = "poodle.button-visual-inventory.v1";

/// The frozen first batch: the denominator both languages hold. Duplicated
/// from the TypeScript roster on purpose — see the module comment.
pub const BUTTON_FIXTURE_NAMES: &[&str] = &[
    "button/rest-secondary",
    "button/variant-primary",
    "button/variant-ghost",
    "button/tone-danger",
    "button/tone-success",
    "button/tone-warning",
    "button/size-xs",
    "button/size-sm",
    "button/size-lg",
    "button/size-xl",
    "button/density-compact",
    "button/density-comfortable",
    "button/state-disabled",
    "button/state-loading",
    "button/state-pressed",
    "button/content-leading-icon",
    "button/content-icon-only",
    "button/theme-iceberg",
];

/// Portable Button contract domain. `ButtonVariant::Danger` exists in
/// `poodle_specs` as a backward-compatibility arm and is deliberately absent:
/// the contract names three variants.
const VARIANTS: &[&str] = &["primary", "secondary", "ghost"];
const TONES: &[&str] = &["default", "danger", "success", "warning"];

/// Density domain, read from the generated token definitions rather than
/// re-listed here. `presentation_axes` covers theme and control size; density
/// has no equivalent GPUI-side enumeration, so the token artifact is the
/// nearest authority.
const DENSITIES: &[poodle_tokens::density::DensityDefinition] = &[
    poodle_tokens::density::COMPACT,
    poodle_tokens::density::DEFAULT,
    poodle_tokens::density::COMFORTABLE,
];

fn is_density(value: &str) -> bool {
    DENSITIES.iter().any(|density| density.name == value)
}

fn density_labels() -> String {
    DENSITIES
        .iter()
        .map(|density| density.name)
        .collect::<Vec<_>>()
        .join(", ")
}

/// Rendering inputs that are already true of the captured frame. Interaction
/// states (`hover`, `active`, `focus`) are not fixture data.
const STATES: &[&str] = &["rest", "disabled", "loading", "pressed"];

const CONTENT_KINDS: &[&str] = &["label", "leading-icon", "icon-only"];

const REPORT_ROLES: &[&str] = &["fill", "border", "text", "shadow", "focus-ring"];

/// `g15.045` measured the adopted GPUI revision's headless window at a
/// hardcoded 2x scale factor.
const SUPPORTED_CAPTURE_SCALES: &[u64] = &[2];

const ROOT_KEYS: &[&str] = &[
    "schema",
    "component",
    "batch",
    "captureScales",
    "reportRoles",
    "fixtures",
];

const FIXTURE_KEYS: &[&str] = &[
    "name",
    "group",
    "theme",
    "size",
    "density",
    "viewport",
    "scale",
    "variant",
    "tone",
    "content",
    "state",
    "landmarks",
];

/// Sentinels that would mean "a runtime resolves this later".
const UNRESOLVED_MARKERS: &[&str] = &["", "inherit", "default-value", "__default__"];

pub fn inventory_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(INVENTORY_RELATIVE_PATH)
}

pub fn canonical_json() -> Value {
    let path = inventory_path();
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("{} is not valid JSON: {error}", path.display()))
}

/// Every problem the parse found, each naming its exact offender.
#[derive(Debug, Default)]
pub struct Problems(pub Vec<String>);

impl Problems {
    fn fail(&mut self, message: impl Into<String>) {
        self.0.push(message.into());
    }

    pub fn contains(&self, needle: &str) -> bool {
        self.0.iter().any(|problem| problem.contains(needle))
    }
}

fn key_diff(actual: &Map<String, Value>, expected: &[&str]) -> (Vec<String>, Vec<String>) {
    let missing = expected
        .iter()
        .filter(|key| !actual.contains_key(**key))
        .map(|key| (*key).to_string())
        .collect();
    let extra = actual
        .keys()
        .filter(|key| !expected.contains(&key.as_str()))
        .cloned()
        .collect();
    (missing, extra)
}

/// Read a required, fully resolved string against a closed domain. `key` is
/// the map key; `field` is how the problem names it, so a nested value reports
/// as `content.kind` rather than a bare `kind`.
fn resolved_field<'a>(
    problems: &mut Problems,
    where_: &str,
    field: &str,
    key: &str,
    row: &'a Map<String, Value>,
    in_domain: impl Fn(&str) -> bool,
    domain_label: &str,
) -> Option<&'a str> {
    match row.get(key) {
        None => {
            problems.fail(format!("{where_}: missing required field '{field}'"));
            None
        }
        Some(Value::Null) => {
            problems.fail(format!(
                "{where_}: field '{field}' is null; every value must be resolved in the file"
            ));
            None
        }
        Some(Value::String(value)) if UNRESOLVED_MARKERS.contains(&value.as_str()) => {
            problems.fail(format!(
                "{where_}: field '{field}' is the unresolved-default marker '{value}'; no runtime may supply it"
            ));
            None
        }
        Some(Value::String(value)) if in_domain(value) => Some(value.as_str()),
        Some(Value::String(value)) => {
            problems.fail(format!(
                "{where_}: field '{field}' value '{value}' is outside the domain [{domain_label}]"
            ));
            None
        }
        Some(other) => {
            problems.fail(format!("{where_}: field '{field}' must be a string, got {other}"));
            None
        }
    }
}

/// Top-level row field: the map key and the reported name are the same.
fn resolved_string<'a>(
    problems: &mut Problems,
    where_: &str,
    field: &str,
    row: &'a Map<String, Value>,
    in_domain: impl Fn(&str) -> bool,
    domain_label: &str,
) -> Option<&'a str> {
    resolved_field(problems, where_, field, field, row, in_domain, domain_label)
}

fn check_viewport(problems: &mut Problems, where_: &str, row: &Map<String, Value>) {
    let Some(value) = row.get("viewport") else {
        problems.fail(format!("{where_}: missing required field 'viewport'"));
        return;
    };
    let Some(viewport) = value.as_object() else {
        problems.fail(format!("{where_}: field 'viewport' must be an object, got {value}"));
        return;
    };
    let (missing, extra) = key_diff(viewport, &["width", "height"]);
    for key in missing {
        problems.fail(format!("{where_}: viewport is missing '{key}'"));
    }
    for key in extra {
        problems.fail(format!("{where_}: viewport has unknown key '{key}'"));
    }
    for key in ["width", "height"] {
        let Some(side) = viewport.get(key) else { continue };
        let ok = integral_number(side).is_some_and(|side| side > 0);
        if !ok {
            problems.fail(format!(
                "{where_}: viewport.{key} must be a positive whole number of logical pixels, got {side}"
            ));
        }
    }
}

/// The largest integer both languages represent exactly: JavaScript's
/// `Number.MAX_SAFE_INTEGER`, 2^53 - 1. Above it, `JSON.parse` and `f64` stop
/// agreeing with `u64`, so the loaders would drift again.
pub const MAX_EXACT_INTEGER: u64 = 9_007_199_254_740_991;

/// The numeric acceptance rule, shared with the TypeScript loader.
///
/// JSON has one number type, so `2`, `2.0`, and `2e0` are the same value.
/// `Value::as_u64()` alone is not that rule: it returns `None` for `2.0`, which
/// would reject bytes TypeScript accepts — TypeScript cannot even tell the two
/// spellings apart after `JSON.parse`. A fixture number is accepted when it is
/// finite, non-negative, mathematically integral, and no larger than
/// `MAX_EXACT_INTEGER`.
///
/// Returns the normalized value, or `None` when the rule rejects it.
pub fn integral_number(value: &Value) -> Option<u64> {
    if let Some(integer) = value.as_u64() {
        return (integer <= MAX_EXACT_INTEGER).then_some(integer);
    }
    let float = value.as_f64()?;
    let integral = float.is_finite()
        && float >= 0.0
        && float.fract() == 0.0
        && float <= MAX_EXACT_INTEGER as f64;
    integral.then(|| float as u64)
}

/// Compare a declared array against an expected one element by element.
///
/// Deliberately not `filter_map(Value::as_str)` and not a join: filtering
/// would silently discard an inserted number or null, and joining would accept
/// a collapsed element (`["root content"]` reading as `["root", "content"]`).
/// Either would let this loader and its TypeScript sibling disagree about the
/// same bytes, which is the one thing a shared fixture parser must not do.
///
/// Returns a problem string, or `None` when the array matches exactly.
fn exact_string_array_problem(
    label: &str,
    value: Option<&Value>,
    expected: &[&str],
) -> Option<String> {
    let Some(Value::Array(entries)) = value else {
        return Some(format!(
            "{label} must be an array of strings, got {}",
            value.unwrap_or(&Value::Null)
        ));
    };
    if entries.len() != expected.len() {
        return Some(format!(
            "{label} must be exactly [{}] ({} entries), got {}",
            expected.join(", "),
            expected.len(),
            Value::Array(entries.clone())
        ));
    }
    for (index, entry) in entries.iter().enumerate() {
        let Some(text) = entry.as_str() else {
            return Some(format!(
                "{label} entry {index} must be the string '{}', got {entry}",
                expected[index]
            ));
        };
        if text != expected[index] {
            return Some(format!(
                "{label} entry {index} must be '{}', got {entry}",
                expected[index]
            ));
        }
    }
    None
}

/// The landmark set is derived from the case, not authored freely.
fn expected_landmarks(content_kind: &str, state: &str) -> Vec<&'static str> {
    let mut landmarks = vec!["root", "content"];
    if content_kind == "leading-icon" || content_kind == "icon-only" {
        landmarks.push("icon");
    }
    if state == "loading" {
        landmarks.push("spinner");
    }
    landmarks
}

fn icon_registry() -> BTreeSet<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../packages/core/src/icons/default-icons.json");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    let value: Value = serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("{} is not valid JSON: {error}", path.display()));
    value["icons"]
        .as_array()
        .expect("default-icons.json must hold an 'icons' array")
        .iter()
        .filter_map(|entry| entry.as_str().map(str::to_string))
        .collect()
}

fn check_content(
    problems: &mut Problems,
    where_: &str,
    row: &Map<String, Value>,
    icons: &BTreeSet<String>,
) -> Option<String> {
    let Some(value) = row.get("content") else {
        problems.fail(format!("{where_}: missing required field 'content'"));
        return None;
    };
    let Some(content) = value.as_object() else {
        problems.fail(format!("{where_}: field 'content' must be an object, got {value}"));
        return None;
    };
    let kind = resolved_field(
        problems,
        where_,
        "content.kind",
        "kind",
        content,
        |value| CONTENT_KINDS.contains(&value),
        &CONTENT_KINDS.join(", "),
    )?
    .to_string();

    let expected_keys: &[&str] = match kind.as_str() {
        "label" => &["kind", "label"],
        "leading-icon" => &["kind", "label", "icon"],
        _ => &["kind", "icon", "ariaLabel"],
    };
    let (missing, extra) = key_diff(content, expected_keys);
    for key in &missing {
        problems.fail(format!("{where_}: content '{kind}' is missing '{key}'"));
    }
    for key in &extra {
        problems.fail(format!("{where_}: content '{kind}' has unknown key '{key}'"));
    }
    if !missing.is_empty() || !extra.is_empty() {
        return None;
    }

    for key in ["label", "ariaLabel"] {
        let Some(text) = content.get(key) else { continue };
        if !text.as_str().is_some_and(|text| !text.is_empty()) {
            problems.fail(format!(
                "{where_}: content.{key} must be a non-empty string, got {text}"
            ));
            return None;
        }
    }
    if let Some(icon) = content.get("icon") {
        let named = icon.as_str().unwrap_or_default();
        if !icons.contains(named) {
            problems.fail(format!(
                "{where_}: content.icon '{named}' is not in the default icon registry (packages/core/src/icons/default-icons.json)"
            ));
            return None;
        }
    }
    Some(kind)
}

/// Validate an already-decoded value; collect every problem so a planted fault
/// is reported by exact fixture name rather than aborting on the first one.
pub fn validate(raw: &Value) -> Problems {
    let mut problems = Problems::default();
    let Some(root) = raw.as_object() else {
        problems.fail(format!("inventory root must be an object, got {raw}"));
        return problems;
    };

    let (missing, extra) = key_diff(root, ROOT_KEYS);
    for key in missing {
        problems.fail(format!("inventory root is missing '{key}'"));
    }
    for key in extra {
        problems.fail(format!("inventory root has unknown key '{key}'"));
    }

    if root.get("schema").and_then(Value::as_str) != Some(INVENTORY_SCHEMA) {
        problems.fail(format!(
            "inventory schema must be '{INVENTORY_SCHEMA}', got {}",
            root.get("schema").unwrap_or(&Value::Null)
        ));
    }
    if root.get("component").and_then(Value::as_str) != Some("button") {
        problems.fail(format!(
            "inventory component must be 'button' - this batch is Button-only, got {}",
            root.get("component").unwrap_or(&Value::Null)
        ));
    }
    if !root
        .get("batch")
        .and_then(Value::as_str)
        .is_some_and(|batch| !batch.is_empty())
    {
        problems.fail("inventory batch must be a non-empty string");
    }

    // Normalized once, so a row's `scale` is compared by value rather than by
    // spelling: `captureScales: [2.0]` and `scale: 2` are the same declaration.
    let capture_scales: Vec<u64> = root
        .get("captureScales")
        .and_then(Value::as_array)
        .map(|scales| scales.iter().filter_map(integral_number).collect())
        .unwrap_or_default();
    match root.get("captureScales").and_then(Value::as_array) {
        Some(scales) if !scales.is_empty() => {
            for scale in scales {
                let accepted = integral_number(scale)
                    .is_some_and(|scale| SUPPORTED_CAPTURE_SCALES.contains(&scale));
                if !accepted {
                    problems.fail(format!(
                        "inventory captureScales entry {scale} is outside the supported set [2]"
                    ));
                }
            }
        }
        _ => problems.fail("inventory captureScales must be a non-empty array"),
    }

    if let Some(problem) = exact_string_array_problem(
        "inventory reportRoles",
        root.get("reportRoles"),
        REPORT_ROLES,
    ) {
        problems.fail(problem);
    }

    let Some(fixtures) = root.get("fixtures").and_then(Value::as_array) else {
        problems.fail("inventory fixtures must be an array");
        return problems;
    };

    let icons = icon_registry();
    let mut seen: BTreeSet<String> = BTreeSet::new();

    for (index, entry) in fixtures.iter().enumerate() {
        let Some(row) = entry.as_object() else {
            problems.fail(format!("fixture at index {index} must be an object, got {entry}"));
            continue;
        };
        let name = row.get("name").and_then(Value::as_str).unwrap_or_default();
        if name.is_empty() {
            problems.fail(format!("fixture at index {index}: 'name' must be a non-empty string"));
            continue;
        }
        let where_ = format!("fixture '{name}'");

        if !BUTTON_FIXTURE_NAMES.contains(&name) {
            problems.fail(format!(
                "unknown fixture name '{name}': not one of the {} g15.046 identities",
                BUTTON_FIXTURE_NAMES.len()
            ));
        } else if seen.contains(name) {
            problems.fail(format!("duplicate fixture name '{name}'"));
        }
        seen.insert(name.to_string());

        let (missing, extra) = key_diff(row, FIXTURE_KEYS);
        for key in missing {
            problems.fail(format!("{where_}: missing required field '{key}'"));
        }
        for key in extra {
            problems.fail(format!("{where_}: unknown field '{key}'"));
        }

        if !row
            .get("group")
            .and_then(Value::as_str)
            .is_some_and(|group| !group.is_empty())
        {
            problems.fail(format!("{where_}: 'group' must be a non-empty string"));
        }

        resolved_string(
            &mut problems,
            &where_,
            "theme",
            row,
            |value| ThemePreset::parse(value).is_some(),
            "the GPUI ThemePreset domain",
        );
        resolved_string(
            &mut problems,
            &where_,
            "size",
            row,
            |value| ControlSize::parse(value).is_some(),
            "xs, sm, md, lg, xl",
        );
        resolved_string(
            &mut problems,
            &where_,
            "density",
            row,
            is_density,
            &density_labels(),
        );
        resolved_string(
            &mut problems,
            &where_,
            "variant",
            row,
            |value| VARIANTS.contains(&value),
            &VARIANTS.join(", "),
        );
        resolved_string(
            &mut problems,
            &where_,
            "tone",
            row,
            |value| TONES.contains(&value),
            &TONES.join(", "),
        );
        let state = resolved_string(
            &mut problems,
            &where_,
            "state",
            row,
            |value| STATES.contains(&value),
            &STATES.join(", "),
        )
        .map(str::to_string);

        check_viewport(&mut problems, &where_, row);

        match row.get("scale") {
            None | Some(Value::Null) => {
                problems.fail(format!("{where_}: missing required field 'scale'"));
            }
            Some(scale) => {
                let accepted =
                    integral_number(scale).is_some_and(|scale| capture_scales.contains(&scale));
                if !accepted {
                    problems.fail(format!(
                        "{where_}: scale {scale} is not one of the inventory captureScales {capture_scales:?}"
                    ));
                }
            }
        }

        let kind = check_content(&mut problems, &where_, row, &icons);

        if let (Some(kind), Some(state)) = (kind, state) {
            let expected = expected_landmarks(&kind, &state);
            if let Some(problem) = exact_string_array_problem(
                &format!("{where_}: landmarks"),
                row.get("landmarks"),
                &expected,
            ) {
                problems.fail(format!("{problem} — content '{kind}', state '{state}'"));
            }
        }
    }

    for name in BUTTON_FIXTURE_NAMES {
        if !seen.contains(*name) {
            problems.fail(format!("missing fixture name '{name}'"));
        }
    }

    problems
}

// ── Typed decode (g15.047) ──────────────────────────────────────────────
//
// Validation stays stringly and problem-collecting above; below is the typed
// read the capture target consumes. `decode_fixtures` only runs after a clean
// `validate`, so every `expect` here is a validation bug, not input handling.

/// Declared logical viewport, in whole logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureViewport {
    pub width: u64,
    pub height: u64,
}

/// The fixture's content shape, decoded to a closed enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixtureContent {
    Label { label: String },
    LeadingIcon { label: String, icon: String },
    IconOnly { icon: String, aria_label: String },
}

/// The rendered state the capture must reach directly (no input replay).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureState {
    Rest,
    Disabled,
    Loading,
    Pressed,
}

/// One inventory row as typed capture input.
#[derive(Debug, Clone, PartialEq)]
pub struct ButtonFixture {
    pub name: String,
    pub theme: ThemePreset,
    pub size: ControlSize,
    pub density: ControlDensity,
    pub viewport: FixtureViewport,
    pub scale: u64,
    pub variant: ButtonVariant,
    pub tone: ButtonTone,
    pub content: FixtureContent,
    pub state: FixtureState,
    pub landmarks: Vec<String>,
}

fn required_string<'a>(row: &'a Map<String, Value>, key: &str) -> &'a str {
    row.get(key)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("validated inventory: fixture string field '{key}' is unreadable"))
}

fn decode_variant(value: &str) -> ButtonVariant {
    match value {
        "primary" => ButtonVariant::Primary,
        "secondary" => ButtonVariant::Secondary,
        "ghost" => ButtonVariant::Ghost,
        other => panic!("validated inventory: variant '{other}' has no decode arm"),
    }
}

fn decode_tone(value: &str) -> ButtonTone {
    match value {
        "default" => ButtonTone::Default,
        "danger" => ButtonTone::Danger,
        "success" => ButtonTone::Success,
        "warning" => ButtonTone::Warning,
        other => panic!("validated inventory: tone '{other}' has no decode arm"),
    }
}

fn decode_density(value: &str) -> ControlDensity {
    match value {
        "compact" => ControlDensity::Compact,
        "default" => ControlDensity::Default,
        "comfortable" => ControlDensity::Comfortable,
        other => panic!("validated inventory: density '{other}' has no decode arm"),
    }
}

fn decode_state(value: &str) -> FixtureState {
    match value {
        "rest" => FixtureState::Rest,
        "disabled" => FixtureState::Disabled,
        "loading" => FixtureState::Loading,
        "pressed" => FixtureState::Pressed,
        other => panic!("validated inventory: state '{other}' has no decode arm"),
    }
}

fn decode_content(row: &Map<String, Value>) -> FixtureContent {
    let content = row
        .get("content")
        .and_then(Value::as_object)
        .expect("validated inventory: content is an object");
    match required_string(content, "kind") {
        "label" => FixtureContent::Label {
            label: required_string(content, "label").to_string(),
        },
        "leading-icon" => FixtureContent::LeadingIcon {
            label: required_string(content, "label").to_string(),
            icon: required_string(content, "icon").to_string(),
        },
        "icon-only" => FixtureContent::IconOnly {
            icon: required_string(content, "icon").to_string(),
            aria_label: required_string(content, "ariaLabel").to_string(),
        },
        other => panic!("validated inventory: content kind '{other}' has no decode arm"),
    }
}

fn decode_fixture(entry: &Value) -> ButtonFixture {
    let row = entry
        .as_object()
        .expect("validated inventory: fixture is an object");
    let viewport = row
        .get("viewport")
        .and_then(Value::as_object)
        .expect("validated inventory: viewport is an object");
    let width = viewport
        .get("width")
        .and_then(integral_number)
        .expect("validated inventory: viewport.width is integral");
    let height = viewport
        .get("height")
        .and_then(integral_number)
        .expect("validated inventory: viewport.height is integral");
    let scale = row
        .get("scale")
        .and_then(integral_number)
        .expect("validated inventory: scale is integral");
    let landmarks = row
        .get("landmarks")
        .and_then(Value::as_array)
        .expect("validated inventory: landmarks is an array")
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .expect("validated inventory: landmark is a string")
                .to_string()
        })
        .collect();
    ButtonFixture {
        name: required_string(row, "name").to_string(),
        theme: ThemePreset::parse(required_string(row, "theme"))
            .expect("validated inventory: theme parses"),
        size: ControlSize::parse(required_string(row, "size"))
            .expect("validated inventory: size parses"),
        density: decode_density(required_string(row, "density")),
        viewport: FixtureViewport { width, height },
        scale,
        variant: decode_variant(required_string(row, "variant")),
        tone: decode_tone(required_string(row, "tone")),
        content: decode_content(row),
        state: decode_state(required_string(row, "state")),
        landmarks,
    }
}

/// Decode an already-validated inventory value into typed fixtures. Callers
/// must run [`validate`] first; this panics on anything validation would have
/// rejected rather than re-reporting problems.
pub fn decode_fixtures(raw: &Value) -> Vec<ButtonFixture> {
    raw.get("fixtures")
        .and_then(Value::as_array)
        .expect("validated inventory: fixtures is an array")
        .iter()
        .map(decode_fixture)
        .collect()
}

/// Read, validate, and decode the canonical inventory. Any validation problem
/// is a hard failure naming every offender — the capture target never renders
/// from an inventory the test target would reject.
pub fn load_inventory() -> Result<Vec<ButtonFixture>> {
    let path = inventory_path();
    let text = fs::read_to_string(&path)
        .with_context(|| format!("cannot read {}", path.display()))?;
    let raw: Value = serde_json::from_str(&text)
        .with_context(|| format!("{} is not valid JSON", path.display()))?;
    let problems = validate(&raw);
    if !problems.0.is_empty() {
        bail!(
            "button visual inventory is invalid:\n  - {}",
            problems.0.join("\n  - ")
        );
    }
    Ok(decode_fixtures(&raw))
}
