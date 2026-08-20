use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{EditableLabel, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EditableLabelActivation, EditableLabelSpec, EditableLabelVariant, EyebrowSpec};
use std::sync::Arc;

fn queue_change(state: &AppState, key: &str) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = Arc::clone(&state.node_events);
    let key = key.to_string();
    Arc::new(move |value: &str| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.clone(),
            value: value.to_string(),
        });
    })
}

fn queue_commit(
    state: &AppState,
    value_key: &str,
    record_event: bool,
) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = Arc::clone(&state.node_events);
    let value_key = value_key.to_string();
    Arc::new(move |value: &str| {
        let mut queue = events.lock().unwrap();
        queue.push(NodeSpecimenEvent::SetText {
            key: value_key.clone(),
            value: value.to_string(),
        });
        if record_event {
            queue.push(NodeSpecimenEvent::SetText {
                key: "editable-label-event".to_string(),
                value: format!("Committed: \"{value}\""),
            });
        }
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let title_value = state
        .specimens
        .text
        .get("editable-label-title")
        .cloned()
        .unwrap_or_else(|| "My project title".to_string());

    let empty_value = state
        .specimens
        .text
        .get("editable-label-empty")
        .cloned()
        .unwrap_or_default();

    let flush_value = state
        .specimens
        .text
        .get("editable-label-flush")
        .cloned()
        .unwrap_or_else(|| "Inline heading".to_string());

    let last_event = state.specimens.text.get("editable-label-event").cloned();

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(384.0))
        // --- Double-click to edit (default, interactive) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Double-click to edit (default)"),
                    theme,
                ))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new().with_value(&title_value),
                        theme,
                    )
                    .with_id("default")
                    .on_change(queue_change(state, "editable-label-title"))
                    .on_commit(queue_commit(
                        state,
                        "editable-label-title",
                        true,
                    )),
                ),
        )
        // --- Editing mode (composed input shown, live) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Editing mode (input shown, live)"),
                    theme,
                ))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value(
                                state
                                    .specimens
                                    .text
                                    .get("editable-label-live")
                                    .cloned()
                                    .unwrap_or_else(|| "My project title".to_string()),
                            )
                            .with_editing(true),
                        theme,
                    )
                    .with_id("editing")
                    .on_change(queue_change(state, "editable-label-live")),
                ),
        )
        // --- Click to edit with icon (enterOrSpace + showEditIcon) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Click to edit with icon"),
                    theme,
                ))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value(&title_value)
                            .with_activation_mode(EditableLabelActivation::EnterOrSpace)
                            .with_show_edit_icon(true),
                        theme,
                    )
                    .with_id("with-icon")
                    .on_commit(queue_commit(
                        state,
                        "editable-label-title",
                        true,
                    )),
                ),
        )
        // --- Empty state (emptyText) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Empty state"),
                    theme,
                ))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value(&empty_value)
                            .with_activation_mode(EditableLabelActivation::EnterOrSpace)
                            .with_empty_text("Add a description\u{2026}")
                            .with_placeholder("Add a description\u{2026}"),
                        theme,
                    )
                    .with_id("empty")
                    .on_commit(queue_commit(
                        state,
                        "editable-label-empty",
                        true,
                    )),
                ),
        )
        // --- Flush variant (display) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Flush variant"),
                    theme,
                ))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value(&flush_value)
                            .with_variant(EditableLabelVariant::Flush)
                            .with_activation_mode(EditableLabelActivation::EnterOrSpace)
                            .with_show_edit_icon(true),
                        theme,
                    )
                    .with_id("flush")
                    .on_commit(queue_commit(
                        state,
                        "editable-label-flush",
                        false,
                    )),
                ),
        )
        // --- With max length (maxLength + placeholder, editing) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With max length (20)"),
                    theme,
                ))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value("Short text")
                            .with_activation_mode(EditableLabelActivation::EnterOrSpace)
                            .with_max_length(20)
                            .with_placeholder("Enter text\u{2026}")
                            .with_editing(true),
                        theme,
                    )
                    .with_id("max-length"),
                ),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value("Read-only value")
                            .with_disabled(true),
                        theme,
                    )
                    .with_id("disabled"),
                ),
        )
        // --- Last event ---
        .when(last_event.is_some(), |el| {
            el.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(Eyebrow::from_spec(
                        EyebrowSpec::new().with_content("Last event"),
                        theme,
                    ))
                    .child(
                        div()
                            .text_sm()
                            .text_color(color_to_hsla(text_secondary))
                            .child(last_event.unwrap_or_default()),
                    ),
            )
        })
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "editable-label",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                EditableLabel::from_spec(EditableLabelSpec::new().with_value("Editable"), theme)
                    .with_id(format!("specimen-size-{:?}", size))
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                EditableLabel::from_spec(EditableLabelSpec::new().with_value("Editable"), theme)
                    .with_id(format!("specimen-density-{:?}", density))
                    .density(density)
                    .into_any_element()
            }),
    )
}
