use gpui::*;
use poodle_primitives::{ControlDensity, ControlSize, DateTimePickerSpec, DateTimeValue, EyebrowSpec};
use poodle_gpui_components::{DateTimePicker, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let default_open = state.specimens.is_on("date-time-picker-default-open");
    let prefilled_open = state.specimens.is_on("date-time-picker-prefilled-open");

    div().flex().flex_col().gap(px(24.0)).max_w(px(320.0)) // 20rem
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
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child({
                    let sizes: &[(&str, ControlSize)] = &[
                        ("xs", ControlSize::Xs),
                        ("sm", ControlSize::Sm),
                        ("md", ControlSize::Md),
                        ("lg", ControlSize::Lg),
                        ("xl", ControlSize::Xl),
                    ];
                    let mut col = div().flex().flex_col().gap(px(8.0));
                    for &(key, size) in sizes {
                        let mut spec = DateTimePickerSpec::new();
                        spec.aria_label = Some(format!("Date time picker size {}", key));
                        col = col.child(
                            DateTimePicker::from_spec(spec, theme)
                                .with_id(format!("size-{}", key))
                                .size(size)
                        );
                    }
                    col
                })
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child({
                    let densities: &[(&str, ControlDensity)] = &[
                        ("compact", ControlDensity::Compact),
                        ("default", ControlDensity::Default),
                        ("comfortable", ControlDensity::Comfortable),
                    ];
                    let mut col = div().flex().flex_col().gap(px(8.0));
                    for &(key, density) in densities {
                        let mut spec = DateTimePickerSpec::new();
                        spec.aria_label = Some(format!("Date time picker density {}", key));
                        col = col.child(
                            DateTimePicker::from_spec(spec, theme)
                                .with_id(format!("density-{}", key))
                                .with_density(density)
                        );
                    }
                    col
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
}
