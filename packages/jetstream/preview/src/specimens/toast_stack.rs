//! ToastStack specimen — notification toast stack.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::toast_stack::js_toast_stack;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{Toast, ToastStackSpec, ToastTone};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let toasts = vec![
        Toast::new("1", "File saved").with_message("Changes saved successfully.").with_tone(ToastTone::Success),
        Toast::new("2", "Warning").with_message("Disk space running low.").with_tone(ToastTone::Warning),
        Toast::new("3", "Error").with_message("Upload failed.").with_tone(ToastTone::Danger),
    ];

    div().flex_col().gap(24.0)
        .child(group("Multiple toasts", secondary,
            js_toast_stack(
                &ToastStackSpec::new().with_toasts(toasts),
                theme,
            )
        ))
        .child(group("Single info toast", secondary,
            js_toast_stack(
                &ToastStackSpec::new().add_toast(
                    Toast::new("4", "Info").with_message("Processing complete.")
                ),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
