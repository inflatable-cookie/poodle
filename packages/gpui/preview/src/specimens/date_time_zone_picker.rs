use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{DateTimeZonePicker, Eyebrow};
use poodle_specs::{DateTimeZonePickerSpec, EyebrowSpec, ZonedDateTimeValue};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let is_open = state.specimens.is_on("dtz-picker-open");

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(320.0))
        // --- Default ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child(
                    DateTimeZonePicker::from_spec(
                        DateTimeZonePickerSpec::new()
                            .with_value(ZonedDateTimeValue::new(
                                Some("2026-03-23".into()),
                                Some("14:30".into()),
                                Some("America/New_York".into()),
                            ))
                            .with_open(is_open),
                        theme,
                    )
                    .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                        this.state.specimens.toggle("dtz-picker-open");
                        cx.notify();
                    })),
                ),
        )
        // --- With default value ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With default value"),
                    theme,
                ))
                .child(DateTimeZonePicker::from_spec(
                    DateTimeZonePickerSpec::new().with_default_value(ZonedDateTimeValue::new(
                        Some("2026-03-14".into()),
                        Some("10:00".into()),
                        Some("America/Los_Angeles".into()),
                    )),
                    theme,
                )),
        )
        // --- Open (calendar + time + zone) ---
        // Static open state so the composed real Calendar + TimeInput +
        // TimeZoneSelect fields are always visible for review without interaction.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Open (calendar + time + zone)"),
                    theme,
                ))
                .child(DateTimeZonePicker::from_spec(
                    DateTimeZonePickerSpec::new()
                        .with_default_value(ZonedDateTimeValue::new(
                            Some("2026-03-23".into()),
                            Some("14:30".into()),
                            Some("America/New_York".into()),
                        ))
                        .with_open(true),
                    theme,
                )),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(DateTimeZonePicker::from_spec(
                    DateTimeZonePickerSpec::new().with_disabled(true),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "date-time-zone-picker",
        examples,
        |size, theme: &GpuiThemeProvider| {
            DateTimeZonePicker::from_spec(
                DateTimeZonePickerSpec::new().with_value(ZonedDateTimeValue::new(
                    Some("2026-03-23".into()),
                    Some("14:30".into()),
                    Some("America/New_York".into()),
                )),
                theme,
            )
            .size(size)
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            DateTimeZonePicker::from_spec(
                DateTimeZonePickerSpec::new().with_value(ZonedDateTimeValue::new(
                    Some("2026-03-23".into()),
                    Some("14:30".into()),
                    Some("America/New_York".into()),
                )),
                theme,
            )
            .with_density(density)
            .into_any_element()
        },
    )
}
