use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{DateRangePickerSpec, DateRangeValue};
use pug_gpui_components::PugDateRangePicker;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child({
            let mut spec = DateRangePickerSpec::new();
            spec.aria_label = Some("Select date range".to_string());
            PugDateRangePicker::new(spec, theme).with_id("default")
        })
        // --- With default range ---
        .child(section_label("WITH DEFAULT RANGE", text_secondary))
        .child({
            let range = DateRangeValue::new(
                Some("2026-03-01".to_string()),
                Some("2026-03-14".to_string()),
            );
            let mut spec = DateRangePickerSpec::new()
                .with_default_value(range);
            spec.aria_label = Some("Pre-filled range".to_string());
            PugDateRangePicker::new(spec, theme).with_id("with-range")
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = DateRangePickerSpec::new();
            spec.is_disabled = true;
            spec.aria_label = Some("Disabled range picker".to_string());
            PugDateRangePicker::new(spec, theme).with_id("disabled")
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
