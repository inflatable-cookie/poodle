use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{DateRangePickerSpec, DateRangeValue};
use pug_gpui_components::DateRangePicker;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let default_open = state.specimens.is_on("date-range-picker-default-open");
    let prefilled_open = state.specimens.is_on("date-range-picker-prefilled-open");

    div().flex().flex_col().gap(px(16.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child({
            let mut spec = DateRangePickerSpec::new()
                .with_open(default_open);
            spec.aria_label = Some("Select date range".to_string());
            DateRangePicker::from_spec(spec, theme)
                .with_id("default")
                .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                    this.state.specimens.toggle("date-range-picker-default-open");
                    cx.notify();
                }))
        })
        // --- With default range ---
        .child(section_label("WITH DEFAULT RANGE", text_secondary))
        .child({
            let range = DateRangeValue::new(
                Some("2026-03-01".to_string()),
                Some("2026-03-14".to_string()),
            );
            let mut spec = DateRangePickerSpec::new()
                .with_default_value(range)
                .with_open(prefilled_open);
            spec.aria_label = Some("Pre-filled range".to_string());
            DateRangePicker::from_spec(spec, theme)
                .with_id("with-range")
                .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                    this.state.specimens.toggle("date-range-picker-prefilled-open");
                    cx.notify();
                }))
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = DateRangePickerSpec::new();
            spec.is_disabled = true;
            spec.aria_label = Some("Disabled range picker".to_string());
            DateRangePicker::from_spec(spec, theme).with_id("disabled")
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
