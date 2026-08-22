//! g15.046 — Button visual fixture inventory: Rust loader and validator.
//!
//! This module parses the same checked-in bytes as its TypeScript sibling,
//! `test/visual/fixtures/button-visual-inventory.ts`. Neither language is
//! generated from the other and there are no bindings between them: the
//! shared thing is one JSON file, and the cost of that choice is the small
//! duplicated roster below.
//!
//! It lives in the GPUI preview crate because GPUI is the runtime that will
//! consume the inventory in `g15.047`. Theme and control-size names are
//! therefore validated against `presentation_axes` — the same domain authority
//! the offscreen capture target already parses its CLI against — rather than a
//! third enumeration that could drift.
//!
//! No rendering happens here. This test opens no window, builds no GPUI
//! context, captures nothing, and compares no pixels.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use serde_json::{Map, Value};

// Included by path, the same way the offscreen capture target includes it.
// This test only needs the two `parse` entry points; the rest of the module is
// for the preview shell and the capture binary.
#[allow(dead_code)]
#[path = "../src/presentation_axes.rs"]
mod presentation_axes;

use presentation_axes::{ControlSize, ThemePreset};

/// Path of the one canonical file, relative to this crate's manifest.
const INVENTORY_RELATIVE_PATH: &str = "../../../test/visual/fixtures/button-visual-inventory.json";

/// The same string the TypeScript loader asserts. Button-local by design.
const INVENTORY_SCHEMA: &str = "poodle.button-visual-inventory.v1";

/// The frozen first batch: the denominator both languages hold. Duplicated
/// from the TypeScript roster on purpose — see the module comment.
const BUTTON_FIXTURE_NAMES: &[&str] = &[
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

fn inventory_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(INVENTORY_RELATIVE_PATH)
}

fn canonical_json() -> Value {
    let path = inventory_path();
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("{} is not valid JSON: {error}", path.display()))
}

/// Every problem the parse found, each naming its exact offender.
#[derive(Debug, Default)]
struct Problems(Vec<String>);

impl Problems {
    fn fail(&mut self, message: impl Into<String>) {
        self.0.push(message.into());
    }

