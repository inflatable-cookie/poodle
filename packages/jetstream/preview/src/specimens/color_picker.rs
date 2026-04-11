//! ColorPicker specimen — with color, without, disabled.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::color_picker::js_color_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::ColorPickerSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // With a selected color
        .child(group("With color", secondary,
            js_color_picker(&ColorPickerSpec::new().with_default_value("#3b82f6"), theme)
        ))
        // No color selected
        .child(group("No color", secondary,
            js_color_picker(&ColorPickerSpec::new(), theme)
        ))
        // With swatches
        .child(group("With swatches", secondary,
            js_color_picker(
                &ColorPickerSpec::new()
                    .with_default_value("#ef4444")
                    .with_swatches(vec![
                        "#ef4444".into(), "#f97316".into(), "#eab308".into(),
                        "#22c55e".into(), "#3b82f6".into(), "#8b5cf6".into(),
                    ]),
                theme,
            )
        ))
        // Disabled
        .child(group("Disabled", secondary,
            js_color_picker(&ColorPickerSpec::new().with_default_value("#8b5cf6").with_disabled(true), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
