//! DateTimeZonePicker specimen — trigger composing a Calendar + TimeInput +
//! TimeZoneSelect overlay.
//!
//! Every group renders the real `js_date_time_zone_picker` builder; trigger,
//! indicator, and (when open) the composed Calendar + Time + Zone fields all
//! resolve from `DateTimeZonePickerSpec` + tokens. Specimens render static
//! state, so the open flag is seeded directly on the spec (`with_open`).

use crate::compat::js_date_time_zone_picker;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlDensity, ControlSize, DateTimeZonePickerSpec, ZonedDateTimeValue};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Closed, with value + zone — formatted date / time / zone in primary.
        .child(group(
            "With value",
            secondary,
            div().w(360.0).child(js_date_time_zone_picker(
                &DateTimeZonePickerSpec::new().with_value(ZonedDateTimeValue::new(
                    Some("2026-03-30".into()),
                    Some("14:30".into()),
                    Some("America/New_York".into()),
                )),
                theme,
            )),
        ))
        // Closed, no value — placeholder text in secondary color.
        .child(group(
            "Placeholder",
            secondary,
            div().w(360.0).child(js_date_time_zone_picker(
                &DateTimeZonePickerSpec::new(),
                theme,
            )),
        ))
        // Open — surface visible: composes the real Calendar + TimeInput +
        // TimeZoneSelect fields.
        .child(group(
            "Open (calendar + time + zone)",
            secondary,
            div().w(360.0).child(js_date_time_zone_picker(
                &DateTimeZonePickerSpec::new()
                    .with_default_value(ZonedDateTimeValue::new(
                        Some("2026-03-23".into()),
                        Some("14:30".into()),
                        Some("America/New_York".into()),
                    ))
                    .with_open(true),
                theme,
            )),
        ))
        // Disabled — reduced opacity, non-interactive trigger.
        .child(group(
            "Disabled",
            secondary,
            div().w(360.0).child(js_date_time_zone_picker(
                &DateTimeZonePickerSpec::new()
                    .with_value(ZonedDateTimeValue::new(
                        Some("2026-06-15".into()),
                        Some("09:00".into()),
                        Some("Europe/London".into()),
                    ))
                    .with_disabled(true),
                theme,
            )),
        ))
        // Sizes — trigger min-height, font-size, indicator font-size.
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(sized_picker(theme, ControlSize::Xs))
                .child(sized_picker(theme, ControlSize::Sm))
                .child(sized_picker(theme, ControlSize::Md))
                .child(sized_picker(theme, ControlSize::Lg))
                .child(sized_picker(theme, ControlSize::Xl)),
        ))
        // Densities — trigger horizontal padding only.
        .child(group(
            "Densities",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(dense_picker(theme, ControlDensity::Compact))
                .child(dense_picker(theme, ControlDensity::Default))
                .child(dense_picker(theme, ControlDensity::Comfortable)),
        ))
}

fn sample_value() -> ZonedDateTimeValue {
    ZonedDateTimeValue::new(
        Some("2026-03-23".into()),
        Some("14:30".into()),
        Some("America/New_York".into()),
    )
}

fn sized_picker(theme: &JetstreamThemeProvider, size: ControlSize) -> El {
    let spec = DateTimeZonePickerSpec::new()
        .with_value(sample_value())
        .with_size(size);
    div().w(360.0).child(js_date_time_zone_picker(&spec, theme))
}

fn dense_picker(theme: &JetstreamThemeProvider, density: ControlDensity) -> El {
    let spec = DateTimeZonePickerSpec::new()
        .with_value(sample_value())
        .with_density(density);
    div().w(360.0).child(js_date_time_zone_picker(&spec, theme))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
