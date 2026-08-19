//! SplitButton specimen — migrated to the node tier (g12.019 Batch B).
//!
//! Every SplitButton below renders through the node tier:
//! `poodle_render::split_button` (`Spec + Theme → Node`) interpreted by
//! `poodle_gpui_node_backend::to_gpui`. The old hand-written
//! `poodle_gpui_components::SplitButton` no longer renders this specimen;
//! everything around the split buttons (layout, Eyebrow headings, captions)
//! is unchanged.
//!
//! Node interaction closures are context-free (`Arc<dyn Fn() + Send + Sync>`),
//! so instead of `cx.listener` the handlers push `NodeSpecimenEvent`s onto a
//! queue the next render drains into specimen state (see `app_state.rs`).
//! Menu open/close stays host-owned; the "Dropdown menu open" section
//! expresses it through the spec's `is_open` field.

use crate::node_compat::Eyebrow;
use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

use poodle_render::{split_button, SplitButtonHandlers};
use poodle_specs::{ButtonTone, ButtonVariant, EyebrowSpec, SplitButtonSpec, SplitMenuItem};

/// Build a node-tier SplitButton with the given handlers.
fn node_split_button(
    spec: SplitButtonSpec,
    state: &AppState,
    handlers: SplitButtonHandlers,
) -> AnyElement {
    let node = split_button(&spec, &state.theme, handlers);
    poodle_gpui_node_backend::to_gpui(&node)
}

/// A node-tier SplitButton with no handlers (matrix / open menu / loading /
/// disabled / sizes / densities).
fn node_split_button_static(spec: SplitButtonSpec, state: &AppState) -> AnyElement {
    node_split_button(spec, state, SplitButtonHandlers::default())
}

/// A click/dropdown handler that records `value` under the shared
/// "split-btn-action" text key, mirroring the old specimen's listeners.
fn action_handler(state: &AppState, value: &'static str) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: "split-btn-action".to_string(),
            value: value.to_string(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let last_action = state
        .specimens
        .text
        .get("split-btn-action")
        .cloned()
        .unwrap_or_else(|| String::from("(none)"));

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Save split action ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Save split action"),
                    theme,
                ))
                .child(node_split_button(
                    SplitButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_label("Save")
                        .with_items(vec![
                            SplitMenuItem::action("save-draft", "Save as draft"),
                            SplitMenuItem::action("save-template", "Save as template"),
                            SplitMenuItem::Separator,
                            SplitMenuItem::action("discard", "Discard changes"),
                        ]),
                    state,
                    SplitButtonHandlers {
                        on_click: Some(action_handler(state, "Save")),
                        on_dropdown: Some(action_handler(state, "dropdown: toggle")),
                        on_action: Some({
                            let events = state.node_events.clone();
                            Arc::new(move |value: &str| {
                                events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                                    key: "split-btn-action".to_string(),
                                    value: value.to_string(),
                                });
                            })
                        }),
                    },
                )),
        )
        // --- Secondary export ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Secondary export"),
                    theme,
                ))
                .child(node_split_button(
                    SplitButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Export")
                        .with_items(vec![
                            SplitMenuItem::action("csv", "Export as CSV"),
                            SplitMenuItem::action("json", "Export as JSON"),
                            SplitMenuItem::action("pdf", "Export as PDF"),
                        ]),
                    state,
                    SplitButtonHandlers {
                        on_click: Some(action_handler(state, "Export")),
                        on_dropdown: Some(action_handler(state, "dropdown: toggle")),
                        on_action: None,
                    },
                )),
        )
        // --- Intent tones ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Intent tones"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(12.0))
                        .flex_wrap()
                        .child(node_split_button(
                            SplitButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_tone(ButtonTone::Danger)
                                .with_label("Delete")
                                .with_items(vec![
                                    SplitMenuItem::action("delete-selected", "Delete selected"),
                                    SplitMenuItem::action("delete-all", "Delete all"),
                                ]),
                            state,
                            SplitButtonHandlers {
                                on_click: Some(action_handler(state, "Delete")),
                                on_dropdown: None,
                                on_action: None,
                            },
                        ))
                        .child(node_split_button(
                            SplitButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_tone(ButtonTone::Success)
                                .with_label("Publish")
                                .with_items(vec![
                                    SplitMenuItem::action("publish-now", "Publish now"),
                                    SplitMenuItem::action("schedule-publish", "Schedule"),
                                ]),
                            state,
                            SplitButtonHandlers {
                                on_click: Some(action_handler(state, "Publish")),
                                on_dropdown: None,
                                on_action: None,
                            },
                        ))
                        .child(node_split_button(
                            SplitButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_tone(ButtonTone::Warning)
                                .with_label("Archive")
                                .with_items(vec![
                                    SplitMenuItem::action("archive-selected", "Archive selected"),
                                    SplitMenuItem::action("archive-all", "Archive all"),
                                ]),
                            state,
                            SplitButtonHandlers {
                                on_click: Some(action_handler(state, "Archive")),
                                on_dropdown: None,
                                on_action: None,
                            },
                        )),
                ),
        )
        // --- Loading and disabled states ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Loading and disabled states"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(12.0))
                        .flex_wrap()
                        .child(node_split_button_static(
                            SplitButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_label("Saving...")
                                .with_loading(true),
                            state,
                        ))
                        .child(node_split_button_static(
                            SplitButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Save")
                                .with_disabled(true),
                            state,
                        )),
                ),
        )
        .child(
            div()
                .text_xs()
                .text_color(color_to_hsla(text_secondary))
                .child(format!("Last action: {}", last_action)),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "split-button",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                let node = split_button(
                    &SplitButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_size(size)
                        .with_label("Action")
                        .with_items(vec![
                            SplitMenuItem::action("a", "Action A"),
                            SplitMenuItem::action("b", "Action B"),
                        ]),
                    theme,
                    SplitButtonHandlers::default(),
                );
                poodle_gpui_node_backend::to_gpui(&node)
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                let node = split_button(
                    &SplitButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Action")
                        .with_items(vec![
                            SplitMenuItem::action("a", "Action A"),
                            SplitMenuItem::action("b", "Action B"),
                        ])
                        .with_density(density),
                    theme,
                    SplitButtonHandlers::default(),
                );
                poodle_gpui_node_backend::to_gpui(&node)
            }),
    )
}
