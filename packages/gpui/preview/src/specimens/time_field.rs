use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::TimeFieldSpec;
use pug_gpui_components::PugTimeField;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child({
            let mut spec = TimeFieldSpec::new();
            spec.aria_label = Some("Start time".to_string());
            PugTimeField::new(spec, theme).with_id("default")
        })
        // --- With default value ---
        .child(section_label("WITH DEFAULT VALUE", text_secondary))
        .child({
            let mut spec = TimeFieldSpec::new()
                .with_default_value("14:30");
            spec.aria_label = Some("Meeting time".to_string());
            PugTimeField::new(spec, theme).with_id("with-value")
        })
        // --- With min/max constraints ---
        .child(section_label("WITH MIN/MAX CONSTRAINTS", text_secondary))
        .child({
            let mut spec = TimeFieldSpec::new()
                .with_default_value("09:00");
            spec.min = Some("08:00".to_string());
            spec.max = Some("18:00".to_string());
            spec.aria_label = Some("Office hours".to_string());
            PugTimeField::new(spec, theme).with_id("constrained")
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = TimeFieldSpec::new()
                .with_default_value("12:00");
            spec.is_disabled = true;
            PugTimeField::new(spec, theme).with_id("disabled")
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
