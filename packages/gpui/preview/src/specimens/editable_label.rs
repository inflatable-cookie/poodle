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
            .on_change(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("editable-label-value".to_string(), val.to_string());
                cx.notify();
            }))
            .on_commit(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("editable-label-value".to_string(), val.to_string());
                this.state.specimens.toggles.insert("editable-label-editing".to_string(), false);
                cx.notify();
            }))
            // Note: on_cancel takes Fn(&mut Window, &mut App) which can't use cx.listener
        )
        // --- Enter/Space Activation ---
        .child(section_label("ENTER/SPACE ACTIVATION", text_secondary))
        .child(
            EditableLabel::from_spec(
                EditableLabelSpec::new()
                    .with_value(&rename_value)
                    .with_editing(rename_editing),
                theme,
            )
            .on_change(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("editable-label-rename".to_string(), val.to_string());
                cx.notify();
            }))
            .on_commit(cx.listener(|this, val: &str, _w, cx| {
                this.state.specimens.text.insert("editable-label-rename".to_string(), val.to_string());
                this.state.specimens.toggles.insert("editable-label-rename-editing".to_string(), false);
                cx.notify();
            }))
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
