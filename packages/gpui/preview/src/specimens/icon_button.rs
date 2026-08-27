//! Icon Button specimen — migrated to the node tier in g12.019 Batch B.
//!
//! Every IconButton below renders through the node tier:
//! `poodle_render::icon_button` (`Spec + Context → Node`) interpreted by
//! `poodle_gpui_node_backend::to_gpui`. The old hand-written
//! `poodle_gpui_components::IconButton` no longer renders this specimen;
//! everything around the buttons (layout, Eyebrow headings, captions) is
//! unchanged.
//!
//! Node interaction closures are context-free (`Arc<dyn Fn() + Send + Sync>`),
//! so instead of `cx.listener` the handlers push `NodeSpecimenEvent`s onto a
//! queue the next render drains into specimen state (see `app_state.rs`).

use crate::node_compat::{Eyebrow, IconButton};
use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

use poodle_specs::{ButtonTone, ButtonVariant, EyebrowSpec, IconButtonSpec};

/// Production compatibility wrapper. Interactive examples attach command
/// and pressed-change handlers; static instances stay handler-free.
fn node_icon_button(
    spec: IconButtonSpec,
    state: &AppState,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> IconButton {
    let mut button = IconButton::from_spec(spec, &state.theme);
    if let Some(handler) = on_click {
        button = button.on_click(handler);
    }
    button
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let last_clicked = state
        .specimens
        .text
        .get("icon-btn-last")
        .cloned()
        .unwrap_or_default();

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Variants ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Variants"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_icon("plus")
                                .with_aria_label("Add"),
                            state,
                            Some({
                                let events = state.node_events.clone();
                                Arc::new(move || {
                                    events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                                        key: "icon-btn-last".to_string(),
                                        value: "Add".to_string(),
                                    });
                                })
                            }),
                        ))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_icon("settings")
                                .with_aria_label("Settings"),
                            state,
                            Some({
                                let events = state.node_events.clone();
                                Arc::new(move || {
                                    events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                                        key: "icon-btn-last".to_string(),
                                        value: "Settings".to_string(),
                                    });
                                })
                            }),
                        ))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_icon("x")
                                .with_aria_label("Close"),
                            state,
                            Some({
                                let events = state.node_events.clone();
                                Arc::new(move || {
                                    events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                                        key: "icon-btn-last".to_string(),
                                        value: "Close".to_string(),
                                    });
                                })
                            }),
                        )),
                ),
        )
        // --- Danger tone ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Danger tone"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_tone(ButtonTone::Danger)
                                .with_icon("trash-2")
                                .with_aria_label("Delete"),
                            state,
                            None,
                        ))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_tone(ButtonTone::Danger)
                                .with_icon("trash-2")
                                .with_aria_label("Delete"),
                            state,
                            None,
                        ))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_tone(ButtonTone::Danger)
                                .with_icon("trash-2")
                                .with_aria_label("Delete"),
                            state,
                            None,
                        )),
                ),
        )
        // --- Success tone ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Success tone"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_tone(ButtonTone::Success)
                                .with_icon("check")
                                .with_aria_label("Approve"),
                            state,
                            None,
                        ))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_tone(ButtonTone::Success)
                                .with_icon("check")
                                .with_aria_label("Approve"),
                            state,
                            None,
                        ))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_tone(ButtonTone::Success)
                                .with_icon("check")
                                .with_aria_label("Approve"),
                            state,
                            None,
                        )),
                ),
        )
        // --- Toggle (text editor toolbar) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Toggle (text editor toolbar)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .items_center()
                        .child(
                            node_icon_button(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_icon("bold")
                                    .with_pressed(state.specimens.is_on("icon-btn-bold"))
                                    .with_aria_label("Bold"),
                                state,
                                None,
                            )
                            .on_pressed_change({
                                let events = state.node_events.clone();
                                Arc::new(move |pressed| {
                                    events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                                        key: "icon-btn-bold".to_string(),
                                        value: pressed,
                                    });
                                })
                            }),
                        )
                        .child(
                            node_icon_button(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_icon("italic")
                                    .with_pressed(state.specimens.is_on("icon-btn-italic"))
                                    .with_aria_label("Italic"),
                                state,
                                None,
                            )
                            .on_pressed_change({
                                let events = state.node_events.clone();
                                Arc::new(move |pressed| {
                                    events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                                        key: "icon-btn-italic".to_string(),
                                        value: pressed,
                                    });
                                })
                            }),
                        )
                        .child(
                            node_icon_button(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_icon("underline")
                                    .with_pressed(state.specimens.is_on("icon-btn-underline"))
                                    .with_aria_label("Underline"),
                                state,
                                None,
                            )
                            .on_pressed_change({
                                let events = state.node_events.clone();
                                Arc::new(move |pressed| {
                                    events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                                        key: "icon-btn-underline".to_string(),
                                        value: pressed,
                                    });
                                })
                            }),
                        ),
                ),
        )
        // --- States ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("States"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .items_center()
                        .child(
                            node_icon_button(
                                IconButtonSpec::new()
                                    .with_icon("map-pin")
                                    .with_pressed(state.specimens.is_on("icon-btn-pinned"))
                                    .with_aria_label("Pin"),
                                state,
                                Some({
                                    let events = state.node_events.clone();
                                    Arc::new(move || {
                                        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                                            key: "icon-btn-last".to_string(),
                                            value: "Pin toggled".to_string(),
                                        });
                                    })
                                }),
                            )
                            .on_pressed_change({
                                let events = state.node_events.clone();
                                Arc::new(move |pressed| {
                                    events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                                        key: "icon-btn-pinned".to_string(),
                                        value: pressed,
                                    });
                                })
                            }),
                        )
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_icon("settings")
                                .with_disabled(true)
                                .with_aria_label("Settings"),
                            state,
                            None,
                        ))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_icon("loader")
                                .with_loading(true)
                                .with_aria_label("Loading"),
                            state,
                            None,
                        )),
                ),
        )
        // --- String name (built-in internals) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("String name (built-in internals)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_icon("plus")
                                .with_aria_label("Add"),
                            state,
                            None,
                        ))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_icon("search")
                                .with_aria_label("Search"),
                            state,
                            None,
                        ))
                        .child(node_icon_button(
                            IconButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_icon("x")
                                .with_aria_label("Close"),
                            state,
                            None,
                        )),
                ),
        )
        // --- Click feedback ---
        .when(!last_clicked.is_empty(), |d| {
            d.child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child(format!("Last action: {}", last_clicked)),
            )
        })
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "icon-button",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                IconButton::from_spec(
                    IconButtonSpec::new()
                        .with_icon("star")
                        .with_size(size)
                        .with_aria_label("Star"),
                    theme,
                )
                .into_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                IconButton::from_spec(
                    IconButtonSpec::new()
                        .with_icon("star")
                        .with_density(density)
                        .with_aria_label("Star"),
                    theme,
                )
                .into_element()
            }),
    )
}
