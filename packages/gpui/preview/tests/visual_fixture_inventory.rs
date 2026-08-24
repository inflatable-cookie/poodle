//! g15.046 — Button visual fixture inventory: Rust loader and validator tests.
//!
//! The parser/validator itself moved in g15.047 to
//! `src/bin/window_capture/inventory.rs`, included here by path, so this
//! test target and the offscreen capture target consume the same code — there
//! is no third parser. It parses the same checked-in bytes as its TypeScript
//! sibling, `test/visual/fixtures/button-visual-inventory.ts`. Neither language
//! is generated from the other and there are no bindings between them: the
//! shared thing is one JSON file, and the cost of that choice is the small
//! duplicated roster the module holds.
//!
//! It lives in the GPUI preview crate because GPUI is the runtime that
//! consumes the inventory in `g15.047`. Theme and control-size names are
//! validated against `presentation_axes` — the same domain authority the
//! offscreen capture target parses its CLI against — rather than a third
//! enumeration that could drift.
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

// The one Rust inventory parser, shared with the capture binary. Items the
// capture target needs but these tests do not (the typed decode layer) stay
// reachable without per-item allows.
#[allow(dead_code)]
#[path = "../src/bin/window_capture/inventory.rs"]
mod inventory;

use inventory::{
    BUTTON_FIXTURE_NAMES, INVENTORY_SCHEMA, MAX_EXACT_INTEGER, Problems, canonical_json,
    integral_number, inventory_path, validate,
};
use presentation_axes::{ControlSize, ThemePreset};

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

/// JSON has one number type, so `2` and `2.0` are the same value and both
/// loaders must agree about them. The accepted-spelling case is planted on the
/// canonical *text*, because that is the only place the spelling exists —
/// after parsing, `2.0` and `2` are indistinguishable to the TypeScript
/// sibling. Mirrors the `numeric spelling is normalized, numeric domain is not`
/// block in `button-visual-inventory.test.ts`.
#[test]
fn numeric_spelling_is_normalized_consistently() {
    let path = inventory_path();
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
        .replace("\"captureScales\": [2]", "\"captureScales\": [2.0]")
        .replace("\"scale\": 2", "\"scale\": 2.0")
        .replace("\"width\": 240", "\"width\": 240.0")
        .replace("\"height\": 80", "\"height\": 80.0");

    assert!(text.contains("\"captureScales\": [2.0]"));
    assert!(text.contains("\"scale\": 2.0"));
    assert!(text.contains("\"width\": 240.0"));
    assert!(text.contains("\"height\": 80.0"));

    let decimal: Value = serde_json::from_str(&text).expect("decimal spelling is valid JSON");
    let problems = validate(&decimal);
    assert!(
        problems.0.is_empty(),
        "integral decimal spellings must be accepted: {:#?}",
        problems.0
    );
    assert_eq!(
        decimal["fixtures"].as_array().expect("fixtures array").len(),
        18
    );
}

#[test]
fn numeric_domain_faults_are_still_rejected() {
    let fractional_scale = problems_for(|inventory| {
        row_at(inventory, "button/size-xs").insert("scale".into(), Value::from(2.5));
    });
    assert_problem(&fractional_scale, "fixture 'button/size-xs': scale 2.5 is not one of");

    let fractional_declared = problems_for(|inventory| {
        inventory["captureScales"] = serde_json::json!([2.5]);
    });
    assert_problem(
        &fractional_declared,
        "inventory captureScales entry 2.5 is outside the supported set",
    );

    let negative_viewport = problems_for(|inventory| {
        row_at(inventory, "button/size-sm")["viewport"]["width"] = Value::from(-240);
    });
    assert_problem(
        &negative_viewport,
        "fixture 'button/size-sm': viewport.width must be a positive whole",
    );

    let negative_scale = problems_for(|inventory| {
        row_at(inventory, "button/size-lg").insert("scale".into(), Value::from(-2));
    });
    assert_problem(&negative_scale, "fixture 'button/size-lg': scale -2 is not one of");

    let numeric_string = problems_for(|inventory| {
        row_at(inventory, "button/variant-ghost").insert("scale".into(), Value::String("2".into()));
    });
    assert_problem(
        &numeric_string,
        "fixture 'button/variant-ghost': scale \"2\" is not one of",
    );

    // 1e16 is integral but beyond 2^53 - 1, where the two languages stop
    // agreeing. Both reject it, so neither can accept a size the other cannot.
    let beyond_exact_range = problems_for(|inventory| {
        row_at(inventory, "button/theme-iceberg")["viewport"]["width"] = Value::from(1e16);
    });
    assert_problem(
        &beyond_exact_range,
        "fixture 'button/theme-iceberg': viewport.width must be a positive whole",
    );
}

#[test]
fn the_numeric_rule_itself() {
    assert_eq!(integral_number(&Value::from(2)), Some(2));
    assert_eq!(integral_number(&Value::from(2.0)), Some(2));
    assert_eq!(integral_number(&Value::from(0)), Some(0));
    assert_eq!(integral_number(&Value::from(2.5)), None);
    assert_eq!(integral_number(&Value::from(-2)), None);
    assert_eq!(integral_number(&Value::from(MAX_EXACT_INTEGER)), Some(MAX_EXACT_INTEGER));
    assert_eq!(integral_number(&Value::from(MAX_EXACT_INTEGER + 2)), None);
    assert_eq!(integral_number(&Value::String("2".into())), None);
    assert_eq!(integral_number(&Value::Null), None);
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
