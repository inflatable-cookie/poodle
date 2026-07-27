//! AlertDialog — composes the real `js_dialog` primitive (surface + overlay +
//! backdrop) with two composed `js_button`s for the cancel/confirm actions,
//! mirroring the Svelte reference (`AlertDialog.svelte`) and the GPUI rebuild
//! (`packages/gpui/components/src/primitives/alert_dialog.rs`).
//!
//! The component owns only the alert-specific content (title/description/optional
//! item-detail row) and the tone→confirm-Button mapping; everything visual
//! (width, shadow, padding, radius, backdrop fill, the divider above actions) is
//! delegated to the composed Dialog/Button primitives so it resolves entirely
//! from tokens and stays in lock-step with their contracts.
//!
//! Svelte/contract parity notes:
//! - Surface: `Dialog role="alertdialog" width="sm"` (contract §2 / §8 passthrough).
//! - Confirm tone: `tone === "danger" ? "danger" : "default"` — warning maps to
//!   the default (accent) Button tone (contract §8 confirm-button mapping). This
//!   is realized by feeding the confirm `js_button` `ButtonTone::Danger` for the
//!   danger tone and `ButtonTone::Default` for warning.
//! - Working state gates dismiss/close and disables both buttons while swapping
//!   the confirm label for `working_label` (contract §4 working state).
//!
//! Preview-loop / accepted limits:
//! - Confirm/cancel click wiring and the escape/backdrop dismiss routes live in
//!   the preview event loop, not the component (matches the existing Jetstream
//!   pattern — `js_button` exposes `.focusable()` only here).
//! - No ARIA channel: `alertdialog` role / `aria-modal` / `aria-label` are carried
//!   on the spec but not emitted (tracked repo-wide for Jetstream).

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    AlertDialogSpec, AlertDialogTone, ButtonSpec, ButtonTone, ButtonVariant, DialogKind,
    DialogSpec, DialogWidth,
};

use crate::button::js_button;
use crate::dialog::js_dialog;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::resolve_color;

/// Default working label shown on the confirm button while `working` is true,
/// matching the contract default `"Working…"`.
const DEFAULT_WORKING_LABEL: &str = "Working\u{2026}";

/// Svelte: `confirmTone = tone === "danger" ? "danger" : "default"`.
fn confirm_button_tone(tone: AlertDialogTone) -> ButtonTone {
    match tone {
        AlertDialogTone::Danger => ButtonTone::Danger,
        AlertDialogTone::Warning => ButtonTone::Default,
    }
}

/// Render an AlertDialog at its default (non-working) state.
pub fn js_alert_dialog(spec: &AlertDialogSpec, theme: &JetstreamThemeProvider) -> JsEl {
    js_alert_dialog_working(spec, theme, false, DEFAULT_WORKING_LABEL)
}

