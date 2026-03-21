use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::CalendarSpec;
use pug_gpui_components::Calendar;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child({
            let mut spec = CalendarSpec::new();
            spec.aria_label = Some("Select a date".to_string());
            Calendar::from_spec(spec, theme).with_id("default")
        })
        // --- With pre-selected date ---
        .child(section_label("WITH PRE-SELECTED DATE", text_secondary))
        .child({
            let mut spec = CalendarSpec::new();
            spec.default_value = Some("2026-03-14".to_string());
            spec.aria_label = Some("Calendar with default".to_string());
            Calendar::from_spec(spec, theme).with_id("preselected")
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = CalendarSpec::new();
            spec.default_value = Some("2026-03-01".to_string());
            spec.is_disabled = true;
            spec.aria_label = Some("Disabled calendar".to_string());
            Calendar::from_spec(spec, theme).with_id("disabled")
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
