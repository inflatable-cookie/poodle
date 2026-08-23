//! Button specimen — migrated to the node tier (g12.019 Batch B).
//!
//! Every Button below renders through the node tier: `poodle_render::button`
//! (`Spec + Context → Node`) interpreted by `poodle_gpui_node_backend::to_gpui`.
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
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

use poodle_render::button;
use poodle_render::RenderContext;
use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant, EyebrowSpec};

/// A node-tier Button with an optional click handler.
fn node_button(
    spec: ButtonSpec,
    state: &AppState,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> AnyElement {
    let node = button(&spec, &RenderContext::new(&state.theme), on_click);
    poodle_gpui_node_backend::to_gpui(&node)
}

/// A node-tier Button with no handler (tones / icons / chevrons / states / sizes / densities).
fn node_button_static(spec: ButtonSpec, theme: &GpuiThemeProvider) -> AnyElement {
    let node = button(&spec, &RenderContext::new(theme), None);
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
        .gap(px(24.0)) // --- A normal action row ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("A normal action row — the primary action, then the way out"),
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
                                .with_label("Save changes"),
                            state,
                            Some(variant_click(state, "Save changes")),
                        ))
                        .child(node_button(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_label("Cancel"),
                            state,
                            Some(variant_click(state, "Cancel")),
                        )),
                ),
        )
        // --- Variants ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Variants — how much weight the action carries"),
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
        // --- Tones ---
        // One variant, every tone. Tone and variant compose freely, so the
        // 3x4 grid this replaced taught nothing the variants row does not.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Tones — what kind of action it is"),
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
                                .with_label("Default"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_tone(ButtonTone::Danger)
                                .with_label("Delete"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_tone(ButtonTone::Success)
                                .with_label("Approve"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_tone(ButtonTone::Warning)
                                .with_label("Override"),
                            theme,
                        )),
                ),
        )
        // --- Icons, disclosure, and icon-only ---
        // The disclosure trigger below is the native evidence for
        // `ButtonSpec.aria_expanded`; the web pages carry the same idea as the
        // chevron button, so it lives in the same section rather than its own.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Icons, disclosure, and icon-only"),
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
                                .with_leading_icon("filter")
                                .with_chevron(true)
                                .with_label("Filter"),
                            theme,
                        ))
                        .child(node_button_static(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_leading_icon("settings")
                                .with_aria_label("Settings"),
                            theme,
                        )),
                )
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
        // (Sizes and Densities moved into the SpecimenLayout tabs below.)
        // --- States ---
        .child({
            let bookmarked = state.specimens.is_on("btn-bookmarked");
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("States — unavailable, working, and held down"),
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
                        .child(node_button(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_leading_icon("star")
                                .with_pressed(bookmarked)
                                .with_label(if bookmarked { "Bookmarked" } else { "Bookmark" }),
                            state,
                            Some(toggle_click(state, "btn-bookmarked")),
                        )),
                )
        })
        // --- Form overrides ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Inside a form — each button can submit somewhere else"),
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
        SpecimenAxes::examples_only()
            .with_sizes(
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
            )
            .with_densities(
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
            ),
    )
}
