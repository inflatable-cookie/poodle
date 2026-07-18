//! DatePicker specimen — trigger button composing a Calendar overlay.
//!
//! Every group renders the real `js_date_picker` builder; trigger, indicator,
//! and (when open) the composed Calendar surface all resolve from
//! `DatePickerSpec` + tokens. Specimens render static state, so the open flag
//! is seeded directly on the spec.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::date_picker::js_date_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlDensity, ControlSize, DatePickerSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Closed, no value — placeholder text in secondary color.
        .child(group(
            "Default",
            secondary,
            js_date_picker(
                &{
                    let mut spec = DatePickerSpec::new();
                    spec.aria_label = Some("Select date".to_string());
                    spec
                },
                theme,
            ),
        ))
        // Closed, with value — formatted date in primary color.
        .child(group(
            "With default value",
            secondary,
            js_date_picker(
                &{
                    let mut spec = DatePickerSpec::new().with_default_value("2026-03-14");
                    spec.aria_label = Some("Pre-filled date".to_string());
                    spec
                },
                theme,
            ),
        ))
        // Open — surface visible, composes the real Calendar surface.
        .child(group(
            "Open",
            secondary,
            js_date_picker(
                &{
                    let mut spec = DatePickerSpec::new()
                        .with_default_value("2026-03-14")
                        .with_default_open(true);
                    spec.aria_label = Some("Open date picker".to_string());
                    spec
                },
                theme,
            ),
        ))
        // Disabled — reduced opacity, non-interactive trigger.
        .child(group(
            "Disabled",
            secondary,
            js_date_picker(
                &{
                    let mut spec = DatePickerSpec::new();
                    spec.placeholder = "Disabled".to_string();
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled date picker".to_string());
                    spec
                },
                theme,
            ),
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
    let mut spec = DatePickerSpec::new().with_size(size).with_default_value("2026-03-14");
    spec.aria_label = Some("Date picker".to_string());
    js_date_picker(&spec, theme)
}

fn dense_picker(theme: &JetstreamThemeProvider, density: ControlDensity) -> JsEl {
    let mut spec = DatePickerSpec::new()
        .with_density(density)
        .with_default_value("2026-03-14");
    spec.aria_label = Some("Date picker".to_string());
    js_date_picker(&spec, theme)
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
