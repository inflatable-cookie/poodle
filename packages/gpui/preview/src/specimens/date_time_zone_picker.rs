use gpui::*;
use poodle_primitives::{ControlDensity, ControlSize, DateTimeZonePickerSpec, EyebrowSpec};
use poodle_gpui_components::{DateTimeZonePicker, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let is_open = state.specimens.is_on("dtz-picker-open");

    div().flex().flex_col().gap(px(24.0)).max_w(px(320.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    DateTimeZonePicker::from_spec(
                        DateTimeZonePickerSpec::new()
                            .with_value("2026-03-23T14:30:00")
                            .with_time_zone("America/New_York")
                            .with_open(is_open),
                        theme,
                    )
                    .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                        this.state.specimens.toggle("dtz-picker-open");
                        cx.notify();
                    }))
                )
        )
        // --- With default value ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With default value"), theme))
                .child(
                    DateTimeZonePicker::from_spec(
                        DateTimeZonePickerSpec::new()
                            .with_value("2026-03-23T19:30:00")
                            .with_time_zone("Europe/London"),
                        theme,
                    )
                )
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child({
                    let make_spec = || DateTimeZonePickerSpec::new()
                        .with_value("2026-03-23T14:30:00")
                        .with_time_zone("America/New_York");
                    div().flex().flex_col().gap(px(8.0))
                        .child(DateTimeZonePicker::from_spec(make_spec(), theme).size(ControlSize::Xs))
                        .child(DateTimeZonePicker::from_spec(make_spec(), theme).size(ControlSize::Sm))
                        .child(DateTimeZonePicker::from_spec(make_spec(), theme).size(ControlSize::Md))
                        .child(DateTimeZonePicker::from_spec(make_spec(), theme).size(ControlSize::Lg))
                        .child(DateTimeZonePicker::from_spec(make_spec(), theme).size(ControlSize::Xl))
                })
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child({
                    let make_spec = || DateTimeZonePickerSpec::new()
                        .with_value("2026-03-23T14:30:00")
                        .with_time_zone("America/New_York");
                    div().flex().flex_col().gap(px(8.0))
                        .child(DateTimeZonePicker::from_spec(make_spec(), theme).with_density(ControlDensity::Compact))
                        .child(DateTimeZonePicker::from_spec(make_spec(), theme).with_density(ControlDensity::Default))
                        .child(DateTimeZonePicker::from_spec(make_spec(), theme).with_density(ControlDensity::Comfortable))
                })
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    DateTimeZonePicker::from_spec(
                        DateTimeZonePickerSpec::new()
                            .with_value("2026-01-01T00:00:00")
                            .with_time_zone("UTC")
                            .with_disabled(true),
                        theme,
                    )
                )
        )
}
