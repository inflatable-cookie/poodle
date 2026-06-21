//! ConfirmAction specimen — trigger + confirm dialog, tone-driven.
//!
//! Covers the contract states the real `js_confirm_action` API can render
//! honestly: the closed default trigger (a composed secondary Button with the
//! derived tone) and the open confirmation dialog (delegated entirely to
//! `js_alert_dialog` — Dialog + tone-driven cancel/confirm Buttons). Both
//! danger and default (warning) tones are shown in each state.
//!
//! `js_confirm_action` exposes neither a custom-trigger slot nor an arbitrary
//! body element (those are AlertDialog-side capabilities — see the parity doc),
//! so this specimen demonstrates the tone/open-state matrix that the component
//! actually supports rather than faking the unsupported groups.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::confirm_action::js_confirm_action;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::ConfirmActionSpec;
use poodle_specs::StatusTone;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Default trigger (danger) — closed: danger-toned secondary Button.
        .child(group(
            "Default trigger (danger)",
            secondary,
            js_confirm_action(
                &ConfirmActionSpec::new(
                    "Delete this record?",
                    "This record will be permanently removed.",
                    "Delete",
                    "Cancel",
                )
                .with_tone(StatusTone::Danger)
                .with_trigger_label("Delete record"),
                theme,
            ),
        ))
        // Warning tone — closed: default-toned secondary Button.
        .child(group(
            "Warning tone (trigger)",
            secondary,
            js_confirm_action(
                &ConfirmActionSpec::new(
                    "Archive this project?",
                    "The project will be moved to the archive and can be restored later.",
                    "Archive",
                    "Cancel",
                )
                .with_tone(StatusTone::Warning)
                .with_trigger_label("Archive project"),
                theme,
            ),
        ))
        // Confirm dialog (danger) — open: danger-toned confirm Button.
        .child(group(
            "Confirm dialog (danger)",
            secondary,
            js_confirm_action(
                &ConfirmActionSpec::new(
                    "Delete this record?",
                    "This record will be permanently removed.",
                    "Delete",
                    "Cancel",
                )
                .with_tone(StatusTone::Danger)
                .with_trigger_label("Delete record")
                .with_open(true),
                theme,
            ),
        ))
        // Confirm dialog (default) — open: default (accent) confirm Button.
        .child(group(
            "Confirm dialog (default)",
            secondary,
            js_confirm_action(
                &ConfirmActionSpec::new(
                    "Archive this project?",
                    "The project will be moved to the archive and can be restored later.",
                    "Archive",
                    "Cancel",
                )
                .with_tone(StatusTone::Warning)
                .with_trigger_label("Archive project")
                .with_open(true),
                theme,
            ),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
