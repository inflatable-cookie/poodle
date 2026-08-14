//! RangeSlider specimen — corpus projection (g14.003).
//!
//! Groups, captions, axes, and fixtures come from the shared case corpus.
//! Every control renders through the node tier (`poodle_render::range_slider`
//! → `poodle_gpui_node_backend::to_gpui`).

use crate::app_state::AppState;
use crate::conformance_support::{
    enum_values, range_slider_spec_from_fixture, RANGE_SLIDER_CASES, RANGE_SLIDER_INTERFACE,
};
use crate::node_compat::Eyebrow;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_render::{range_slider, RangeSliderHandlers};
use poodle_specs::{EyebrowSpec, RangeSliderSpec};
use serde_json::Value;

fn node_range_slider_static(spec: RangeSliderSpec, theme: &GpuiThemeProvider) -> AnyElement {
    let node = range_slider(
        &spec,
        theme,
        RangeSliderHandlers {
            on_change: None,
            on_value_commit: None,
        },
    );
    poodle_gpui_node_backend::to_gpui(&node)
}

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    render_corpus_groups(&state.theme)
}

fn render_corpus_groups(theme: &GpuiThemeProvider) -> Div {
    let cases: Value =
        serde_json::from_str(RANGE_SLIDER_CASES).expect("committed corpus parses");
    let interface: Value =
        serde_json::from_str(RANGE_SLIDER_INTERFACE).expect("committed interface parses");
    let case_list = cases
        .get("cases")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let mut root = div().flex().flex_col().gap(px(24.0));
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
                root = root.child(div().h(px(8.0)));
            }
            root = root.child(group_section(theme, group.as_str()));
            current_group = Some(group);
        }
        let caption = case
            .get("specimen")
            .and_then(|s| s.get("caption"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned();
        let fixture = case
            .get("fixture")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));
        let axes = case
            .get("specimen")
            .and_then(|s| s.get("axes"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        root = root.child(row(
            theme,
            &caption,
            range_slider_spec_from_fixture(&fixture),
        ));

        let props = fixture
            .get("props")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));
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
                    theme,
                    &format!("{caption} · {value}"),
                    range_slider_spec_from_fixture(&expanded),
                ));
            }
        }
    }
    root
}

fn group_section(theme: &GpuiThemeProvider, title: &str) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(title),
            theme,
        ))
}

fn row(theme: &GpuiThemeProvider, caption: &str, spec: RangeSliderSpec) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.0))
        .child(
            div()
                .w(px(200.0))
                .text_color(crate::style_bridge::color_to_hsla(text_secondary))
                .text_size(px(12.0))
                .child(caption.to_string()),
        )
        .child(
            div()
                .w(px(220.0))
                .child(node_range_slider_static(spec, theme)),
        )
}
