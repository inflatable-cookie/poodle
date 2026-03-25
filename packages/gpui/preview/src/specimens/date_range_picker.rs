use gpui::*;
use poodle_primitives::{DateRangePickerSpec, DateRangeValue, EyebrowSpec};
use poodle_gpui_components::{DateRangePicker, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let default_open = state.specimens.is_on("date-range-picker-default-open");
    let prefilled_open = state.specimens.is_on("date-range-picker-prefilled-open");

    div().flex().flex_col().gap(px(24.0)).max_w(px(320.0)) // 20rem
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
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
        )
        // --- With default range ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With default range"), theme))
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
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = DateRangePickerSpec::new();
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled range picker".to_string());
                    DateRangePicker::from_spec(spec, theme).with_id("disabled")
                })
        )
}
