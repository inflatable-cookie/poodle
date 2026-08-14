//! Conformance specimen support (spec 066, g14.001): the fixture → spec
//! adapter shared by the conformance runner bin and the Button specimen
//! page. Pure — no crate-internal imports, so the bin can pull it in via
//! `#[path]`. The corpus JSON is the generated copy of the TypeScript
//! authority — never restated here.

use poodle_specs::{
    ButtonSpec, ButtonTone, ButtonVariant, ControlDensity, ControlSize,
};
use serde_json::Value;

pub const CASES: &str = include_str!("./generated/conformance/button-cases.json");
pub const INTERFACE: &str = include_str!("./generated/conformance/button-interface.json");

/// The fixture → spec adapter (the harness's mount step for Button).
pub fn spec_from_fixture(fixture: &Value) -> ButtonSpec {
    let props = fixture.get("props").cloned().unwrap_or_else(|| serde_json::json!({}));
    let regions = fixture.get("regions").cloned().unwrap_or_else(|| serde_json::json!({}));

    let mut spec = ButtonSpec::new();
    if let Some(label) = regions.get("label").and_then(Value::as_str) {
        spec = spec.with_label(label);
    }
    if let Some(icon) = regions.get("leading").and_then(Value::as_str) {
        spec = spec.with_leading_icon(icon);
    }
    if let Some(icon) = regions.get("trailing").and_then(Value::as_str) {
        spec = spec.with_trailing_icon(icon);
    }
    let Some(props_map) = props.as_object() else {
        return spec;
    };
    for (key, value) in props_map {
        match key.as_str() {
            "variant" => {
                if let Some(v) = value.as_str() {
                    spec = spec.with_variant(match v {
                        "primary" => ButtonVariant::Primary,
                        "ghost" => ButtonVariant::Ghost,
                        _ => ButtonVariant::Secondary,
                    });
                }
            }
            "tone" => {
                if let Some(v) = value.as_str() {
                    spec = spec.with_tone(match v {
                        "danger" => ButtonTone::Danger,
                        "success" => ButtonTone::Success,
                        "warning" => ButtonTone::Warning,
                        _ => ButtonTone::Default,
                    });
                }
            }
            "size" => {
                if let Some(v) = value.as_str() {
                    spec = spec.with_size(match v {
                        "xs" => ControlSize::Xs,
                        "sm" => ControlSize::Sm,
                        "lg" => ControlSize::Lg,
                        "xl" => ControlSize::Xl,
                        _ => ControlSize::Md,
                    });
                }
            }
            "density" => {
                if let Some(v) = value.as_str() {
                    spec = spec.with_density(match v {
                        "compact" => ControlDensity::Compact,
                        "comfortable" => ControlDensity::Comfortable,
                        _ => ControlDensity::Default,
                    });
                }
            }
            "disabled" => {
                if value.as_bool() == Some(true) {
                    spec = spec.with_disabled(true);
                }
            }
            "loading" => {
                if value.as_bool() == Some(true) {
                    spec = spec.with_loading(true);
                }
            }
            "chevron" => {
                if value.as_bool() == Some(true) {
                    spec = spec.with_chevron(true);
                }
            }
            "pressed" => {
                if let Some(pressed) = value.as_bool() {
                    spec = spec.with_pressed(pressed);
                }
            }
            _ => {}
        }
    }
    spec
}

/// Enum values for an interface prop (axis expansion).
#[allow(dead_code)]
pub fn enum_values(interface: &Value, prop_name: &str) -> Vec<String> {
    interface
        .get("props")
        .and_then(Value::as_array)
        .and_then(|props| {
            props
                .iter()
                .find(|p| p.get("name").and_then(Value::as_str) == Some(prop_name))
        })
        .and_then(|prop| prop.get("type").and_then(|t| t.get("values")))
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}
