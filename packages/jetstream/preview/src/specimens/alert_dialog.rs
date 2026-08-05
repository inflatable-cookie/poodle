//! AlertDialog specimen — full contract state coverage.
//!
//! Covers the contract §13 specimen definitions with REAL components: every
//! dialog is a `js_alert_dialog` / `js_alert_dialog_working` that composes the
//! real Dialog primitive (surface/overlay/backdrop) + tone-driven cancel/confirm
//! Buttons. No hand-coded colors, sizes, or layout — all visuals resolve from
//! tokens through the composed primitives.
//!
//! Groups:
//! - Danger tone (default): confirm Button resolves to the danger tone.
//! - Warning tone: confirm Button resolves to the default (accent) tone.
//! - Working / pending: `js_alert_dialog_working` swaps the confirm label for
//!   the working label and disables both buttons (escape/backdrop gated).
//! - With item-detail row: `with_item_detail` renders the highlighted
//!   label:value row ahead of the body (contract §2/§8 item-detail).

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::{js_alert_dialog, js_alert_dialog_working};

use poodle_specs::{AlertDialogSpec, AlertDialogTone};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Danger tone (default) — confirm Button resolves to the danger tone.
        .child(group(
            "Danger tone (default)",
            secondary,
            js_alert_dialog(
                &AlertDialogSpec::new("Delete this item?")
                    .with_description(
                        "This action cannot be undone. The item and all associated data \
                         will be permanently removed.",
                    )
                    .with_confirm_label("Delete")
                    .with_cancel_label("Keep it")
                    .with_open(true),
                theme,
            ),
        ))
        // Warning tone — confirm Button resolves to the default (accent) tone.
        .child(group(
            "Warning tone",
            secondary,
            js_alert_dialog(
                &AlertDialogSpec::new("Reset all settings?")
                    .with_description(
                        "Your customized settings will be restored to their default values.",
                    )
                    .with_tone(AlertDialogTone::Warning)
                    .with_confirm_label("Reset")
                    .with_cancel_label("Cancel")
                    .with_open(true),
                theme,
            ),
        ))
        // Working / pending — confirm label swaps + both buttons disabled.
        .child(group(
            "Working / pending state",
            secondary,
            js_alert_dialog_working(
                &AlertDialogSpec::new("Archive this project?")
                    .with_description(
                        "The project will be hidden from active lists but can still be \
                         restored later.",
                    )
                    .with_confirm_label("Archive")
                    .with_cancel_label("Cancel")
                    .with_open(true),
                theme,
                true,
                "Archiving\u{2026}",
            ),
        ))
        // With item-detail row — highlighted label:value ahead of the body.
        .child(group(
            "With item-detail row",
            secondary,
            js_alert_dialog(
                &AlertDialogSpec::new("Remove this user?")
                    .with_description("The following user will lose access immediately.")
                    .with_item_detail("User", "Ada Lovelace · ada@example.com")
                    .with_confirm_label("Remove")
                    .with_cancel_label("Cancel")
                    .with_open(true),
                theme,
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
