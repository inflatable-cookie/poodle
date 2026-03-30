//! ConfirmAction specimen — confirmation trigger button (closed state).

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::confirm_action::js_confirm_action;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::ConfirmActionSpec;
use poodle_primitives::StatusTone;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Neutral (closed)", secondary,
            js_confirm_action(
                &ConfirmActionSpec::new(
                    "Archive project",
                    "This project will be moved to the archive.",
                    "Archive",
                    "Cancel",
                ),
                theme,
            )
        ))
        .child(group("Destructive (closed)", secondary,
            js_confirm_action(
                &ConfirmActionSpec::new(
                    "Delete permanently",
                    "This action cannot be undone.",
                    "Delete",
                    "Cancel",
                )
                .with_tone(StatusTone::Danger),
                theme,
            )
        ))
        .child(group("Open state", secondary,
            js_confirm_action(
                &ConfirmActionSpec::new(
                    "Publish changes",
                    "All pending changes will go live.",
                    "Publish",
                    "Cancel",
                )
                .with_open(true),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
