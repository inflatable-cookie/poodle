//! Conformance specimen support (spec 066, g14.001): the fixture → spec
//! adapter shared by the conformance runner bin and the Button specimen
//! page. Pure — no crate-internal imports, so the bin can pull it in via
//! `#[path]`. Both canonical JSON fixtures are included directly from the
//! TypeScript authority's checked output — never copied or restated here.

use poodle_node::Node;
use poodle_specs::{
    ActiveEdge, ActiveFill, ButtonSpec, ButtonTone, ButtonVariant, ControlDensity, ControlSize,
    Orientation, OverlayPlacement, PopoverInitialFocus, PopoverSpec, PopoverSurfaceWidth,
    RangeSliderSpec, SemanticControlSizeRole, SliderPolarity, SliderVariant, TabActivationMode,
    TabDefinition, TabVariant, TabsSpec,
};
use serde_json::Value;

// The canonical fixtures: the TypeScript authority's serialized output,
// gated byte-exact by `conformance:serialize-check`. No copies — every
// consumer reads the same bytes.
pub const CASES: &str = include_str!("../../../codegen/fixtures/conformance/button-cases.json");
pub const INTERFACE: &str =
    include_str!("../../../codegen/fixtures/conformance/button-interface.json");
pub const RANGE_SLIDER_CASES: &str =
    include_str!("../../../codegen/fixtures/conformance/range-slider-cases.json");
pub const RANGE_SLIDER_INTERFACE: &str =
    include_str!("../../../codegen/fixtures/conformance/range-slider-interface.json");
pub const TABS_CASES: &str = include_str!("../../../codegen/fixtures/conformance/tabs-cases.json");
pub const TABS_INTERFACE: &str =
    include_str!("../../../codegen/fixtures/conformance/tabs-interface.json");
pub const POPOVER_CASES: &str = include_str!("../../../codegen/fixtures/conformance/popover-cases.json");
pub const POPOVER_INTERFACE: &str =
    include_str!("../../../codegen/fixtures/conformance/popover-interface.json");

/// Fixture → PopoverSpec adapter (g14.005). All 12 placements, the numeric
/// offset, the guard, the focus strategies, and the surface width strategy
/// map mechanically; the web-only extension prop never appears here.
pub fn popover_spec_from_fixture(fixture: &Value) -> PopoverSpec {
    let props = fixture
        .get("props")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let mut spec = PopoverSpec::new();
    if let Some(open) = props.get("open").and_then(Value::as_bool) {
        spec = spec.with_open(open);
    }
    if let Some(default_open) = props.get("defaultOpen").and_then(Value::as_bool) {
        spec = spec.with_default_open(default_open);
    }
    if let Some(placement) = props.get("placement").and_then(Value::as_str) {
        spec = spec.with_placement(match placement {
            "top" => OverlayPlacement::Top,
            "top-start" => OverlayPlacement::TopStart,
            "top-end" => OverlayPlacement::TopEnd,
            "bottom" => OverlayPlacement::Bottom,
            "bottom-end" => OverlayPlacement::BottomEnd,
            "left" => OverlayPlacement::Left,
            "left-start" => OverlayPlacement::LeftStart,
            "left-end" => OverlayPlacement::LeftEnd,
            "right" => OverlayPlacement::Right,
            "right-start" => OverlayPlacement::RightStart,
            "right-end" => OverlayPlacement::RightEnd,
            _ => OverlayPlacement::BottomStart,
        });
    }
    if let Some(offset) = props.get("offset").and_then(Value::as_f64) {
        spec = spec.with_offset(offset as f32);
    }
    if let Some(dismiss) = props.get("dismissOnOutsideInteract").and_then(Value::as_bool) {
        spec = spec.with_dismiss_on_outside_interact(dismiss);
    }
    if let Some(strategy) = props.get("initialFocus").and_then(Value::as_str) {
        spec = spec.with_initial_focus(match strategy {
            "content" => PopoverInitialFocus::Content,
            "none" => PopoverInitialFocus::None,
            _ => PopoverInitialFocus::FirstFocusable,
        });
    }
    if let Some(label) = props.get("ariaLabel").and_then(Value::as_str) {
        spec = spec.with_aria_label(label);
    }
    if let Some(block) = props.get("block").and_then(Value::as_bool) {
        spec = spec.with_block(block);
    }
    if let Some(disabled) = props.get("disabled").and_then(Value::as_bool) {
        spec = spec.with_disabled(disabled);
    }
    if let Some(width) = props.get("surfaceWidth").and_then(Value::as_str) {
        spec = spec.with_surface_width(if width == "trigger" {
            PopoverSurfaceWidth::Trigger
        } else {
            PopoverSurfaceWidth::Content
        });
    }
    if let Some(min_width) = props.get("surfaceMinWidth").and_then(Value::as_str) {
        spec = spec.with_surface_min_width(poodle_specs::Dimension::new(min_width));
    }
    if let Some(max_width) = props.get("surfaceMaxWidth").and_then(Value::as_str) {
        spec = spec.with_surface_max_width(poodle_specs::Dimension::new(max_width));
    }
    spec
}

