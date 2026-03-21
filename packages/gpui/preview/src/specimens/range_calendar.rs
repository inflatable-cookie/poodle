use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{RangeCalendarSpec, DateRangeValue};
use pug_gpui_components::RangeCalendar;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child({
            let mut spec = RangeCalendarSpec::new();
            spec.aria_label = Some("Select a date range".to_string());
            RangeCalendar::from_spec(spec, theme).with_id("default")
        })
        // --- With pre-selected range ---
        .child(section_label("WITH PRE-SELECTED RANGE", text_secondary))
        .child({
            let range = DateRangeValue::new(
                Some("2026-03-05".to_string()),
                Some("2026-03-12".to_string()),
            );
            let mut spec = RangeCalendarSpec::new()
                .with_default_value(range);
            spec.aria_label = Some("Pre-selected range".to_string());
            RangeCalendar::from_spec(spec, theme).with_id("preselected")
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = RangeCalendarSpec::new();
            spec.is_disabled = true;
            spec.aria_label = Some("Disabled range calendar".to_string());
            RangeCalendar::from_spec(spec, theme).with_id("disabled")
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
