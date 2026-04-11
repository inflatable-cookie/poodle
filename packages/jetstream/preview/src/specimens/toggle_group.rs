//! ToggleGroup specimen — single/multiple selection, disabled items.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::toggle_group::js_toggle_group;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let options = vec![
        ToggleGroupOption::new("bold", "B"),
        ToggleGroupOption::new("italic", "I"),
        ToggleGroupOption::new("underline", "U"),
    ];

    // Single selection
    div().flex_col().gap(24.0)
        .child(group("Single selection", secondary,
            js_toggle_group(
                &ToggleGroupSpec::new(options.clone())
                    .with_selection_mode(ToggleGroupSelectionMode::Single)
                    .with_value(vec!["bold".into()]),
                theme,
            )
        ))
        // Multiple selection
        .child(group("Multiple selection", secondary,
            js_toggle_group(
                &ToggleGroupSpec::new(options.clone())
                    .with_selection_mode(ToggleGroupSelectionMode::Multiple)
                    .with_value(vec!["bold".into(), "underline".into()]),
                theme,
            )
        ))
        // Disabled items
        .child(group("With disabled item", secondary, {
            let mut opts = options.clone();
            opts[2] = ToggleGroupOption::new("underline", "U").with_disabled(true);
            js_toggle_group(
                &ToggleGroupSpec::new(opts)
                    .with_value(vec!["italic".into()]),
                theme,
            )
        }))
        // Fully disabled
        .child(group("Fully disabled", secondary,
            js_toggle_group(
                &ToggleGroupSpec::new(options)
                    .with_value(vec!["bold".into()])
                    .with_disabled(true),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
