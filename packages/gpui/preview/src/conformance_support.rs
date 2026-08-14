//! Conformance specimen support (spec 066, g14.001): the fixture → spec
//! adapter shared by the conformance runner bin and the Button specimen
//! page. Pure — no crate-internal imports, so the bin can pull it in via
//! `#[path]`. Both canonical JSON fixtures are included directly from the
//! TypeScript authority's checked output — never copied or restated here.

use poodle_specs::{
    ButtonSpec, ButtonTone, ButtonVariant, ControlDensity, ControlSize, Orientation,
    RangeSliderSpec, SliderPolarity, SliderVariant,
};
use serde_json::Value;

// The canonical fixtures: the TypeScript authority's serialized output,
// gated byte-exact by `conformance:serialize-check`. No copies — every
// consumer reads the same bytes.
pub const CASES: &str = include_str!("../../../codegen/fixtures/conformance/button-cases.json");
pub const INTERFACE: &str = include_str!("../../../codegen/fixtures/conformance/button-interface.json");
pub const RANGE_SLIDER_CASES: &str =
    include_str!("../../../codegen/fixtures/conformance/range-slider-cases.json");
pub const RANGE_SLIDER_INTERFACE: &str =
    include_str!("../../../codegen/fixtures/conformance/range-slider-interface.json");

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
            "defaultPressed" => {
                if let Some(pressed) = value.as_bool() {
                    spec = spec.with_default_pressed(pressed);
                }
            }
            _ => {}
        }
    }
    spec
}

/// The fixture → spec adapter for RangeSlider (g14.003).
pub fn range_slider_spec_from_fixture(fixture: &Value) -> RangeSliderSpec {
    let props = fixture.get("props").cloned().unwrap_or_else(|| serde_json::json!({}));
    let mut spec = RangeSliderSpec::default();
    if let Some(pair) = props.get("value").and_then(Value::as_array) {
        if pair.len() == 2 {
            spec.low = pair[0].as_f64().unwrap_or(spec.low);
            spec.high = pair[1].as_f64().unwrap_or(spec.high);
        }
    }
    if let Some(v) = props.get("min").and_then(Value::as_f64) {
        spec.min = v;
    }
    if let Some(v) = props.get("max").and_then(Value::as_f64) {
        spec.max = v;
    }
    if let Some(v) = props.get("step").and_then(Value::as_f64) {
        spec.step = v;
    }
    if let Some(v) = props.get("disabled").and_then(Value::as_bool) {
        spec.is_disabled = v;
    }
    if let Some(v) = props.get("ariaLabel").and_then(Value::as_str) {
        spec.aria_label = Some(v.to_owned());
    }
    if let Some(v) = props.get("orientation").and_then(Value::as_str) {
        spec.orientation = match v {
            "vertical" => Orientation::Vertical,
            _ => Orientation::Horizontal,
        };
    }
    if let Some(v) = props.get("variant").and_then(Value::as_str) {
        spec.variant = match v {
            "embedded" => SliderVariant::Embedded,
            _ => SliderVariant::Standard,
        };
    }
    if let Some(v) = props.get("polarity").and_then(Value::as_str) {
        spec.polarity = match v {
            "bipolar" => SliderPolarity::Bipolar,
            _ => SliderPolarity::Unipolar,
        };
    }
    if let Some(v) = props.get("size").and_then(Value::as_str) {
        spec.size = match v {
            "xs" => ControlSize::Xs,
            "sm" => ControlSize::Sm,
            "lg" => ControlSize::Lg,
            "xl" => ControlSize::Xl,
            _ => ControlSize::Md,
        };
    }
    if let Some(v) = props.get("density").and_then(Value::as_str) {
        spec.density = match v {
            "compact" => ControlDensity::Compact,
            "comfortable" => ControlDensity::Comfortable,
            _ => ControlDensity::Default,
        };
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
