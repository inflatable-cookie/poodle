//! Conformance specimen support (spec 066, g14.001): the fixture → spec
//! adapter and the corpus-driven specimen projection shared by the
//! conformance runner bin and the Button specimen page. The corpus JSON is
//! the generated copy of the TypeScript authority — never restated here.

use crate::compat::js_button;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
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
fn enum_values(interface: &Value, prop_name: &str) -> Vec<String> {
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

/// Renders the corpus's Button groups as specimen rows: one row per case
/// (plus size/density axis expansion), caption first, button from fixture.
pub fn render_corpus_groups(theme: &JetstreamThemeProvider) -> El {
    let cases: Value = serde_json::from_str(CASES).expect("committed corpus parses");
    let interface: Value = serde_json::from_str(INTERFACE).expect("committed interface parses");
    let case_list = cases
        .get("cases")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let secondary = resolve_color(theme, "color.text.secondary");

    let mut root = div().flex_col().gap(24.0);
    let mut current_group: Option<String> = None;
    for case in &case_list {
        let group = case
            .get("specimen")
            .and_then(|s| s.get("group"))
            .and_then(Value::as_str)
            .unwrap_or("Other")
            .to_owned();
        if current_group.as_deref() != Some(group.as_str()) {
            if current_group.is_some() {
                root = root.child(div().h(8.0));
            }
            root = root.child(group_section(group.as_str(), secondary, div()));
            current_group = Some(group);
        }
        let caption = case
            .get("specimen")
            .and_then(|s| s.get("caption"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned();
        let fixture = case.get("fixture").cloned().unwrap_or_else(|| serde_json::json!({}));
        let axes = case
            .get("specimen")
            .and_then(|s| s.get("axes"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        root = root.child(row(
            &caption,
            spec_from_fixture(&fixture),
            theme,
        ));

        let props = fixture.get("props").cloned().unwrap_or_else(|| serde_json::json!({}));
        let props_fixed = |axis: &str| props.get(axis).is_some();
        for axis in axes.iter().filter_map(Value::as_str) {
            if axis != "size" && axis != "density" {
                continue;
            }
            if props_fixed(axis) {
                continue;
            }
            for value in enum_values(&interface, axis) {
                let mut expanded = fixture.clone();
                expanded["props"][axis] = serde_json::json!(value);
                root = root.child(row(
                    &format!("{caption} · {value}"),
                    spec_from_fixture(&expanded),
                    theme,
                ));
            }
        }
    }
    root
}

fn row(caption: &str, spec: ButtonSpec, theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    div()
        .flex_row()
        .gap(12.0)
        .items_center()
        .child(label(caption).text_color(secondary).text_size(12.0).w(200.0))
        .child(js_button(&spec, theme))
}

/// The group chrome other specimens share.
fn group_section(title: &str, color: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(color).text_size(13.0))
        .child(content)
}
