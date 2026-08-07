//! Button specimen — migrated to the node tier (g12.019 Batch B).
//!
//! Every Button below renders through the node tier: `poodle_render::button`
//! (`Spec + Theme → Node`) interpreted by `poodle_gpui_node_backend::to_gpui`.
//! The old hand-written `poodle_gpui_components::Button` no longer renders
//! this specimen; everything around the buttons (layout, Eyebrow headings,
//! captions) is unchanged.
//!
//! Node interaction closures are context-free (`Arc<dyn Fn() + Send + Sync>`),
//! so instead of `cx.listener` the handlers push `NodeSpecimenEvent`s onto a
//! queue the next render drains into specimen state (see `app_state.rs`).

use crate::node_compat::Eyebrow;
use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

use poodle_render::button;
use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant, EyebrowSpec};

/// A node-tier Button with an optional click handler.
fn node_button(
    spec: ButtonSpec,
    state: &AppState,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> AnyElement {
    let node = button(&spec, &state.theme, on_click);
    poodle_gpui_node_backend::to_gpui(&node)
}

/// A node-tier Button with no handler (tones / icons / chevrons / states / sizes / densities).
fn node_button_static(spec: ButtonSpec, theme: &GpuiThemeProvider) -> AnyElement {
    let node = button(&spec, theme, None);
    poodle_gpui_node_backend::to_gpui(&node)
}

/// Click handler for the Variants row: bump the click counter and record
/// which variant was clicked. Mirrors the old specimen's `cx.listener`.
fn variant_click(state: &AppState, label: &'static str) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move || {
        let mut events = events.lock().unwrap();
        events.push(NodeSpecimenEvent::Increment("btn-clicks".to_string()));
        events.push(NodeSpecimenEvent::SetText {
            key: "btn-last-clicked".to_string(),
            value: label.to_string(),
        });
    })
}

/// Click handler that flips a boolean specimens key (toggle / disclosure rows).
fn toggle_click(state: &AppState, key: &'static str) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move || {
        events
            .lock()
            .unwrap()
            .push(NodeSpecimenEvent::Toggle(key.to_string()));
    })
}

/// Click handler that records a text specimens key (form overrides row).
fn set_text_click(
    state: &AppState,
    key: &'static str,
    value: &'static str,
) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: value.to_string(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let click_count = state.specimens.count("btn-clicks");
    let last_clicked = state
        .specimens
        .text
        .get("btn-last-clicked")
        .cloned()
        .unwrap_or_default();
    let disclosure_open = state.specimens.is_on("btn-disclosure-open");

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
                        .flex_wrap()
                        .child(node_button(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_label("Primary"),
                            state,
                            Some(variant_click(state, "Primary")),
                        ))
                        .child(node_button(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Secondary"),
                            state,
                            Some(variant_click(state, "Secondary")),
                        ))
                        .child(node_button(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_label("Ghost"),
                            state,
                            Some(variant_click(state, "Ghost")),
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
                        .flex_wrap()
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_tone(ButtonTone::Danger)
                                .with_label("Danger primary"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_tone(ButtonTone::Danger)
                                .with_label("Danger secondary"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_tone(ButtonTone::Danger)
                                .with_label("Danger ghost"),
                            theme,
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
                        .flex_wrap()
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_tone(ButtonTone::Success)
                                .with_label("Success primary"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_tone(ButtonTone::Success)
                                .with_label("Success secondary"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_tone(ButtonTone::Success)
                                .with_label("Success ghost"),
                            theme,
                        )),
                ),
        )
        // --- Warning tone ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Warning tone"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .flex_wrap()
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_tone(ButtonTone::Warning)
                                .with_label("Warning primary"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_tone(ButtonTone::Warning)
                                .with_label("Warning secondary"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_tone(ButtonTone::Warning)
                                .with_label("Warning ghost"),
                            theme,
                        )),
                ),
        )
        // --- With icons ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With icons"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .flex_wrap()
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_leading_icon("plus")
                                .with_label("Create"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_trailing_icon("external-link")
                                .with_label("Open"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_leading_icon("save")
                                .with_trailing_icon("check")
                                .with_label("Save"),
                            theme,
                        )),
                ),
        )
        // --- With chevron ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With chevron"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .flex_wrap()
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_chevron(true)
                                .with_label("Options"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_chevron(true)
                                .with_label("Actions"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_leading_icon("filter")
                                .with_chevron(true)
                                .with_label("Filter"),
                            theme,
                        )),
                ),
        )
        // (Sizes and Densities moved into the SpecimenLayout tabs below.)
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
                        .flex_wrap()
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_disabled(true)
                                .with_label("Disabled"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_loading(true)
                                .with_label("Loading"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_disabled(true)
                                .with_label("Disabled secondary"),
                            theme,
                        )),
                ),
        )
        // --- Toggle (pressed state) ---
        .child({
            let bold = state.specimens.is_on("btn-bold");
            let italic = state.specimens.is_on("btn-italic");
            let underline = !state.specimens.is_on("btn-underline-off");
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Toggle (pressed state)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .child(node_button(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_leading_icon("bold")
                                .with_pressed(bold)
                                .with_label("B"),
                            state,
                            Some(toggle_click(state, "btn-bold")),
                        ))
                        .child(node_button(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_leading_icon("italic")
                                .with_pressed(italic)
                                .with_label("I"),
                            state,
                            Some(toggle_click(state, "btn-italic")),
                        ))
                        .child(node_button(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_leading_icon("underline")
                                .with_pressed(underline)
                                .with_label("U"),
                            state,
                            Some(toggle_click(state, "btn-underline-off")),
                        )),
                )
        })
        // --- Disclosure: aria_expanded on ButtonSpec (native ARIA pending D-002) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disclosure trigger (aria_expanded on spec)"),
                    theme,
                ))
                .child(node_button(
                    ButtonSpec::new()
                        .with_label("Sections")
                        .with_chevron(true)
                        .with_aria_expanded(disclosure_open),
                    state,
                    Some(toggle_click(state, "btn-disclosure-open")),
                ))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!(
                            "ButtonSpec.aria_expanded = {:?}",
                            Some(disclosure_open)
                        )),
                ),
        )
        // --- Form overrides ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Form overrides"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(8.0))
                        .child(node_button(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Save"),
                            state,
                            Some(set_text_click(state, "btn-last", "Save (submit)")),
                        ))
                        .child(node_button(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_label("Publish"),
                            state,
                            Some(set_text_click(state, "btn-last", "Publish (formaction)")),
                        )),
                ),
        )
        // --- Click counter ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Clicks: {}", click_count)),
                )
                .when(!last_clicked.is_empty(), |d| {
                    d.child(
                        div()
                            .text_sm()
                            .text_color(color_to_hsla(text_secondary))
                            .child(format!("Last clicked: {}", last_clicked)),
                    )
                }),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "button",
        examples,
        // Sizes pane: one button per size.
        |size, theme: &GpuiThemeProvider| {
            node_button_static(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_size(size)
                    .with_label("Enabled"),
                theme,
            )
        },
        // Densities pane: one button per density.
        |density, theme: &GpuiThemeProvider| {
            node_button_static(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label("Toggle")
                    .with_density(density),
                theme,
            )
        },
    )
}
