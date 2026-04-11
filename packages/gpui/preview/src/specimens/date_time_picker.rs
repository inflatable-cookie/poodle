use gpui::*;
use poodle_specs::{DateTimePickerSpec, DateTimeValue, EyebrowSpec};
use poodle_gpui_components::{DateTimePicker, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let default_open = state.specimens.is_on("date-time-picker-default-open");
    let prefilled_open = state.specimens.is_on("date-time-picker-prefilled-open");

    let examples = div().flex().flex_col().gap(px(24.0)).max_w(px(320.0)) // 20rem
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child({
                    let mut spec = DateTimePickerSpec::new();
                    spec.open = Some(default_open);
                    spec.aria_label = Some("Select date and time".to_string());
                    DateTimePicker::from_spec(spec, theme)
                        .with_id("default")
                        .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                            this.state.specimens.toggle("date-time-picker-default-open");
                            cx.notify();
                        }))
                })
        )
        // --- With default value ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With default value"), theme))
                .child({
                    let value = DateTimeValue::new(
                        Some("2026-03-14".to_string()),
                        Some("14:30".to_string()),
                    );
                    let mut spec = DateTimePickerSpec::new()
                        .with_default_value(value);
                    spec.open = Some(prefilled_open);
                    spec.aria_label = Some("Pre-filled date time".to_string());
                    DateTimePicker::from_spec(spec, theme)
                        .with_id("with-value")
                        .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                            this.state.specimens.toggle("date-time-picker-prefilled-open");
                            cx.notify();
                        }))
                })
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = DateTimePickerSpec::new();
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled date time picker".to_string());
                    DateTimePicker::from_spec(spec, theme).with_id("disabled")
                })
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "date-time-picker",
        examples,
        |size, theme: &GpuiThemeProvider| {
            let mut spec = DateTimePickerSpec::new();
            spec.aria_label = Some("Date time picker".to_string());
            DateTimePicker::from_spec(spec, theme)
                .with_id(format!("specimen-size-{:?}", size))
                .size(size)
                .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            let mut spec = DateTimePickerSpec::new();
            spec.aria_label = Some("Date time picker".to_string());
            DateTimePicker::from_spec(spec, theme)
                .with_id(format!("specimen-density-{:?}", density))
                .with_density(density)
                .into_any_element()
        },
    )
}
