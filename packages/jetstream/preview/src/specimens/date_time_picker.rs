//! DateTimePicker specimen — trigger composing a Calendar + TimeInput overlay.
//!
//! Every group renders the real `js_date_time_picker` builder; trigger,
//! indicator, and (when open) the composed Calendar + Time Section surface all
//! resolve from `DateTimePickerSpec` + tokens. Specimens render static state,
//! so the open flag is seeded directly on the spec (`spec.open`).

use crate::compat::js_date_time_picker;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlDensity, ControlSize, DateTimePickerSpec, DateTimeValue};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Closed, no value — placeholder text in secondary color.
        .child(group(
            "Default",
            secondary,
            div().w(320.0).child(js_date_time_picker(
                &{
                    let mut spec = DateTimePickerSpec::new();
                    spec.aria_label = Some("Select date and time".to_string());
                    spec
                },
                theme,
            )),
        ))
        // Closed, with value — formatted date + time in primary color.
        .child(group(
            "With default value",
            secondary,
            div().w(320.0).child(js_date_time_picker(
                &{
                    let mut spec = DateTimePickerSpec::new().with_default_value(
                        DateTimeValue::new(Some("2026-03-14".into()), Some("14:30".into())),
                    );
                    spec.aria_label = Some("Pre-filled date time".to_string());
                    spec
                },
                theme,
            )),
        ))
        // Open — surface visible, composes the real Calendar + TimeInput.
        .child(group(
            "Open (calendar + time)",
            secondary,
            div().w(320.0).child(js_date_time_picker(
                &{
                    let mut spec = DateTimePickerSpec::new().with_default_value(
                        DateTimeValue::new(Some("2026-03-14".into()), Some("14:30".into())),
                    );
                    spec.open = Some(true);
                    spec.aria_label = Some("Open date time picker".to_string());
                    spec
                },
                theme,
            )),
        ))
        // Disabled — reduced opacity, non-interactive trigger.
        .child(group(
            "Disabled",
            secondary,
            div().w(320.0).child(js_date_time_picker(
                &{
                    let mut spec = DateTimePickerSpec::new().with_default_value(
                        DateTimeValue::new(Some("2026-06-15".into()), Some("09:00".into())),
                    );
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled date time picker".to_string());
                    spec
                },
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

fn sized_picker(theme: &JetstreamThemeProvider, size: ControlSize) -> El {
    let mut spec = DateTimePickerSpec::new()
        .with_size(size)
        .with_default_value(DateTimeValue::new(
            Some("2026-03-14".into()),
            Some("14:30".into()),
        ));
    spec.aria_label = Some("Date time picker".to_string());
    div().w(320.0).child(js_date_time_picker(&spec, theme))
}

fn dense_picker(theme: &JetstreamThemeProvider, density: ControlDensity) -> El {
    let mut spec = DateTimePickerSpec::new()
        .with_density(density)
        .with_default_value(DateTimeValue::new(
            Some("2026-03-14".into()),
            Some("14:30".into()),
        ));
    spec.aria_label = Some("Date time picker".to_string());
    div().w(320.0).child(js_date_time_picker(&spec, theme))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
