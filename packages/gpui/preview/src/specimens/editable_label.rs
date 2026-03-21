use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::EditableLabelSpec;
use pug_gpui_components::EditableLabel;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Double-click To Edit (Default) ---
        .child(section_label("DOUBLE-CLICK TO EDIT (DEFAULT)", text_secondary))
        .child(
            EditableLabel::from_spec(
                EditableLabelSpec::new()
                    .with_value("Untitled project"),
                theme,
            )
        )
        // --- Enter/Space Activation ---
        .child(section_label("ENTER/SPACE ACTIVATION", text_secondary))
        .child(
            EditableLabel::from_spec(
                EditableLabelSpec::new()
                    .with_value("Click to rename"),
                theme,
            )
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
