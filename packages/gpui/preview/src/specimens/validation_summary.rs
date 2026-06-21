//! ValidationSummary specimen — exercises the GPUI `ValidationSummary`
//! composite across its contract states (§4): blocking, pending-only, mixed,
//! and empty.
//!
//! Each group builds a real `ValidationSummary` from `ValidationSummarySpec`.
//! The surface border resolves from `border_token()` (danger when any entry is
//! blocking, accent when pending-only); every entry row paints a tone dot plus
//! the field label and message resolved from spec tokens. The empty group
//! supplies only valid entries so `active_entries()` is empty and the component
//! renders nothing — proven by the placeholder note beside it.

use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, ValidationSummary};
use poodle_specs::{
    EyebrowSpec, ValidationState, ValidationSummaryEntry, ValidationSummarySpec,
};

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(480.0))
        // ── Blocking (one or more invalid → danger border) ──────
        .child(group(
            theme,
            "Blocking (danger border)",
            ValidationSummary::from_spec(
                ValidationSummarySpec::new(vec![
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
            )
            .into_any_element(),
        ))
        // ── Pending-only (includePending → accent border) ───────
        .child(group(
            theme,
            "Pending only (accent border)",
            ValidationSummary::from_spec(
                ValidationSummarySpec::new(vec![ValidationSummaryEntry::new(
                    "username",
                    "Username",
                    "Checking availability…",
                    ValidationState::Pending,
                )])
                .with_include_pending(true)
                .with_title("Validating"),
                theme,
            )
            .into_any_element(),
        ))
        // ── Mixed (invalid + pending → danger precedence) ───────
        .child(group(
            theme,
            "Mixed (danger precedence)",
            ValidationSummary::from_spec(
                ValidationSummarySpec::new(vec![
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
            )
            .into_any_element(),
        ))
        // ── Untitled (no heading) ───────────────────────────────
        .child(group(
            theme,
            "Untitled",
            ValidationSummary::from_spec(
                ValidationSummarySpec::new(vec![ValidationSummaryEntry::new(
                    "terms",
                    "Terms",
                    "You must accept the terms to continue",
                    ValidationState::Invalid,
                )]),
                theme,
            )
            .into_any_element(),
        ))
        // ── Empty (no active entries → renders nothing) ─────────
        .child(empty_group(theme))
}

/// A labelled specimen group: eyebrow + the supplied summary element.
fn group(theme: &GpuiThemeProvider, label: &str, summary: AnyElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(EyebrowSpec::new().with_content(label), theme))
        .child(summary)
}

/// The empty state: only valid entries, so `active_entries()` is empty and the
/// component renders nothing. A muted placeholder note makes the absence legible
/// in the specimen (the note is specimen chrome, not part of the component).
fn empty_group(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content("Empty (renders nothing)"),
            theme,
        ))
        .child(
            ValidationSummary::from_spec(
                ValidationSummarySpec::new(vec![ValidationSummaryEntry::new(
                    "email",
                    "Email",
                    "",
                    ValidationState::Valid,
                )]),
                theme,
            )
            .into_any_element(),
        )
        .child(
            div()
                .text_sm()
                .text_color(color_to_hsla(text_secondary))
                .child("(no active entries — component renders nothing)"),
        )
}
