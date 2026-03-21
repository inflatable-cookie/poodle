use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::DatePickerSpec;
use pug_gpui_components::DatePicker;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child({
            let mut spec = DatePickerSpec::new();
            spec.aria_label = Some("Select date".to_string());
            DatePicker::from_spec(spec, theme).with_id("default")
        })
        // --- With default value ---
        .child(section_label("WITH DEFAULT VALUE", text_secondary))
        .child({
            let mut spec = DatePickerSpec::new()
                .with_default_value("2026-03-14");
            spec.aria_label = Some("Pre-filled date".to_string());
            DatePicker::from_spec(spec, theme).with_id("with-value")
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = DatePickerSpec::new();
            spec.placeholder = "Disabled".to_string();
            spec.is_disabled = true;
            spec.aria_label = Some("Disabled date picker".to_string());
            DatePicker::from_spec(spec, theme).with_id("disabled")
        })
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
