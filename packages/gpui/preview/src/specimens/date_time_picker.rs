use gpui::*;
use flint_primitives::{DateTimePickerSpec, DateTimeValue, EyebrowSpec};
use flint_gpui_components::{DateTimePicker, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let default_open = state.specimens.is_on("date-time-picker-default-open");
    let prefilled_open = state.specimens.is_on("date-time-picker-prefilled-open");

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(10.0))
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
            div().flex().flex_col().gap(px(10.0))
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
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = DateTimePickerSpec::new();
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled date time picker".to_string());
                    DateTimePicker::from_spec(spec, theme).with_id("disabled")
                })
        )
}
