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

fn live_editable_label(
    state: &AppState,
    key: &str,
    mut spec: EditableLabelSpec,
    theme: &GpuiThemeProvider,
    record_commit: bool,
) -> EditableLabel {
    let draft_key = format!("{key}-draft");
    let editing_key = format!("{key}-editing");
    let committed = spec.value.clone();
    let select_on_focus = spec.select_on_focus;
    let editing = state
        .specimens
        .toggles
        .get(&editing_key)
        .copied()
        .unwrap_or(spec.is_editing);
    if let Some(draft) = state.specimens.text.get(&draft_key) {
        spec = spec.with_draft_value(Some(draft.clone()));
    } else if editing {
        spec = spec.with_draft_value(Some(committed.clone()));
    }
    spec = spec.with_editing(editing);
    let live_len = spec.live_text().chars().count();
    let (start, end) = state.specimens.carets.get(key).copied().unwrap_or_else(|| {
        if select_on_focus {
            (0, live_len)
        } else {
            (live_len, live_len)
        }
    });
    spec = spec.with_selection(start, end);
    let restore_key = format!("{key}-restore-focus");
    let restore = state.specimens.is_on(&restore_key);
    spec = spec.with_request_focus(restore);

    let events = Arc::clone(&state.node_events);
    let draft_events = Arc::clone(&state.node_events);
    let caret_events = Arc::clone(&state.node_events);
    let commit_events = Arc::clone(&state.node_events);
    let cancel_events = Arc::clone(&state.node_events);
    let start_events = Arc::clone(&state.node_events);
    let restore_events = Arc::clone(&state.node_events);
    let key_owned = key.to_string();
    let draft_owned = draft_key.clone();
    let editing_owned = editing_key.clone();
    let restore_owned = restore_key.clone();

    if restore {
        events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
            key: restore_key,
            value: false,
        });
    }

    EditableLabel::from_spec(spec, theme)
        .on_edit_start({
            let events = start_events;
            let committed = committed.clone();
            let draft_key = draft_owned.clone();
            let editing_key = editing_owned.clone();
            let caret_key = key_owned.clone();
            Arc::new(move || {
                let len = committed.chars().count();
                let mut queue = events.lock().unwrap();
                queue.push(NodeSpecimenEvent::SetText {
                    key: draft_key.clone(),
                    value: committed.clone(),
                });
                queue.push(NodeSpecimenEvent::SetToggle {
                    key: editing_key.clone(),
                    value: true,
                });
                queue.push(NodeSpecimenEvent::SetCaret {
                    key: caret_key.clone(),
                    start: if select_on_focus { 0 } else { len },
                    end: len,
                });
            })
        })
        .on_change(Arc::new(move |value: &str| {
            draft_events
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetText {
                    key: draft_owned.clone(),
                    value: value.to_string(),
                });
        }))
        .on_selection_change(Arc::new(move |start: usize, end: usize| {
            caret_events
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetCaret {
                    key: key_owned.clone(),
                    start,
                    end,
                });
        }))
        .on_commit({
            let value_key = key.to_string();
            let draft_key = draft_key.clone();
            let editing_key = editing_key.clone();
            Arc::new(move |value: &str, _previous: &str| {
                let mut queue = commit_events.lock().unwrap();
                queue.push(NodeSpecimenEvent::SetText {
                    key: value_key.clone(),
                    value: value.to_string(),
                });
                queue.push(NodeSpecimenEvent::SetOptionalText {
                    key: draft_key.clone(),
                    value: None,
                });
                queue.push(NodeSpecimenEvent::SetToggle {
                    key: editing_key.clone(),
                    value: false,
                });
                if record_commit {
                    queue.push(NodeSpecimenEvent::SetText {
                        key: "editable-label-event".to_string(),
                        value: format!("Committed: \"{value}\""),
                    });
                }
            })
        })
        .on_cancel(Arc::new(move || {
            let mut queue = cancel_events.lock().unwrap();
            queue.push(NodeSpecimenEvent::SetOptionalText {
                key: draft_key.clone(),
                value: None,
            });
            queue.push(NodeSpecimenEvent::SetToggle {
                key: editing_key.clone(),
                value: false,
            });
            queue.push(NodeSpecimenEvent::SetText {
                key: "editable-label-event".to_string(),
                value: "Edit cancelled".to_string(),
            });
        }))
        .on_restore_display_focus(Arc::new(move || {
            restore_events
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetToggle {
                    key: restore_owned.clone(),
                    value: true,
                });
        }))
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
                    live_editable_label(
                        state,
                        "editable-label-title",
                        EditableLabelSpec::new().with_value(&title_value),
                        theme,
                        true,
                    )
                    .with_id("default"),
                ),
        )
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
                    live_editable_label(
                        state,
                        "editable-label-live",
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
                        false,
                    )
                    .with_id("editing"),
                ),
        )
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
                    live_editable_label(
                        state,
                        "editable-label-title",
                        EditableLabelSpec::new()
                            .with_value(&title_value)
                            .with_activation_mode(EditableLabelActivation::EnterOrSpace)
                            .with_show_edit_icon(true),
                        theme,
                        true,
                    )
                    .with_id("with-icon"),
                ),
        )
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
                    live_editable_label(
                        state,
                        "editable-label-empty",
                        EditableLabelSpec::new()
                            .with_value(&empty_value)
                            .with_activation_mode(EditableLabelActivation::EnterOrSpace)
                            .with_empty_text("Add a description\u{2026}")
                            .with_placeholder("Add a description\u{2026}"),
                        theme,
                        true,
                    )
                    .with_id("empty"),
                ),
        )
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
                    live_editable_label(
                        state,
                        "editable-label-flush",
                        EditableLabelSpec::new()
                            .with_value(&flush_value)
                            .with_variant(EditableLabelVariant::Flush)
                            .with_activation_mode(EditableLabelActivation::EnterOrSpace)
                            .with_show_edit_icon(true),
                        theme,
                        false,
                    )
                    .with_id("flush"),
                ),
        )
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
                    live_editable_label(
                        state,
                        "editable-label-max",
                        EditableLabelSpec::new()
                            .with_value("Short text")
                            .with_activation_mode(EditableLabelActivation::EnterOrSpace)
                            .with_max_length(20)
                            .with_placeholder("Enter text\u{2026}")
                            .with_editing(true),
                        theme,
                        false,
                    )
                    .with_id("max-length"),
                ),
        )
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