/// Builds the popover content node from the fixture's `children` region.
/// The region may carry inline `<button>label</button>` markup (the focus
/// corpus's focusable content convention); plain text becomes a text node.
/// Focusable buttons get stable ids so the real backend focus path can
/// target them.
pub fn popover_content_from_fixture(fixture: &Value, instance_id: &str) -> Option<Node> {
    let regions = fixture
        .get("regions")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let raw = regions.get("children").and_then(Value::as_str).unwrap_or_default();
    let focusables = fixture
        .pointer("/host/focusables")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if raw.is_empty() && focusables.is_empty() {
        return None;
    }
    let mut content = Node::container();
    if !raw.is_empty() {
        content = content.child(Node::text(raw));
    }
    for (index, label) in focusables.into_iter().enumerate() {
        let mut button = Node::button(label.clone());
        button.interaction.focusable = true;
        button.id = Some(format!("{instance_id}:popover-content-focusable-{index}"));
        button.a11y.label = Some(label);
        content = content.child(button);
    }
    Some(content)
}

/// The trigger node for the composition, from the fixture's `trigger` region.
pub fn popover_trigger_from_fixture(fixture: &Value) -> Option<Node> {
    let regions = fixture
        .get("regions")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let raw = regions.get("trigger").and_then(Value::as_str).unwrap_or_default();
    if raw.is_empty() {
        None
    } else {
        Some(Node::text(raw))
    }
}

/// The fixture → spec adapter (the harness's mount step for Button).
pub fn spec_from_fixture(fixture: &Value) -> ButtonSpec {
    let props = fixture
        .get("props")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let regions = fixture
        .get("regions")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));

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
    let props = fixture
        .get("props")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
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
    spec.center_value = props.get("centerValue").and_then(Value::as_f64);
    spec.lower_value_text = props
        .get("lowerValueText")
        .and_then(Value::as_str)
        .map(str::to_owned);
    spec.upper_value_text = props
        .get("upperValueText")
        .and_then(Value::as_str)
        .map(str::to_owned);
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
    if let Some(v) = props.get("sizeRole").and_then(Value::as_str) {
        spec.size_role = match v {
            "chrome" => SemanticControlSizeRole::Chrome,
            "prominent" => SemanticControlSizeRole::Prominent,
            _ => SemanticControlSizeRole::Control,
        };
    }
    spec
}

