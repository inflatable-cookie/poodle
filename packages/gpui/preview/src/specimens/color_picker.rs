use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::ColorPickerSpec;
use pug_gpui_components::ColorPicker;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0)).max_w(px(320.0))
        // --- Basic Picker ---
        .child(section_label("BASIC PICKER", text_secondary))
        .child(
            ColorPicker::from_spec(
                ColorPickerSpec::new()
                    .with_value("#6366f1"),
                theme,
            )
        )
        // --- With Alpha ---
        .child(section_label("WITH ALPHA", text_secondary))
        .child(
            ColorPicker::from_spec(
                ColorPickerSpec::new()
                    .with_value("#3b82f6")
                    .with_show_alpha(true),
                theme,
            )
        )
        // --- Default Open ---
        .child(section_label("DEFAULT OPEN, RGB MODE", text_secondary))
        .child(
            ColorPicker::from_spec(
                ColorPickerSpec::new()
                    .with_value("#22c55e")
                    .with_open(true),
                theme,
            )
        )
        // --- Preview Only (No Input) ---
        .child(section_label("PREVIEW ONLY (NO INPUT)", text_secondary))
        .child(
            ColorPicker::from_spec(
                ColorPickerSpec::new()
                    .with_value("#6366f1"),
                theme,
            )
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            ColorPicker::from_spec(
                ColorPickerSpec::new()
                    .with_value("#22c55e")
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
