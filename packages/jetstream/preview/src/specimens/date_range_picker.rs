//! DateRangePicker specimen — trigger composing a range Calendar overlay.
//!
//! Every group renders the real `js_date_range_picker` builder; trigger,
//! indicator, and (when open) the composed Calendar surface in range mode all
//! resolve from `DateRangePickerSpec` + tokens. Specimens render static state,
//! so the open flag is seeded directly on the spec (`with_open`).

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::date_range_picker::js_date_range_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlDensity, ControlSize, DateRangePickerSpec, DateRangeValue};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Closed, no value — placeholder text in secondary color.
        .child(group(
            "Default",
            secondary,
            div().w(320.0).child(js_date_range_picker(
                &{
                    let mut spec = DateRangePickerSpec::new();
                    spec.aria_label = Some("Select date range".to_string());
                    spec
                },
                theme,
            )),
        ))
        // Closed, with range — formatted start–end in primary color.
        .child(group(
            "With default range",
            secondary,
            div().w(320.0).child(js_date_range_picker(
                &{
                    let mut spec = DateRangePickerSpec::new().with_default_value(
                        DateRangeValue::new(Some("2026-03-01".into()), Some("2026-03-14".into())),
                    );
                    spec.aria_label = Some("Pre-filled range".to_string());
                    spec
                },
                theme,
            )),
        ))
        // Open — surface visible, composes the real Calendar in range mode.
        .child(group(
            "Open (range calendar)",
            secondary,
            div().w(320.0).child(js_date_range_picker(
                &{
                    let mut spec = DateRangePickerSpec::new()
                        .with_default_value(DateRangeValue::new(
                            Some("2026-03-03".into()),
                            Some("2026-03-19".into()),
                        ))
                        .with_open(true);
                    spec.aria_label = Some("Open range picker".to_string());
                    spec
                },
                theme,
            )),
        ))
        // Disabled — reduced opacity, non-interactive trigger.
        .child(group(
            "Disabled",
            secondary,
            div().w(320.0).child(js_date_range_picker(
                &{
                    let mut spec = DateRangePickerSpec::new().with_default_value(
                        DateRangeValue::new(Some("2026-01-01".into()), Some("2026-01-31".into())),
                    );
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled range picker".to_string());
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

fn sized_picker(theme: &JetstreamThemeProvider, size: ControlSize) -> JsEl {
    let mut spec = DateRangePickerSpec::new()
        .with_size(size)
        .with_default_value(DateRangeValue::new(
            Some("2026-03-01".into()),
            Some("2026-03-14".into()),
        ));
    spec.aria_label = Some("Range picker".to_string());
    div().w(320.0).child(js_date_range_picker(&spec, theme))
}

fn dense_picker(theme: &JetstreamThemeProvider, density: ControlDensity) -> JsEl {
    let mut spec = DateRangePickerSpec::new()
        .with_density(density)
        .with_default_value(DateRangeValue::new(
            Some("2026-03-01".into()),
            Some("2026-03-14".into()),
        ));
    spec.aria_label = Some("Range picker".to_string());
    div().w(320.0).child(js_date_range_picker(&spec, theme))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