    fn contains(&self, needle: &str) -> bool {
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
        let ok = side.as_u64().is_some_and(|side| side > 0);
        if !ok {
            problems.fail(format!(
                "{where_}: viewport.{key} must be a positive whole number of logical pixels, got {side}"
            ));
        }
    }
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
fn validate(raw: &Value) -> Problems {
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

    let capture_scales: Vec<u64> = root
        .get("captureScales")
        .and_then(Value::as_array)
        .map(|scales| scales.iter().filter_map(Value::as_u64).collect())
        .unwrap_or_default();
    match root.get("captureScales").and_then(Value::as_array) {
        Some(scales) if !scales.is_empty() => {
            for scale in scales {
                let accepted = scale
                    .as_u64()
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
                let accepted = scale
                    .as_u64()
                    .is_some_and(|scale| capture_scales.contains(&scale));
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

/// Plant a fault on a clone of the canonical file and return the problems.
/// The canonical file on disk is never mutated.
fn problems_for(mutate: impl FnOnce(&mut Value)) -> Problems {
    let mut planted = canonical_json();
    mutate(&mut planted);
    let problems = validate(&planted);
    assert!(
        !problems.0.is_empty(),
        "expected the planted inventory to be rejected, but it validated clean"
    );
    problems
}

fn row_at<'a>(inventory: &'a mut Value, name: &str) -> &'a mut Map<String, Value> {
    inventory["fixtures"]
        .as_array_mut()
        .expect("fixtures array")
        .iter_mut()
        .find(|row| row["name"] == name)
        .unwrap_or_else(|| panic!("planting error: no fixture named '{name}'"))
        .as_object_mut()
        .expect("fixture object")
}

fn assert_problem(problems: &Problems, needle: &str) {
    assert!(
        problems.contains(needle),
        "expected a problem containing {needle:?}, got {:#?}",
        problems.0
    );
}

#[test]
fn canonical_inventory_validates_and_holds_exactly_eighteen_identities() {
    let inventory = canonical_json();
    let problems = validate(&inventory);
    assert!(problems.0.is_empty(), "canonical inventory is invalid: {:#?}", problems.0);

    let names: Vec<&str> = inventory["fixtures"]
        .as_array()
        .expect("fixtures array")
        .iter()
        .map(|row| row["name"].as_str().expect("fixture name"))
        .collect();
    assert_eq!(names.len(), 18);
    assert_eq!(names, BUTTON_FIXTURE_NAMES);
    assert_eq!(names.iter().collect::<BTreeSet<_>>().len(), 18);
}

#[test]
fn rust_and_typescript_read_the_same_checked_in_file() {
    let typescript = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../test/visual/fixtures/button-visual-inventory.ts");
    let source = fs::read_to_string(&typescript)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", typescript.display()));
    assert!(source.contains("button-visual-inventory.json"));
    assert!(source.contains(INVENTORY_SCHEMA));
    for name in BUTTON_FIXTURE_NAMES {
        assert!(source.contains(name), "TypeScript roster is missing '{name}'");
    }
}

#[test]
fn every_row_resolves_the_fixed_environment() {
    let inventory = canonical_json();
    for row in inventory["fixtures"].as_array().expect("fixtures array") {
        assert_eq!(row["viewport"]["width"], 240);
        assert_eq!(row["viewport"]["height"], 80);
        assert_eq!(row["scale"], 2);
        let theme = row["theme"].as_str().expect("theme");
        assert!(ThemePreset::parse(theme).is_some(), "unknown theme '{theme}'");
        let size = row["size"].as_str().expect("size");
        assert!(ControlSize::parse(size).is_some(), "unknown control size '{size}'");
    }
}

#[test]
fn missing_identity_is_named() {
    let problems = problems_for(|inventory| {
        let fixtures = inventory["fixtures"].as_array_mut().expect("fixtures array");
        fixtures.retain(|row| row["name"] != "button/size-lg");
    });
    assert_problem(&problems, "missing fixture name 'button/size-lg'");
}

#[test]
fn extra_identity_is_named() {
    let problems = problems_for(|inventory| {
        let mut extra = Value::Object(row_at(inventory, "button/tone-danger").clone());
        extra["name"] = Value::String("button/tone-info".into());
        inventory["fixtures"].as_array_mut().expect("fixtures array").push(extra);
    });
    assert_problem(&problems, "unknown fixture name 'button/tone-info'");
}

#[test]
fn duplicate_identity_is_named() {
    let problems = problems_for(|inventory| {
        let clone = Value::Object(row_at(inventory, "button/variant-ghost").clone());
        inventory["fixtures"].as_array_mut().expect("fixtures array").push(clone);
    });
    assert_problem(&problems, "duplicate fixture name 'button/variant-ghost'");
}

#[test]
fn unknown_domain_values_are_named() {
    let tone = problems_for(|inventory| {
        row_at(inventory, "button/tone-danger").insert("tone".into(), "info".into());
    });
    assert_problem(&tone, "fixture 'button/tone-danger': field 'tone' value 'info'");

    let variant = problems_for(|inventory| {
        row_at(inventory, "button/variant-primary").insert("variant".into(), "danger".into());
    });
    assert_problem(
        &variant,
        "fixture 'button/variant-primary': field 'variant' value 'danger'",
    );

    let theme = problems_for(|inventory| {
        row_at(inventory, "button/theme-iceberg").insert("theme".into(), "iceberg-light".into());
    });
    assert_problem(
        &theme,
        "fixture 'button/theme-iceberg': field 'theme' value 'iceberg-light'",
    );

    let size = problems_for(|inventory| {
        row_at(inventory, "button/size-xl").insert("size".into(), "xxl".into());
    });
    assert_problem(&size, "fixture 'button/size-xl': field 'size' value 'xxl'");

    let state = problems_for(|inventory| {
        row_at(inventory, "button/state-pressed").insert("state".into(), "hover".into());
    });
    assert_problem(
        &state,
        "fixture 'button/state-pressed': field 'state' value 'hover'",
    );

    let icon = problems_for(|inventory| {
        row_at(inventory, "button/content-leading-icon")["content"]["icon"] =
            Value::String("rocket-ship".into());
    });
    assert_problem(
        &icon,
        "fixture 'button/content-leading-icon': content.icon 'rocket-ship'",
    );
}

#[test]
fn unresolved_defaults_are_named() {
    let null_density = problems_for(|inventory| {
        row_at(inventory, "button/density-compact").insert("density".into(), Value::Null);
    });
    assert_problem(
        &null_density,
        "fixture 'button/density-compact': field 'density' is null",
    );

    let marker = problems_for(|inventory| {
        row_at(inventory, "button/rest-secondary").insert("theme".into(), "inherit".into());
    });
    assert_problem(
        &marker,
        "fixture 'button/rest-secondary': field 'theme' is the unresolved-default marker 'inherit'",
    );

    let absent = problems_for(|inventory| {
        row_at(inventory, "button/variant-ghost").remove("tone");
    });
    assert_problem(
        &absent,
        "fixture 'button/variant-ghost': missing required field 'tone'",
    );
}

#[test]
fn invalid_viewport_and_scale_are_named() {
    let zero = problems_for(|inventory| {
        row_at(inventory, "button/size-xs")["viewport"]["width"] = Value::from(0);
    });
    assert_problem(&zero, "fixture 'button/size-xs': viewport.width must be a positive whole");

    let fractional = problems_for(|inventory| {
        row_at(inventory, "button/size-sm")["viewport"]["height"] = Value::from(80.5);
    });
    assert_problem(
        &fractional,
        "fixture 'button/size-sm': viewport.height must be a positive whole",
    );

    let stray_key = problems_for(|inventory| {
        row_at(inventory, "button/size-lg")["viewport"]
            .as_object_mut()
            .expect("viewport object")
            .insert("dpr".into(), Value::from(2));
    });
    assert_problem(&stray_key, "fixture 'button/size-lg': viewport has unknown key 'dpr'");

    let row_scale = problems_for(|inventory| {
        row_at(inventory, "button/state-loading").insert("scale".into(), Value::from(1));
    });
    assert_problem(&row_scale, "fixture 'button/state-loading': scale 1 is not one of");

    let unsupported = problems_for(|inventory| {
        inventory["captureScales"] = Value::from(vec![3]);
    });
    assert_problem(
        &unsupported,
        "inventory captureScales entry 3 is outside the supported set",
    );
}

#[test]
fn shape_faults_keep_the_format_button_specific() {
    let props_bag = problems_for(|inventory| {
        row_at(inventory, "button/rest-secondary")
            .insert("props".into(), serde_json::json!({ "variant": "secondary" }));
    });
    assert_problem(&props_bag, "fixture 'button/rest-secondary': unknown field 'props'");

    let second_component = problems_for(|inventory| {
        inventory["component"] = Value::String("icon-button".into());
    });
    assert_problem(&second_component, "inventory component must be 'button'");

    let wrong_schema = problems_for(|inventory| {
        inventory["schema"] = Value::String("poodle.component-visual-inventory.v1".into());
    });
    assert_problem(&wrong_schema, "inventory schema must be");

    let not_an_object = validate(&Value::Array(vec![]));
    assert_problem(&not_an_object, "inventory root must be an object");
}

/// Arrays are compared element by element, never joined and never filtered.
/// A filter accepts an inserted non-string; a join accepts a collapsed
/// element. Both would let this loader and the TypeScript one disagree about
/// the same bytes. Mirrors the `declared arrays must match element by element`
/// block in `button-visual-inventory.test.ts`.
#[test]
fn declared_arrays_must_match_element_by_element() {
    let collapsed_role = problems_for(|inventory| {
        inventory["reportRoles"] =
            serde_json::json!(["fill border", "text", "shadow", "focus-ring"]);
    });
    assert_problem(
        &collapsed_role,
        "inventory reportRoles must be exactly [fill, border, text, shadow, focus-ring] (5 entries)",
    );

    let non_string_role = problems_for(|inventory| {
        inventory["reportRoles"] = serde_json::json!(["fill", "border", 3, "shadow", "focus-ring"]);
    });
    assert_problem(
        &non_string_role,
        "inventory reportRoles entry 2 must be the string 'text', got 3",
    );

    let inserted_role = problems_for(|inventory| {
        inventory["reportRoles"] =
            serde_json::json!(["fill", "border", "text", "shadow", "focus-ring", null]);
    });
    assert_problem(
        &inserted_role,
        "inventory reportRoles must be exactly [fill, border, text, shadow, focus-ring] (5 entries)",
    );

    let not_an_array = problems_for(|inventory| {
        inventory["reportRoles"] = Value::String("fill border text shadow focus-ring".into());
    });
    assert_problem(&not_an_array, "inventory reportRoles must be an array of strings");

    let collapsed_landmark = problems_for(|inventory| {
        row_at(inventory, "button/rest-secondary")
            .insert("landmarks".into(), serde_json::json!(["root content"]));
    });
    assert_problem(
        &collapsed_landmark,
        "fixture 'button/rest-secondary': landmarks must be exactly [root, content] (2 entries)",
    );

    let non_string_landmark = problems_for(|inventory| {
        row_at(inventory, "button/state-loading")
            .insert("landmarks".into(), serde_json::json!(["root", "content", 7]));
    });
    assert_problem(
        &non_string_landmark,
        "fixture 'button/state-loading': landmarks entry 2 must be the string 'spinner', got 7",
    );

    let inserted_landmark = problems_for(|inventory| {
        row_at(inventory, "button/content-leading-icon").insert(
            "landmarks".into(),
            serde_json::json!(["root", "content", "icon", null]),
        );
    });
    assert_problem(
        &inserted_landmark,
        "fixture 'button/content-leading-icon': landmarks must be exactly [root, content, icon] (3 entries)",
    );

    let landmarks_not_an_array = problems_for(|inventory| {
        row_at(inventory, "button/theme-iceberg")
            .insert("landmarks".into(), Value::String("root content".into()));
    });
    assert_problem(
        &landmarks_not_an_array,
        "fixture 'button/theme-iceberg': landmarks must be an array of strings",
    );
}

#[test]
fn content_and_landmark_faults_are_named() {
    let missing_aria = problems_for(|inventory| {
        row_at(inventory, "button/content-icon-only")["content"]
            .as_object_mut()
            .expect("content object")
            .remove("ariaLabel");
    });
    assert_problem(
        &missing_aria,
        "fixture 'button/content-icon-only': content 'icon-only' is missing 'ariaLabel'",
    );

    let stray_icon = problems_for(|inventory| {
        row_at(inventory, "button/rest-secondary")["content"]
            .as_object_mut()
            .expect("content object")
            .insert("icon".into(), "play".into());
    });
    assert_problem(
        &stray_icon,
        "fixture 'button/rest-secondary': content 'label' has unknown key 'icon'",
    );

    let no_spinner = problems_for(|inventory| {
        row_at(inventory, "button/state-loading")
            .insert("landmarks".into(), serde_json::json!(["root", "content"]));
    });
    assert_problem(
        &no_spinner,
        "fixture 'button/state-loading': landmarks must be exactly [root, content, spinner]",
    );

    let no_icon = problems_for(|inventory| {
        row_at(inventory, "button/content-leading-icon")
            .insert("landmarks".into(), serde_json::json!(["root", "content"]));
    });
    assert_problem(
        &no_icon,
        "fixture 'button/content-leading-icon': landmarks must be exactly [root, content, icon]",
    );
}
