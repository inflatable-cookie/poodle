//! ConfirmAction — Jetstream confirm action backed by ConfirmActionSpec.
//!
//! Contract: `docs/contracts/components/confirm-action.md`
//!
//! Mirrors the Svelte reference (`ConfirmAction.svelte`) and the GPUI composite
//! (`packages/gpui/components/src/composites/confirm_action.rs`): the component
//! owns only the trigger/tone wiring and delegates every dialog and button
//! visual to the composed primitives, so it never re-implements (and never
//! drifts from) the AlertDialog/Button contracts.
//!
//! - Default trigger: a composed secondary `js_button` with derived tone
//!   (`tone === "danger" ? "danger" : "default"`, Svelte line 48).
//! - Open dialog: delegates entirely to `js_alert_dialog`, which composes the
//!   real Dialog primitive (surface/overlay/backdrop) + cancel/confirm Buttons.
//!
//! Preview-loop / accepted limits:
//! - Trigger activation, confirm/cancel clicks and backdrop/escape dismiss live
//!   in the preview event loop, not the component (the composed Button exposes
//!   `.focusable()` here; the composed Dialog gates dismiss internally).
//! - The contract §3 `children` body slot maps to the AlertDialog item-detail
//!   row; an arbitrary body element is an AlertDialog-side capability and is
//!   noted as a remaining limit rather than hand-rolled here.

use jetstream_runtime::ui_element::JsEl;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    AlertDialogSpec, AlertDialogTone, ButtonSpec, ButtonTone, ButtonVariant, ConfirmActionSpec,
    StatusTone,
};

use crate::alert_dialog::js_alert_dialog;
use crate::button::js_button;

/// Svelte: `triggerTone = tone === "danger" ? "danger" : "default"`.
fn trigger_button_tone(spec: &ConfirmActionSpec) -> ButtonTone {
    if spec.is_destructive() {
        ButtonTone::Danger
    } else {
        ButtonTone::Default
    }
}

/// Map the ConfirmAction `StatusTone` onto the `AlertDialogTone` the composed
/// AlertDialog accepts. AlertDialog has only `Danger | Warning`; non-danger →
/// `Warning`, which resolves the confirm Button to the default (accent) tone —
/// matching Svelte/GPUI where non-danger tones map the confirm Button to
/// `default`.
fn alert_tone(spec: &ConfirmActionSpec) -> AlertDialogTone {
    match spec.tone {
        StatusTone::Danger => AlertDialogTone::Danger,
        _ => AlertDialogTone::Warning,
    }
}

pub fn js_confirm_action(spec: &ConfirmActionSpec, theme: &JetstreamThemeProvider) -> JsEl {
    if !spec.is_open {
        // Closed: render the default trigger — a composed secondary Button with
        // the derived tone (contract §2 DefaultTrigger). All Button visuals
        // (height, padding, fill, border, radius, focus) resolve via js_button.
        let trigger_spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Secondary)
            .with_tone(trigger_button_tone(spec))
            .with_size(spec.size)
            .with_size_role(spec.size_role)
            .with_density(spec.density)
            .with_label(spec.trigger_label.clone());
        return js_button(&trigger_spec, theme);
    }

    // Open: delegate to the composed AlertDialog primitive (Dialog + Buttons).
    let alert_spec = AlertDialogSpec::new(spec.title.clone())
        .with_description(spec.description.clone())
        .with_tone(alert_tone(spec))
        .with_confirm_label(spec.confirm_label.clone())
        .with_cancel_label(spec.cancel_label.clone())
        .with_open(true)
        .with_size(spec.size)
        .with_size_role(spec.size_role)
        .with_density(spec.density);

    js_alert_dialog(&alert_spec, theme)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ControlSize, SemanticControlSizeRole, StatusTone};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn base_spec() -> ConfirmActionSpec {
        ConfirmActionSpec::new(
            "Delete this record?",
            "This record will be permanently removed.",
            "Delete",
            "Cancel",
        )
        .with_trigger_label("Delete record")
    }

    #[test]
    fn closed_renders_trigger_label_only() {
        let th = theme();
        let spec = base_spec().with_tone(StatusTone::Danger);
        let el = js_confirm_action(&spec, &th);
        let tree = probe(&el, 480.0, 320.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(
            tree.has_text("Delete record"),
            "trigger label missing: {:?}",
            tree.texts()
        );
        // Closed state shows no dialog title / action labels.
        assert!(
            !tree.has_text("Delete this record?"),
            "dialog title leaked into closed state: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn open_renders_dialog_title_description_and_actions() {
        let th = theme();
        let spec = base_spec().with_tone(StatusTone::Danger).with_open(true);
        let el = js_confirm_action(&spec, &th);
        let tree = probe(&el, 640.0, 480.0);

        assert!(
            tree.has_text("Delete this record?"),
            "dialog title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("This record will be permanently removed."),
            "dialog description missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Delete"),
            "confirm label missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Cancel"),
            "cancel label missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn warning_tone_routes_through_alert_dialog() {
        // Warning tone: trigger is a default-toned secondary Button; the dialog
        // opens with the (non-danger) AlertDialog tone. Both paths must compose.
        let th = theme();
        let closed = ConfirmActionSpec::new("Archive this project?", "It will be archived.", "Archive", "Cancel")
            .with_tone(StatusTone::Warning)
            .with_trigger_label("Archive project");
        let closed_tree = probe(&js_confirm_action(&closed, &th), 480.0, 320.0);
        assert!(
            closed_tree.has_text("Archive project"),
            "warning trigger label missing: {:?}",
            closed_tree.texts()
        );

        let open = closed.with_open(true);
        let open_tree = probe(&js_confirm_action(&open, &th), 640.0, 480.0);
        assert!(
            open_tree.has_text("Archive this project?"),
            "warning dialog title missing: {:?}",
            open_tree.texts()
        );
        assert!(
            open_tree.has_text("Archive"),
            "warning confirm label missing: {:?}",
            open_tree.texts()
        );
    }

    #[test]
    fn size_and_density_propagate_without_panic() {
        let th = theme();
        let spec = base_spec()
            .with_size(ControlSize::Lg)
            .with_size_role(SemanticControlSizeRole::Prominent)
            .with_open(true);
        let tree = probe(&js_confirm_action(&spec, &th), 720.0, 540.0);
        assert!(tree.has_text("Delete this record?"));
    }
}
