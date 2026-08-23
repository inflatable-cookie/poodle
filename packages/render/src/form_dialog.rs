//! FormDialog — dialog wrapping a form workflow.
//!
//! Contract: `docs/contracts/components/form-dialog.md`
//! Ported from: `packages/jetstream/components/src/form_dialog.rs`.
//!
//! Composes `dialog` (shell: backdrop + panel + header + close + separator),
//! `form_layout` (body: fields + error/success callouts), and `button` /
//! `form_actions` (footer: default submit/cancel row) or a caller-owned custom
//! actions slot.

use std::sync::Arc;

use poodle_node::{LayoutDirection, Node};
use poodle_specs::{
    ButtonSpec, ButtonVariant, DialogSpec, FormActionAlign, FormActionsSpec, FormDialogSpec,
    FormLayoutSpec,
};

use crate::button::button;
use crate::context::RenderContext;
use crate::dialog::dialog;
use crate::form_actions::form_actions;
use crate::form_layout::form_layout;
use crate::presentation::{rem_to_px, size_font_rem};

/// Handlers mirror the GPUI target's names. `on_cancel` also covers the
/// dialog's dismissal routes. With `custom_actions` the host owns the buttons,
/// so the default handlers have nothing to attach to — wire the custom actions
/// directly.
#[derive(Default)]
pub struct FormDialogHandlers {
    /// Fires when the default submit button is pressed. Never fires while
    /// submitting or disabled.
    pub on_submit: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Fires for the cancel button and the dialog's dismissal routes.
    pub on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub fn form_dialog(
    spec: &FormDialogSpec,
    ctx: &RenderContext<'_>,
    children: Vec<Node>,
    custom_actions: Option<Node>,
    handlers: FormDialogHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(size_font_rem(effective_size));
    // FormDialog's body stack follows the shared stack-md contract. Using the
    // control font ladder here made subtitle/form and bare-body spacing drift
    // from the old GPUI composite.
    let section_gap = ctx.theme().resolve_space("space.stack.md");

    let text_secondary = ctx.theme().resolve_color("color.text.secondary");

    let subtitle_label = |content: &str| -> Node {
        let mut t = Node::text(content);
        t.style.text_size = Some(font_size);
        t.style.descriptor.text_color = Some(text_secondary);
        t
    };

    // ── Body ──────────────────────────────────────────────────────────────────
    // Subtitle (when present) is rendered inline at the top of the body; the
    // Dialog `description` slot is then suppressed to avoid a duplicate
    // announcement (contract §9). When no subtitle, the description flows to
    // the Dialog header.
    let body: Node = if spec.is_bare {
        // Bare mode: children rendered directly, no FormLayout wrapper.
        let mut col = Node::container();
        col.style.descriptor.layout.direction = LayoutDirection::Column;
        col.style.descriptor.layout.spacing.gap = section_gap;
        let mut col = col;
        if let Some(ref subtitle) = spec.subtitle {
            col = col.child(subtitle_label(subtitle));
        }
        for child in children {
            col = col.child(child);
        }
        col
    } else {
        let layout_spec = FormLayoutSpec {
            columns: spec.columns,
            error: spec.error.clone(),
            success: spec.success.clone(),
            description: None,
            ..FormLayoutSpec::default()
        };
        let form = form_layout(&layout_spec, ctx, children, None);
        if let Some(ref subtitle) = spec.subtitle {
            let mut col = Node::container();
            col.style.descriptor.layout.direction = LayoutDirection::Column;
            col.style.descriptor.layout.spacing.gap = section_gap;
            col.child(subtitle_label(subtitle)).child(form)
        } else {
            form
        }
    };

    // ── Actions ───────────────────────────────────────────────────────────────
    // Custom slot replaces the default row entirely; otherwise the default
    // ghost-Cancel + primary-Submit pair is composed from `button` and laid
    // out by `form_actions` with top separation removed so the row sits flush
    // on the Dialog footer rail (contract §7 nested-FormActions padding-top: 0).
    let actions: Option<Node> = if let Some(custom) = custom_actions {
        let mut wrap = Node::container();
        // Explicit Row (see switch.rs).
        wrap.style.descriptor.layout.direction = LayoutDirection::Row;
        wrap.style.fill_width = true;
        Some(wrap.child(custom))
    } else if spec.show_default_actions {
        let submit_disabled = spec.is_submitting || spec.is_disabled;

        // Cancel — ghost Button, disabled during submitting (Svelte parity).
        let cancel = button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label(spec.cancel_label.clone())
                .with_size(base_size)
                .with_size_role(spec.size_role)
                .with_density(density)
                .with_disabled(spec.is_submitting),
            ctx,
            handlers.on_cancel.as_ref().map(Arc::clone),
        );

        // Submit — primary Button. Label flips to "Submitting…" and the button
        // is disabled while submitting or explicitly disabled.
        let submit_label = if spec.is_submitting {
            "Submitting\u{2026}".to_string()
        } else {
            spec.submit_label.clone()
        };
        let submit = button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_label(submit_label)
                .with_size(base_size)
                .with_size_role(spec.size_role)
                .with_density(density)
                .with_disabled(submit_disabled),
            ctx,
            handlers.on_submit.as_ref().map(Arc::clone),
        );

        let row_spec = FormActionsSpec::new()
            .with_align(FormActionAlign::End)
            .with_density(density)
            // Sit flush on the Dialog footer rail; the Dialog draws the divider.
            .with_top_separation(false);
        Some(form_actions(&row_spec, ctx, vec![cancel, submit]))
    } else {
        None
    };

    // ── Dialog shell ──────────────────────────────────────────────────────────
    // Compose `dialog` for the surface, backdrop, header (title + description +
    // close), and the action separator. Submitting blocks Escape/backdrop
    // dismiss (contract §6); dismiss wiring itself lives in the host loop.
    let mut dialog_spec = DialogSpec::new()
        .with_default_open(true)
        .with_show_close_button(true)
        .with_size(base_size)
        .with_size_role(spec.size_role)
        .with_density(density)
        .with_dismiss_on_escape(!spec.is_submitting)
        .with_dismiss_on_backdrop(!spec.is_submitting);

    if !spec.title.is_empty() {
        dialog_spec = dialog_spec.with_title(spec.title.clone());
    }
    // Subtitle takes precedence over description; when a subtitle is rendered
    // inline in the body, suppress the Dialog description to avoid duplication.
    if spec.subtitle.is_none() {
        if let Some(ref description) = spec.description {
            dialog_spec = dialog_spec.with_description(description.clone());
        }
    }
    if let Some(width) = spec.width {
        dialog_spec = dialog_spec.with_width(width);
    }
    if let Some(ref aria) = spec.aria_label {
        dialog_spec = dialog_spec.with_aria_label(aria.clone());
    }

    // Dismiss routes (Escape/backdrop/close) are host-wired; the old tier does
    // not attach on_cancel to the shell either, so the close affordance stays
    // handler-free here for byte parity.
    dialog(&dialog_spec, ctx, vec![body], actions, None)
}
