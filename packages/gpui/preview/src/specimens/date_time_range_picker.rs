use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{DateTimeRangePickerSpec, DateTimeRangeValue, DateTimeValue};
use pug_gpui_components::PugDateTimeRangePicker;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child({
            let mut spec = DateTimeRangePickerSpec::new();
            spec.aria_label = Some("Select date and time range".to_string());
            PugDateTimeRangePicker::new(spec, theme).with_id("default")
        })
        // --- With default range ---
        .child(section_label("WITH DEFAULT RANGE", text_secondary))
        .child({
            let range = DateTimeRangeValue::new(
                DateTimeValue::new(
                    Some("2026-03-10".to_string()),
                    Some("09:00".to_string()),
                ),
                DateTimeValue::new(
                    Some("2026-03-14".to_string()),
                    Some("17:00".to_string()),
                ),
            );
            let mut spec = DateTimeRangePickerSpec::new()
                .with_default_value(range);
            spec.aria_label = Some("Pre-filled range".to_string());
            PugDateTimeRangePicker::new(spec, theme).with_id("with-range")
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = DateTimeRangePickerSpec::new();
            spec.is_disabled = true;
            spec.aria_label = Some("Disabled range picker".to_string());
            PugDateTimeRangePicker::new(spec, theme).with_id("disabled")
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
