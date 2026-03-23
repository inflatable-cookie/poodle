//! AlertDialog specimen — alert dialogs with different tones and content.

use jetstream_runtime::ui_element::*;
use flint_jetstream::JetstreamThemeProvider;
use flint_jetstream_components::alert_dialog::js_alert_dialog;
use flint_jetstream_components::theme_ext::*;
use flint_primitives::{AlertDialogSpec, AlertDialogTone};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Danger tone (default)
        .child(group("Danger tone", secondary,
            js_alert_dialog(
                &AlertDialogSpec::new("Delete Item")
                    .with_description("This action cannot be undone. Are you sure?"),
                theme,
            )
        ))
        // Warning tone
        .child(group("Warning tone", secondary,
            js_alert_dialog(
                &AlertDialogSpec::new("Unsaved Changes")
                    .with_description("You have unsaved changes. Discard them?")
                    .with_tone(AlertDialogTone::Warning)
                    .with_confirm_label("Discard")
                    .with_cancel_label("Keep Editing"),
                theme,
            )
        ))
        // Minimal (no description)
        .child(group("No description", secondary,
            js_alert_dialog(
                &AlertDialogSpec::new("Confirm logout"),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
