//! PickerShell specimen — generic picker container.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::picker_shell::js_picker_shell;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::PickerShellSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        .child(group("With content", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Select an item"),
                theme,
                Some(
                    div().flex_col().gap(4.0)
                        .child(label("Option A").text_color(text_primary).text_size(13.0))
                        .child(label("Option B").text_color(text_primary).text_size(13.0))
                        .child(label("Option C").text_color(text_primary).text_size(13.0))
                ),
            )
        ))
        .child(group("Empty", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Choose item"),
                theme,
                None,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
