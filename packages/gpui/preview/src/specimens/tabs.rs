//! Tabs specimen — corpus projection (g14.004).
//!
//! Groups, captions, axes, ordered item collections, and fixtures come from
//! the shared case corpus. GPUI contributes only the native renderer.

use crate::app_state::AppState;
use crate::conformance_support::{enum_values, tabs_spec_from_fixture, TABS_CASES, TABS_INTERFACE};
use crate::node_compat::Eyebrow;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::{tabs_with_panel, TabsHandlers};
use poodle_specs::{EyebrowSpec, TabsSpec};
use serde_json::Value;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    render_corpus_groups(&state.theme)
}

fn render_corpus_groups(theme: &GpuiThemeProvider) -> Div {
    let cases: Value = serde_json::from_str(TABS_CASES).expect("committed corpus parses");
    let interface: Value =
        serde_json::from_str(TABS_INTERFACE).expect("committed interface parses");
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
            .and_then(|value| value.get("group"))
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
            .and_then(|value| value.get("caption"))
            .and_then(Value::as_str)
            .unwrap_or("");
        let fixture = case
            .get("fixture")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));
        root = root.child(row(
            theme,
            caption,
            tabs_spec_from_fixture(&fixture),
            panel_text(&fixture),
        ));

        let axes = case
            .get("specimen")
            .and_then(|value| value.get("axes"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let props = fixture
            .get("props")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));
        for axis in axes.iter().filter_map(Value::as_str) {
            if axis != "size" && axis != "density" || props.get(axis).is_some() {
                continue;
            }
            for value in enum_values(&interface, axis) {
                let mut expanded = fixture.clone();
                expanded["props"][axis] = serde_json::json!(value);
                root = root.child(row(
                    theme,
                    &format!("{caption} · {value}"),
                    tabs_spec_from_fixture(&expanded),
                    panel_text(&expanded),
                ));
            }
        }
    }
    root
}

fn panel_text(fixture: &Value) -> String {
    fixture
        .get("regions")
        .and_then(|value| value.get("panel"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
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

fn row(theme: &GpuiThemeProvider, caption: &str, spec: TabsSpec, panel: String) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let node = tabs_with_panel(&spec, theme, TabsHandlers::default(), Node::text(panel));
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
                .child(caption.to_owned()),
        )
        .child(
            div()
                .w(px(360.0))
                .child(poodle_gpui_node_backend::to_gpui(&node)),
        )
}
