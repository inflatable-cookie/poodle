use gpui::*;
use poodle_specs::{DateTimeRangePickerSpec, DateTimeRangeValue, DateTimeValue, EyebrowSpec};
use poodle_gpui_components::{DateTimeRangePicker, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let default_open = state.specimens.is_on("datetime-range-default-open");
    let prefilled_open = state.specimens.is_on("datetime-range-prefilled-open");

    let examples = div().flex().flex_col().gap(px(24.0)).max_w(px(384.0)) // 24rem
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child({
                    let mut spec = DateTimeRangePickerSpec::new();
                    spec.open = Some(default_open);
                    spec.aria_label = Some("Select date and time range".to_string());
                    DateTimeRangePicker::from_spec(spec, theme)
                        .with_id("default")
                        .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                            this.state.specimens.toggle("datetime-range-default-open");
                            cx.notify();
                        }))
                })
        )
        // --- With default range ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With default range"), theme))
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
                    spec.open = Some(prefilled_open);
                    spec.aria_label = Some("Pre-filled range".to_string());
                    DateTimeRangePicker::from_spec(spec, theme)
                        .with_id("with-range")
                        .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                            this.state.specimens.toggle("datetime-range-prefilled-open");
                            cx.notify();
                        }))
                })
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = DateTimeRangePickerSpec::new();
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled range picker".to_string());
                    DateTimeRangePicker::from_spec(spec, theme).with_id("disabled")
                })
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "date-time-range-picker",
        examples,
        |size, theme: &GpuiThemeProvider| {
            let mut spec = DateTimeRangePickerSpec::new();
            spec.aria_label = Some("Date time range".to_string());
            DateTimeRangePicker::from_spec(spec, theme)
                .with_id(format!("specimen-size-{:?}", size))
                .size(size)
                .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            let mut spec = DateTimeRangePickerSpec::new();
            spec.aria_label = Some("Date time range".to_string());
            DateTimeRangePicker::from_spec(spec, theme)
                .with_id(format!("specimen-density-{:?}", density))
                .with_density(density)
                .into_any_element()
        },
    )
}