/// Render an AlertDialog, optionally in the working state.
///
/// While `working` is true the confirm label is swapped for `working_label`,
/// both buttons render disabled (dimmed + non-interactive), and the composed
/// Dialog's escape/backdrop/close dismissal is suppressed (contract §4).
pub fn js_alert_dialog_working(
    spec: &AlertDialogSpec,
    theme: &JetstreamThemeProvider,
    working: bool,
    working_label: &str,
) -> JsEl {
    let size = spec.size;
    let size_role = spec.size_role;
    let density = spec.density;

    // ── Cancel button (ghost) — Svelte AlertDialog cancel action ──
    let cancel_spec = ButtonSpec::new()
        .with_variant(ButtonVariant::Ghost)
        .with_size(size)
        .with_size_role(size_role)
        .with_density(density)
        .with_label(spec.cancel_label.clone())
        .with_disabled(working);
    let cancel_btn = js_button(&cancel_spec, theme);

    // ── Confirm button (primary, tone-driven) ──
    // Working swaps the label for `working_label` (contract §4).
    let confirm_text = if working {
        working_label.to_string()
    } else {
        spec.confirm_label.clone()
    };
    let confirm_spec = ButtonSpec::new()
        .with_variant(ButtonVariant::Primary)
        .with_tone(confirm_button_tone(spec.tone))
        .with_size(size)
        .with_size_role(size_role)
        .with_density(density)
        .with_label(confirm_text)
        .with_disabled(working);
    let confirm_btn = js_button(&confirm_spec, theme);

    // ── Actions row: cancel left, confirm right (delegated gap via Dialog) ──
    let actions_gap = crate::theme_ext::resolve_px(theme, spec.actions_gap_token());
    let actions = ui_element::div()
        .flex_row()
        .flex_wrap()
        .justify_end()
        .items_center()
        .gap(actions_gap)
        .child(cancel_btn)
        .child(confirm_btn);

    // ── Body: optional item-detail row (itemLabel/itemValue) ──
    // Contract §8: text-secondary body, strong (label) in text-primary.
    let mut children: Vec<JsEl> = Vec::new();
    if let (Some(label), Some(value)) = (spec.item_label.as_ref(), spec.item_value.as_ref()) {
        let effective_size = resolve_semantic_size(size, size_role);
        let body_font = rem_to_px(size_font_rem(effective_size));
        let text_secondary: Color = resolve_color(theme, spec.description_color_token()).into();
        let text_primary: Color = resolve_color(theme, spec.title_color_token()).into();
        let detail_gap = crate::theme_ext::resolve_px(theme, spec.actions_gap_token());
        children.push(
            ui_element::div()
                .flex_row()
                .items_center()
                .gap(detail_gap)
                .child(
                    ui_element::label(&format!("{label}:"))
                        .text_color(text_primary)
                        .text_size(body_font)
                        .text_weight(600),
                )
                .child(
                    ui_element::label(value)
                        .text_color(text_secondary)
                        .text_size(body_font),
                ),
        );
    }

    // ── Compose the real Dialog primitive for the surface/overlay ──
    // role=AlertDialog, width=Sm; dismiss/close gated by working (contract §8).
    let mut dialog_spec = DialogSpec::new()
        .with_role(DialogKind::AlertDialog)
        .with_width(DialogWidth::Sm)
        .with_open(true)
        .with_size(size)
        .with_size_role(size_role)
        .with_density(density)
        .with_dismiss_on_escape(!working)
        .with_dismiss_on_backdrop(!working)
        .with_show_close_button(!working);

    if !spec.title.is_empty() {
        dialog_spec = dialog_spec.with_title(spec.title.clone());
    }
    if let Some(ref desc) = spec.description {
        dialog_spec = dialog_spec.with_description(desc.clone());
    }
    if let Some(ref aria) = spec.aria_label {
        dialog_spec = dialog_spec.with_aria_label(aria.clone());
    }

    // Contract: an alert dialog is `role="alertdialog"`, not a plain dialog —
    // it interrupts, and assistive technology treats the two differently. The
    // shared `js_dialog` cannot know which it is building, so the role is
    // overridden here on the way out.
    js_dialog(&dialog_spec, theme, children, Some(actions))
        .aria_role(jetstream_ui::accesskit::Role::AlertDialog)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> AlertDialogSpec {
        AlertDialogSpec::new("Delete this item?")
            .with_description("This action cannot be undone.")
            .with_confirm_label("Delete")
            .with_cancel_label("Keep it")
            .with_open(true)
    }

    #[test]
    fn renders_title_and_both_action_labels() {
        let th = theme();
        let el = js_alert_dialog(&spec(), &th);
        let tree = probe(&el, 480.0, 320.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(
            tree.has_text("Delete this item?"),
            "title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Keep it"),
            "cancel label missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Delete"),
            "confirm label missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn working_state_swaps_the_confirm_label() {
        let th = theme();
        let el = js_alert_dialog_working(&spec(), &th, true, "Deleting\u{2026}");
        let tree = probe(&el, 480.0, 320.0);

        assert!(
            tree.has_text("Deleting\u{2026}"),
            "working label not rendered: {:?}",
            tree.texts()
        );
        assert!(
            !tree.has_text("Delete"),
            "confirm label should be replaced by the working label: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn item_detail_row_renders() {
        let th = theme();
        let mut s = spec();
        s.item_label = Some("User".to_string());
        s.item_value = Some("Ada Lovelace".to_string());
        let el = js_alert_dialog(&s, &th);
        let tree = probe(&el, 480.0, 320.0);

        assert!(
            tree.has_text("User:"),
            "item-detail label missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Ada Lovelace"),
            "item-detail value missing: {:?}",
            tree.texts()
        );
    }
}