/// Fixture → TabsSpec adapter. Item order and identity come only from the
/// shared corpus; no preview-local item model exists.
pub fn tabs_spec_from_fixture(fixture: &Value) -> TabsSpec {
    let props = fixture
        .get("props")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let tabs = props
        .get("items")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    let value = item.get("value")?.as_str()?;
                    let label = item.get("label")?.as_str()?;
                    let mut tab = TabDefinition::new(value, label);
                    if let Some(icon) = item.get("icon").and_then(Value::as_str) {
                        tab = tab.with_icon(icon);
                    }
                    if let Some(disabled) = item.get("disabled").and_then(Value::as_bool) {
                        tab = tab.with_disabled(disabled);
                    }
                    if let Some(closable) = item.get("closable").and_then(Value::as_bool) {
                        tab = tab.with_closable(closable);
                    }
                    if let Some(count) = item.get("count").and_then(Value::as_u64) {
                        tab = tab.with_count(count as u32);
                    }
                    Some(tab)
                })
                .collect()
        })
        .unwrap_or_default();
    let mut spec = TabsSpec::new(tabs);
    if let Some(value) = props.get("value").and_then(Value::as_str) {
        spec = spec.with_value(value);
    }
    if let Some(value) = props.get("variant").and_then(Value::as_str) {
        spec = spec.with_variant(match value {
            "pill" => TabVariant::Pill,
            "block" => TabVariant::Block,
            _ => TabVariant::Card,
        });
    }
    if let Some(value) = props.get("activeEdge").and_then(Value::as_str) {
        spec = spec.with_active_edge(match value {
            "outline" => ActiveEdge::Outline,
            "underline" => ActiveEdge::Underline,
            _ => ActiveEdge::None,
        });
    }
    if let Some(value) = props.get("activeFill").and_then(Value::as_str) {
        spec = spec.with_active_fill(match value {
            "none" => ActiveFill::None,
            "solid" => ActiveFill::Solid,
            _ => ActiveFill::Tint,
        });
    }
    if props.get("orientation").and_then(Value::as_str) == Some("vertical") {
        spec = spec.with_orientation(Orientation::Vertical);
    }
    if props.get("activationMode").and_then(Value::as_str) == Some("manual") {
        spec = spec.with_activation_mode(TabActivationMode::Manual);
    }
    if let Some(value) = props.get("bordered").and_then(Value::as_bool) {
        spec = spec.with_bordered(value);
    }
    if let Some(value) = props.get("fullWidth").and_then(Value::as_bool) {
        spec = spec.with_full_width(value);
    }
    if let Some(value) = props.get("reorderable").and_then(Value::as_bool) {
        spec = spec.with_reorderable(value);
    }
    if let Some(value) = props.get("ariaLabel").and_then(Value::as_str) {
        spec = spec.with_aria_label(value);
    }
    if let Some(value) = props.get("size").and_then(Value::as_str) {
        spec = spec.with_size(match value {
            "xs" => ControlSize::Xs,
            "sm" => ControlSize::Sm,
            "lg" => ControlSize::Lg,
            "xl" => ControlSize::Xl,
            _ => ControlSize::Md,
        });
    }
    if let Some(value) = props.get("sizeRole").and_then(Value::as_str) {
        spec = spec.with_size_role(match value {
            "control" => SemanticControlSizeRole::Control,
            "prominent" => SemanticControlSizeRole::Prominent,
            _ => SemanticControlSizeRole::Chrome,
        });
    }
    if let Some(value) = props.get("density").and_then(Value::as_str) {
        spec = spec.with_density(match value {
            "compact" => ControlDensity::Compact,
            "comfortable" => ControlDensity::Comfortable,
            _ => ControlDensity::Default,
        });
    }
    poodle_specs::generated::tabs::TabsPortableSpec {
        tabs: spec.tabs,
        value: spec.value,
        variant: spec.variant,
        active_edge: spec.active_edge,
        active_fill: spec.active_fill,
        orientation: spec.orientation,
        activation_mode: spec.activation_mode,
        is_bordered: spec.is_bordered,
        is_full_width: spec.is_full_width,
        is_reorderable: spec.is_reorderable,
        aria_label: spec.aria_label,
        size: Some(spec.size),
        size_role: spec.size_role,
        density: Some(spec.density),
    }
    .into()
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
