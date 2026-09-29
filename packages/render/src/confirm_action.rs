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
    confirm_action_with_slots_state(
        spec,
        ctx,
        trigger,
        content,
        false,
        "Working\u{2026}",
        handlers,
    )
}

/// Render with host-owned in-flight state. Native adapters use this seam to
/// rebuild the synchronous node tree while asynchronous application work is
/// pending; web runtimes keep that same state internally in AlertDialog.
pub fn confirm_action_with_slots_state(
    spec: &ConfirmActionSpec,
    ctx: &RenderContext<'_>,
    trigger: Option<Node>,
    content: Option<Node>,
    working: bool,
    working_label: &str,
    handlers: ConfirmActionHandlers,
) -> Node {
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let ConfirmActionHandlers {
        on_trigger,
        on_confirm,
        on_cancel,
    } = handlers;
    // Svelte mounts the trigger in both states: closed shows only the trigger,
    // open keeps it beside the AlertDialog. Build the default trigger once so
    // both states share the composed secondary button (contract §2
    // DefaultTrigger) — all button visuals (height, padding, fill, border,
    // radius, focus) still resolve through button. A caller's custom trigger is
    // used verbatim when supplied.
    let trigger = trigger.unwrap_or_else(|| {
        let trigger_spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Secondary)
            .with_tone(trigger_button_tone(spec))
            .with_size(base_size)
            .with_size_role(spec.size_role)
            .with_density(density)
            .with_label(spec.trigger_label.clone());
        button(&trigger_spec, ctx, on_trigger)
    });
    if !spec.is_open {
        return trigger;
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
        working,
        working_label,
        content.into_iter().collect(),
        AlertDialogHandlers {
            confirm: on_confirm,
            cancel: on_cancel,
        },
    );

    // The open dialog keeps the trigger mounted beside it. The wrapper is not
    // positioned, so the Absolute backdrop shares its parent with an in-flow
    // trigger; the node backend's overlay gate resolves that backdrop against
    // the window containing block instead of the trigger row.
    Node::container().child(trigger).child(dialog)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn open_keeps_the_default_trigger_mounted_and_live() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let trigger_seen = Arc::clone(&seen);
        let spec = ConfirmActionSpec::new("Delete?", "Permanent.", "Delete", "Cancel")
            .with_trigger_label("Reveal")
            .with_open(true);
        let theme = theme();
        let ctx = RenderContext::new(&theme);

        let node = confirm_action_with_slots_state(
            &spec,
            &ctx,
            None,
            None,
            false,
            "Working\u{2026}",
            ConfirmActionHandlers {
                on_trigger: Some(Arc::new(move || trigger_seen.lock().unwrap().push("trigger"))),
                on_confirm: None,
                on_cancel: None,
            },
        );

        // Svelte keeps the default trigger mounted beside the open AlertDialog.
        assert_eq!(
            node.children.len(),
            2,
            "open ConfirmAction composes the trigger beside the dialog"
        );
        match &node.children[0].kind {
            poodle_node::NodeKind::Button { label } => assert_eq!(label, "Reveal"),
            _ => panic!("open ConfirmAction keeps the default trigger first"),
        }
        assert_eq!(
            node.children[1].id.as_deref(),
            Some("poodle-dialog-backdrop"),
            "the composed AlertDialog backdrop follows the trigger"
        );
        assert!(node.has_text("Delete?"));
        let trigger = node
            .find(&|node| {
                matches!(&node.kind, poodle_node::NodeKind::Button { label } if label == "Reveal")
            })
            .expect("default trigger button");
        (trigger
            .interaction
            .on_activate
            .as_ref()
            .expect("the mounted trigger stays live"))();
        assert_eq!(seen.lock().unwrap().as_slice(), ["trigger"]);
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
