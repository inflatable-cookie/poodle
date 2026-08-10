//! AlertDialog — composes [`crate::dialog::dialog`] with two composed
//! [`crate::button::button`]s for cancel/confirm.
//!
//! Contract: `docs/contracts/components/alert-dialog.md`
//! Ported from: `packages/jetstream/components/src/alert_dialog.rs`. Owns only
//! the alert-specific content and the tone→confirm mapping; everything visual
//! delegates to the composed primitives.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node, NodeRole};
use poodle_specs::{
    AlertDialogSpec, AlertDialogTone, ButtonSpec, ButtonTone, ButtonVariant, DialogKind,
    DialogSpec, DialogWidth,
};

use crate::button::button;
use crate::dialog::dialog;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};

/// Contract default confirm label while working.
pub const DEFAULT_WORKING_LABEL: &str = "Working\u{2026}";

/// Svelte: `confirmTone = tone === "danger" ? "danger" : "default"`.
fn confirm_button_tone(tone: AlertDialogTone) -> ButtonTone {
    match tone {
        AlertDialogTone::Danger => ButtonTone::Danger,
        AlertDialogTone::Warning => ButtonTone::Default,
    }
}

#[derive(Default)]
pub struct AlertDialogHandlers {
    pub confirm: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Fires for cancel AND every dismissal route, per contract.
    pub cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub fn alert_dialog(
    spec: &AlertDialogSpec,
    theme: &dyn ThemeProvider,
    working: bool,
    working_label: &str,
    handlers: AlertDialogHandlers,
) -> Node {
    alert_dialog_with_content(spec, theme, working, working_label, Vec::new(), handlers)
}

/// Render an alert dialog with caller-supplied body nodes between the optional
/// item-detail row and the actions. This is the shared equivalent of the
/// Svelte/legacy GPUI default content slot.
pub fn alert_dialog_with_content(
    spec: &AlertDialogSpec,
    theme: &dyn ThemeProvider,
    working: bool,
    working_label: &str,
    content: Vec<Node>,
    handlers: AlertDialogHandlers,
) -> Node {
    let size = spec.size;
    let size_role = spec.size_role;
    let density = spec.density;

    // Cancel (ghost).
    let cancel_spec = ButtonSpec::new()
        .with_variant(ButtonVariant::Ghost)
        .with_size(size)
        .with_size_role(size_role)
        .with_density(density)
        .with_label(spec.cancel_label.clone())
        .with_disabled(working);
    let cancel_btn = button(&cancel_spec, theme, handlers.cancel.clone());

    // Confirm (primary, tone-driven; working swaps the label).
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
    let confirm_btn = button(&confirm_spec, theme, handlers.confirm.clone());

    // Actions row: cancel left, confirm right.
    let actions_gap = theme.resolve_space(spec.actions_gap_token());
    let mut actions = Node::container();
    {
        let s = &mut actions.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.alignment.main = MainAxisAlignment::End;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = actions_gap;
    }
    let actions = actions.child(cancel_btn).child(confirm_btn);

    // Optional item-detail row.
    let mut children: Vec<Node> = Vec::new();
    if let (Some(label), Some(value)) = (spec.item_label.as_ref(), spec.item_value.as_ref()) {
        let effective_size = resolve_semantic_size(size, size_role);
        let body_font = rem_to_px(size_font_rem(effective_size));
        let text_secondary = theme.resolve_color(spec.description_color_token());
        let text_primary = theme.resolve_color(spec.title_color_token());
        let detail_gap = theme.resolve_space(spec.actions_gap_token());

        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = detail_gap;
        }
        let mut l = Node::text(format!("{label}:"));
        l.style.descriptor.text_color = Some(text_primary);
        l.style.text_size = Some(body_font);
        l.style.text_weight = Some(600);
        let mut v = Node::text(value);
        v.style.descriptor.text_color = Some(text_secondary);
        v.style.text_size = Some(body_font);
        children.push(row.child(l).child(v));
    }
    children.extend(content);

    // Compose the real dialog; working gates every dismissal route.
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

    let mut root = dialog(
        &dialog_spec,
        theme,
        children,
        Some(actions),
        handlers.cancel,
    );
    // An alert dialog interrupts; assistive technology must know the
    // difference from a plain dialog. Overridden on the way out, as the
    // shared surface cannot know which it is building.
    root.a11y.role = Some(NodeRole::AlertDialog);
    root
}
