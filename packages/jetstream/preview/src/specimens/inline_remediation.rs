//! InlineRemediation specimen — exercises the Jetstream `js_inline_remediation`
//! composite across the contract anatomy and states (§2/§4): tone variants
//! (info / warning / danger), actionless messaging, an action delegating to a
//! real Button, referenced-field attribution, and a disabled action.
//!
//! Real components only (CLAUDE.md "no fakes"): every group builds a real
//! `js_inline_remediation` from `InlineRemediationSpec`. The tone-colored left
//! border resolves from `border_token()`, the title/message/hint from spec
//! tokens, and the optional action delegates to the real `js_button` primitive
//! via `RemediationAction`. The specimen adds no chrome beyond the eyebrow group
//! label.

use crate::compat::js_inline_remediation;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ButtonVariant, InlineRemediationSpec, RemediationAction, StatusTone};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // ── Tones (info / warning / danger) ──
        .child(group(
            "Tones",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(js_inline_remediation(
                    &InlineRemediationSpec::new("Double-check this value before continuing.")
                        .with_tone(StatusTone::Info)
                        .with_title("Info"),
                    theme,
                ))
                .child(js_inline_remediation(
                    &InlineRemediationSpec::new(
                        "This setting may slow down syncing on large workspaces.",
                    )
                    .with_tone(StatusTone::Warning)
                    .with_title("Warning"),
                    theme,
                ))
                .child(js_inline_remediation(
                    &InlineRemediationSpec::new("The end date must fall after the start date.")
                        .with_tone(StatusTone::Danger)
                        .with_title("Danger"),
                    theme,
                )),
        ))
        // ── Actionless (pure messaging) ──
        .child(group(
            "Actionless",
            secondary,
            js_inline_remediation(
                &InlineRemediationSpec::new("Slugs must be lowercase and contain no spaces.")
                    .with_tone(StatusTone::Info),
                theme,
            ),
        ))
        // ── With action (delegates to Button primitive) ──
        .child(group(
            "With action",
            secondary,
            js_inline_remediation(
                &InlineRemediationSpec::new("Upload failed — the file may be too large.")
                    .with_tone(StatusTone::Danger)
                    .with_title("Upload failed")
                    .with_action(
                        RemediationAction::new("retry", "Retry")
                            .with_variant(ButtonVariant::Secondary),
                    ),
                theme,
            ),
        ))
        // ── Referenced fields (cross-field attribution hint) ──
        .child(group(
            "Referenced fields",
            secondary,
            js_inline_remediation(
                &InlineRemediationSpec::new("Password and confirmation must match.")
                    .with_tone(StatusTone::Warning)
                    .with_title("Mismatch")
                    .with_referenced_field_ids(vec![
                        "password".to_string(),
                        "confirm-password".to_string(),
                    ])
                    .with_action(
                        RemediationAction::new("clear", "Clear both")
                            .with_variant(ButtonVariant::Ghost),
                    ),
                theme,
            ),
        ))
        // ── Disabled action ──
        .child(group(
            "Disabled action",
            secondary,
            js_inline_remediation(
                &InlineRemediationSpec::new("Reconnect to retry — currently offline.")
                    .with_tone(StatusTone::Danger)
                    .with_title("Offline")
                    .with_action(
                        RemediationAction::new("retry", "Retry")
                            .with_variant(ButtonVariant::Secondary)
                            .with_disabled(true),
                    ),
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
