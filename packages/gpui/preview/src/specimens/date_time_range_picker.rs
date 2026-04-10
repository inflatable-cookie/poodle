use gpui::*;
use poodle_primitives::{ControlDensity, ControlSize, DateTimeRangePickerSpec, DateTimeRangeValue, DateTimeValue, EyebrowSpec};
use poodle_gpui_components::{DateTimeRangePicker, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let default_open = state.specimens.is_on("datetime-range-default-open");
    let prefilled_open = state.specimens.is_on("datetime-range-prefilled-open");

    div().flex().flex_col().gap(px(24.0)).max_w(px(384.0)) // 24rem
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
                        let mut spec = DateTimeRangePickerSpec::new();
                        spec.aria_label = Some(format!("Date time range size {}", key));
                        col = col.child(
                            DateTimeRangePicker::from_spec(spec, theme)
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
                        let mut spec = DateTimeRangePickerSpec::new();
                        spec.aria_label = Some(format!("Date time range density {}", key));
                        col = col.child(
                            DateTimeRangePicker::from_spec(spec, theme)
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
                    let mut spec = DateTimeRangePickerSpec::new();
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled range picker".to_string());
                    DateTimeRangePicker::from_spec(spec, theme).with_id("disabled")
                })
        )
}
