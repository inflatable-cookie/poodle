//! ValidationSummary specimen — exercises the Jetstream `js_validation_summary`
//! composite across its contract states (§4): blocking, pending-only, mixed,
//! untitled, and empty.
//!
//! Real components only (CLAUDE.md "no fakes"): every group builds a real
//! `js_validation_summary` from `ValidationSummarySpec`. The surface border
//! resolves from `border_token()` (danger when any entry is blocking, accent
//! when pending-only); each entry row paints a tone dot plus the field label and
//! message resolved from spec tokens. The empty group supplies only valid entries
//! so `active_entries()` is empty and the component renders nothing — proven by
//! the muted placeholder note (specimen chrome, not part of the component).

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::theme_ext::resolve_color;
use poodle_jetstream_components::validation_summary::js_validation_summary;
use poodle_specs::{ValidationState, ValidationSummaryEntry, ValidationSummarySpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .w(480.0)
        // ── Blocking (one or more invalid → danger border) ──────
        .child(group(
            "Blocking (danger border)",
            secondary,
            js_validation_summary(
                &ValidationSummarySpec::new(vec![
                    ValidationSummaryEntry::new(
                        "email",
                        "Email",
                        "Enter a valid email address",
                        ValidationState::Invalid,
                    ),
                    ValidationSummaryEntry::new(
                        "password",
                        "Password",
                        "Must be at least 8 characters",
                        ValidationState::Invalid,
                    ),
                ])
                .with_title("Please fix the following"),
                theme,
            ),
        ))
        // ── Pending-only (includePending → accent border) ───────
        .child(group(
            "Pending only (accent border)",
            secondary,
            js_validation_summary(
                &ValidationSummarySpec::new(vec![ValidationSummaryEntry::new(
                    "username",
                    "Username",
                    "Checking availability…",
                    ValidationState::Pending,
                )])
                .with_include_pending(true)
                .with_title("Validating"),
                theme,
            ),
        ))
        // ── Mixed (invalid + pending → danger precedence) ───────
        .child(group(
            "Mixed (danger precedence)",
            secondary,
            js_validation_summary(
                &ValidationSummarySpec::new(vec![
                    ValidationSummaryEntry::new(
                        "email",
                        "Email",
                        "Enter a valid email address",
                        ValidationState::Invalid,
                    ),
                    ValidationSummaryEntry::new(
                        "username",
                        "Username",
                        "Checking availability…",
                        ValidationState::Pending,
                    ),
                ])
                .with_include_pending(true)
                .with_title("Please fix the following"),
                theme,
            ),
        ))
        // ── Untitled (no heading) ───────────────────────────────
        .child(group(
            "Untitled",
            secondary,
            js_validation_summary(
                &ValidationSummarySpec::new(vec![ValidationSummaryEntry::new(
                    "terms",
                    "Terms",
                    "You must accept the terms to continue",
                    ValidationState::Invalid,
                )]),
                theme,
            ),
        ))
        // ── Empty (no active entries → renders nothing) ─────────
        .child(empty_group(theme, secondary))
}

/// The empty state: only valid entries, so `active_entries()` is empty and the
/// component renders nothing. A muted placeholder note makes the absence legible
/// (the note is specimen chrome, not part of the component).
fn empty_group(theme: &JetstreamThemeProvider, secondary: glam::Vec4) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(
            label("Empty (renders nothing)")
                .text_color(secondary)
                .text_size(11.0),
        )
        .child(js_validation_summary(
            &ValidationSummarySpec::new(vec![ValidationSummaryEntry::new(
                "email",
                "Email",
                "",
                ValidationState::Valid,
            )]),
            theme,
        ))
        .child(
            label("(no active entries — component renders nothing)")
                .text_color(secondary)
                .text_size(12.0),
        )
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
