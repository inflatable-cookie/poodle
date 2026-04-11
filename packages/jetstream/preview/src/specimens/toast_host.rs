//! ToastHost specimen — positioned toast container.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::toast_host::js_toast_host;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{Toast, ToastHostPlacement, ToastHostSpec, ToastStackSpec, ToastTone};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let toasts = vec![
        Toast::new("1", "Saved").with_message("Changes saved.").with_tone(ToastTone::Success),
        Toast::new("2", "Warning").with_message("Disk space low.").with_tone(ToastTone::Warning),
    ];

    div().flex_col().gap(24.0)
        .child(group("Bottom-end (default)", secondary,
            js_toast_host(
                &ToastHostSpec::new(),
                theme,
                &ToastStackSpec::new().with_toasts(toasts.clone()),
            )
        ))
        .child(group("Top-start", secondary,
            js_toast_host(
                &ToastHostSpec::new()
                    .with_placement(ToastHostPlacement::TopStart),
                theme,
                &ToastStackSpec::new().with_toasts(toasts),
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
