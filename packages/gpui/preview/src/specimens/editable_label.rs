use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::EditableLabelSpec;
use pug_gpui_components::EditableLabel;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let label_value = state.specimens.text.get("editable-label-value").cloned()
        .unwrap_or_else(|| "Untitled project".to_string());
    let label_editing = state.specimens.is_on("editable-label-editing");

    let rename_value = state.specimens.text.get("editable-label-rename").cloned()
        .unwrap_or_else(|| "Click to rename".to_string());
    let rename_editing = state.specimens.is_on("editable-label-rename-editing");

    let last_event = state.specimens.text.get("editable-label-event").cloned();

    div().flex().flex_col().gap(px(16.0))
        // --- Double-click To Edit (Default) ---
        .child(section_label("DOUBLE-CLICK TO EDIT (DEFAULT)", text_secondary))
        .child(
            EditableLabel::from_spec(
                EditableLabelSpec::new()
                    .with_value(&label_value)
                    .with_editing(label_editing),
                theme,
            )
            .with_id("default")
            .on_change(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("editable-label-value".to_string(), val.to_string());
                cx.notify();
            }))
            .on_commit(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("editable-label-value".to_string(), val.to_string());
                this.state.specimens.toggles.insert("editable-label-editing".to_string(), false);
                this.state.specimens.text.insert(
                    "editable-label-event".to_string(),
                    format!("Committed: \"{}\"", val),
                );
                cx.notify();
            }))
        )

        // --- Click to edit with icon ---
        .child(section_label("CLICK TO EDIT WITH ICON", text_secondary))
        .child(
            EditableLabel::from_spec(
                EditableLabelSpec::new()
                    .with_value(&rename_value)
                    .with_editing(rename_editing),
                theme,
            )
            .with_id("with-icon")
            .on_change(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("editable-label-rename".to_string(), val.to_string());
                cx.notify();
            }))
            .on_commit(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("editable-label-rename".to_string(), val.to_string());
                this.state.specimens.toggles.insert("editable-label-rename-editing".to_string(), false);
                this.state.specimens.text.insert(
                    "editable-label-event".to_string(),
                    format!("Committed: \"{}\"", val),
                );
                cx.notify();
            }))
        )

        // --- Empty state (placeholder) ---
        .child(section_label("EMPTY STATE (PLACEHOLDER)", text_secondary))
        .child(
            EditableLabel::from_spec(
                EditableLabelSpec::new()
                    .with_placeholder("Add a description…"),
                theme,
            )
            .with_id("empty")
            .on_commit(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert(
                    "editable-label-event".to_string(),
                    format!("Committed empty-state: \"{}\"", val),
                );
                cx.notify();
            }))
        )

        // --- Pre-filled editing ---
        .child(section_label("PRE-FILLED (EDITING MODE)", text_secondary))
        .child(
            EditableLabel::from_spec(
                EditableLabelSpec::new()
                    .with_value("Currently editing this label")
                    .with_editing(true),
                theme,
            )
            .with_id("prefilled")
        )

        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            EditableLabel::from_spec(
                EditableLabelSpec::new()
                    .with_value("Locked label")
                    .with_disabled(true),
                theme,
            )
            .with_id("disabled")
        )

        // --- Last event ---
        .child(section_label("LAST EVENT", text_secondary))
        .child(
            div().text_xs().text_color(color_to_hsla(text_secondary))
                .child(match last_event {
                    Some(ref evt) => evt.clone(),
                    None => "No events yet — edit a label above.".to_string(),
                })
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
