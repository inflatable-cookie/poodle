use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{EditableLabel, Eyebrow};
use poodle_specs::{
    EditableLabelActivation, EditableLabelSpec, EditableLabelVariant, EyebrowSpec,
};

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
        // --- Display mode (value + pencil edit-icon) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Display mode (value + edit icon)"),
                    theme,
                ))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value("My project title")
                            .with_show_edit_icon(true),
                        theme,
                    )
                    .with_id("display-icon"),
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
                    .on_change(cx.listener(|this, val: &str, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("editable-label-live".to_string(), val.to_string());
                        cx.notify();
                    })),
                ),
        )
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
                    .on_change(cx.listener(|this, val: &str, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("editable-label-title".to_string(), val.to_string());
                        cx.notify();
                    }))
                    .on_commit(cx.listener(|this, val: &str, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("editable-label-title".to_string(), val.to_string());
                        this.state.specimens.text.insert(
                            "editable-label-event".to_string(),
                            format!("Committed: \"{}\"", val),
                        );
                        cx.notify();
                    })),
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
                    .on_commit(cx.listener(|this, val: &str, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("editable-label-title".to_string(), val.to_string());
                        this.state.specimens.text.insert(
                            "editable-label-event".to_string(),
                            format!("Committed: \"{}\"", val),
                        );
                        cx.notify();
                    })),
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
                    .on_commit(cx.listener(|this, val: &str, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("editable-label-empty".to_string(), val.to_string());
                        this.state.specimens.text.insert(
                            "editable-label-event".to_string(),
                            format!("Committed: \"{}\"", val),
                        );
                        cx.notify();
                    })),
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
                    .on_commit(cx.listener(|this, val: &str, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("editable-label-flush".to_string(), val.to_string());
                        cx.notify();
                    })),
                ),
        )
        // --- Flush variant (editing — bottom border only) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Flush variant (editing)"),
                    theme,
                ))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value("Inline heading")
                            .with_variant(EditableLabelVariant::Flush)
                            .with_editing(true),
                        theme,
                    )
                    .with_id("flush-editing"),
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
        |size, theme: &GpuiThemeProvider| {
            EditableLabel::from_spec(EditableLabelSpec::new().with_value("Editable"), theme)
                .with_id(format!("specimen-size-{:?}", size))
                .size(size)
                .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            EditableLabel::from_spec(EditableLabelSpec::new().with_value("Editable"), theme)
                .with_id(format!("specimen-density-{:?}", density))
                .density(density)
                .into_any_element()
        },
    )
}
