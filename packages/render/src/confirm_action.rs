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

use poodle_node::Node;
use poodle_specs::{
    AlertDialogSpec, AlertDialogTone, ButtonSpec, ButtonTone, ButtonVariant, ConfirmActionSpec,
    StatusTone,
};

use crate::alert_dialog::{alert_dialog_with_content, AlertDialogHandlers};
use crate::button::button;
use crate::context::RenderContext;

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
    ctx: &RenderContext<'_>,
    handlers: ConfirmActionHandlers,
) -> Node {
    confirm_action_with_slots(spec, ctx, None, None, handlers)
}

/// Render with optional trigger and dialog-body slots. The slots remain nodes,
/// so every backend sees the same composed structure.
pub fn confirm_action_with_slots(
    spec: &ConfirmActionSpec,
    ctx: &RenderContext<'_>,
    trigger: Option<Node>,
    content: Option<Node>,
    handlers: ConfirmActionHandlers,
) -> Node {
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    if !spec.is_open {
        if let Some(trigger) = trigger {
            return trigger;
        }
        // Closed: render the default trigger — a composed secondary button with
        // the derived tone (contract §2 DefaultTrigger). All button visuals
        // (height, padding, fill, border, radius, focus) resolve via button.
        let trigger_spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Secondary)
            .with_tone(trigger_button_tone(spec))
            .with_size(base_size)
            .with_size_role(spec.size_role)
            .with_density(density)
            .with_label(spec.trigger_label.clone());
        return button(&trigger_spec, ctx, handlers.on_trigger);
    }

    // Open: delegate to the composed alert_dialog primitive (dialog + buttons).
    let alert_spec = AlertDialogSpec::new(spec.title.clone())
        .with_description(spec.description.clone())
        .with_tone(alert_tone(spec))
        .with_confirm_label(spec.confirm_label.clone())
        .with_cancel_label(spec.cancel_label.clone())
        .with_open(true)
        .with_size(base_size)
        .with_size_role(spec.size_role)
        .with_density(density);

    let dialog = alert_dialog_with_content(
        &alert_spec,
        ctx,
        false,
        "Working\u{2026}",
        content.into_iter().collect(),
        AlertDialogHandlers {
            confirm: handlers.on_confirm,
            cancel: handlers.on_cancel,
        },
    );

    if let Some(trigger) = trigger {
        Node::container().child(trigger).child(dialog)
    } else {
        dialog
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn open_custom_slots_and_actions_stay_live() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let confirm_seen = Arc::clone(&seen);
        let cancel_seen = Arc::clone(&seen);
        let spec =
            ConfirmActionSpec::new("Delete?", "Permanent.", "Delete", "Cancel").with_open(true);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = confirm_action_with_slots(
            &spec,
            &ctx,
            Some(Node::button("Custom trigger")),
            Some(Node::text("Typed confirmation")),
            ConfirmActionHandlers {
                on_trigger: None,
                on_confirm: Some(Arc::new(move || {
                    confirm_seen.lock().unwrap().push("confirm")
                })),
                on_cancel: Some(Arc::new(move || cancel_seen.lock().unwrap().push("cancel"))),
            },
        );

        assert!(node.has_text("Custom trigger"));
        assert!(node.has_text("Typed confirmation"));
        let confirm = node
            .find(&|node| {
                matches!(&node.kind, poodle_node::NodeKind::Button { label } if label == "Delete")
            })
            .expect("confirm button");
        let cancel = node
            .find(&|node| {
                matches!(&node.kind, poodle_node::NodeKind::Button { label } if label == "Cancel")
            })
            .expect("cancel button");
        (confirm.interaction.on_activate.as_ref().unwrap())();
        (cancel.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), ["confirm", "cancel"]);
    }
}
