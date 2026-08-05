//! ConfirmAction — a trigger that opens a confirm dialog.
//!
//! Contract: `docs/contracts/components/confirm-action.md`
//! Ported from: `packages/jetstream/components/src/confirm_action.rs`.
//!
//! Owns only the trigger/tone wiring and delegates every dialog and button
//! visual to the composed primitives, so it never re-implements (and never
//! drifts from) the alert_dialog/button contracts.
//!
//! - Closed: a composed secondary `button` with derived tone
//!   (`tone === "danger" ? "danger" : "default"`).
//! - Open: delegates entirely to `alert_dialog` (surface/overlay/backdrop +
//!   cancel/confirm buttons). `on_cancel` covers the cancel button and every
//!   dismissal route, as alert_dialog does.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::Node;
use poodle_specs::{
    AlertDialogSpec, AlertDialogTone, ButtonSpec, ButtonTone, ButtonVariant, ConfirmActionSpec,
    StatusTone,
};

use crate::alert_dialog::{alert_dialog, AlertDialogHandlers};
use crate::button::button;

/// Host callbacks: trigger (closed state), confirm and cancel (open state).
#[derive(Default)]
pub struct ConfirmActionHandlers {
    pub on_trigger: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_confirm: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

/// Svelte: `triggerTone = tone === "danger" ? "danger" : "default"`.
fn trigger_button_tone(spec: &ConfirmActionSpec) -> ButtonTone {
    if spec.is_destructive() {
        ButtonTone::Danger
    } else {
        ButtonTone::Default
    }
}

/// Map the ConfirmAction `StatusTone` onto the `AlertDialogTone` the composed
/// alert_dialog accepts. AlertDialog has only `Danger | Warning`; non-danger →
/// `Warning`, which resolves the confirm button to the default (accent) tone —
/// matching Svelte/GPUI where non-danger tones map the confirm button to
/// `default`.
fn alert_tone(spec: &ConfirmActionSpec) -> AlertDialogTone {
    match spec.tone {
        StatusTone::Danger => AlertDialogTone::Danger,
        _ => AlertDialogTone::Warning,
    }
}

pub fn confirm_action(
    spec: &ConfirmActionSpec,
    theme: &dyn ThemeProvider,
    handlers: ConfirmActionHandlers,
) -> Node {
    if !spec.is_open {
        // Closed: render the default trigger — a composed secondary button with
        // the derived tone (contract §2 DefaultTrigger). All button visuals
        // (height, padding, fill, border, radius, focus) resolve via button.
        let trigger_spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Secondary)
            .with_tone(trigger_button_tone(spec))
            .with_size(spec.size)
            .with_size_role(spec.size_role)
            .with_density(spec.density)
            .with_label(spec.trigger_label.clone());
        return button(&trigger_spec, theme, handlers.on_trigger);
    }

    // Open: delegate to the composed alert_dialog primitive (dialog + buttons).
    let alert_spec = AlertDialogSpec::new(spec.title.clone())
        .with_description(spec.description.clone())
        .with_tone(alert_tone(spec))
        .with_confirm_label(spec.confirm_label.clone())
        .with_cancel_label(spec.cancel_label.clone())
        .with_open(true)
        .with_size(spec.size)
        .with_size_role(spec.size_role)
        .with_density(spec.density);

    alert_dialog(
        &alert_spec,
        theme,
        false,
        "Working\u{2026}",
        AlertDialogHandlers {
            confirm: handlers.on_confirm,
            cancel: handlers.on_cancel,
        },
    )
}
