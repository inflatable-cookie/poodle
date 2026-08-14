//! Tabs specimen — corpus projection (g14.004).
//!
//! Groups, captions, axes, ordered item collections, and fixtures come from
//! the shared case corpus. GPUI contributes only the native renderer.

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::conformance_support::{enum_values, tabs_spec_from_fixture, TABS_CASES, TABS_INTERFACE};
use crate::node_compat::Eyebrow;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::{tabs_with_panel, TabsHandlers};
use poodle_specs::{
    ActiveEdge, ActiveFill, ControlDensity, ControlSize, EyebrowSpec, TabDefinition, TabVariant,
    TabsSpec,
};
use serde_json::Value;
use std::sync::Arc;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    render_corpus_groups(state)
}

fn render_corpus_groups(state: &AppState) -> Div {
    let theme = &state.theme;
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
            state,
            theme,
            case.get("id")
                .and_then(Value::as_str)
                .unwrap_or("tabs/case"),
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
                    state,
                    theme,
                    &format!(
                        "{}:{value}",
                        case.get("id")
                            .and_then(Value::as_str)
                            .unwrap_or("tabs/case")
                    ),
                    &format!("{caption} · {value}"),
                    tabs_spec_from_fixture(&expanded),
                    panel_text(&expanded),
                ));
            }
        }
    }
    let residual_items = || {
        vec![
            TabDefinition::new("editor", "Editor").with_icon("code"),
            TabDefinition::new("preview", "Preview")
                .with_icon("eye")
                .with_count(12),
            TabDefinition::new("terminal", "Terminal")
                .with_icon("terminal")
                .with_closable(true),
        ]
    };
    root = root
        .child(div().h(px(8.0)))
        .child(group_section(
            theme,
            "Residual visual and operator coverage",
        ))
        .child(row(
            state,
            theme,
            "residual:closable",
            "Closable, reorderable, icons and counts",
            TabsSpec::new(residual_items())
                .with_variant(TabVariant::Card)
                .with_active_edge(ActiveEdge::Outline)
                .with_active_fill(ActiveFill::Solid)
                .with_reorderable(true),
            String::new(),
        ))
        .child(row(
            state,
            theme,
            "residual:panel",
            "Full-width block with panel",
            TabsSpec::new(residual_items())
                .with_variant(TabVariant::Block)
                .with_active_edge(ActiveEdge::Underline)
                .with_active_fill(ActiveFill::None)
                .with_full_width(true),
            "Surface content area".to_owned(),
        ))
        .child(row(
            state,
            theme,
            "residual:scale",
            "Large comfortable pill",
            TabsSpec::new(residual_items())
                .with_variant(TabVariant::Pill)
                .with_size(ControlSize::Lg)
                .with_density(ControlDensity::Comfortable),
            String::new(),
        ));
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

fn row(
    state: &AppState,
    theme: &GpuiThemeProvider,
    key: &str,
    caption: &str,
    mut spec: TabsSpec,
    panel: String,
) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let state_key = format!("tabs-corpus:{key}");
    if let Some(value) = state.specimens.text.get(&state_key) {
        spec.value = Some(value.clone());
    }
    let handlers = projected_tabs_handlers(state, state_key);
    let node = tabs_with_panel(&spec, theme, handlers, Node::text(panel));
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

fn projected_tabs_handlers(state: &AppState, state_key: String) -> TabsHandlers {
    let events = Arc::clone(&state.node_events);
    let on_change = Arc::new(move |value: &str| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: state_key.clone(),
            value: value.to_owned(),
        });
    });
    TabsHandlers {
        on_change: Some(on_change),
        ..TabsHandlers::default()
    }
}
