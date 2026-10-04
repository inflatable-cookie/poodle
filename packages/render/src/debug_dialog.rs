//! DebugDialog — debug-data disclosure.
//!
//! Contract: `docs/contracts/components/debug-dialog.md`
//! Ported from: `packages/jetstream/components/src/debug_dialog.rs`.
//!
//! Renders a trigger button when a value is present. The dialog is host-owned
//! (`open`): Svelte keeps the trigger mounted and opens the Dialog from it.
//! Nothing renders without a value.

use std::sync::Arc;

use poodle_node::{LayoutDirection, Node};
use poodle_specs::{ButtonSpec, CodeSpec, DebugDialogSpec, DialogSpec, DialogWidth};

use crate::button::button;
use crate::code::code;
use crate::context::RenderContext;
use crate::dialog::dialog;

#[derive(Default)]
pub struct DebugDialogHandlers {
    /// Fires with `true` from the trigger and `false` from Dialog dismissal.
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

/// Catalogue/Jetstream path: a present value paints the open dialog so the
/// JSON dump is visible without a host toggle.
pub fn debug_dialog(spec: &DebugDialogSpec, ctx: &RenderContext<'_>) -> Node {
    debug_dialog_with_state(spec, ctx, spec.has_value(), DebugDialogHandlers::default())
}

pub fn debug_dialog_with_state(
    spec: &DebugDialogSpec,
    ctx: &RenderContext<'_>,
    open: bool,
    handlers: DebugDialogHandlers,
) -> Node {
    if !spec.has_value() {
        return Node::container();
    }

    let mut button_spec = ButtonSpec::new()
        .with_label(spec.trigger_label.as_str())
        .with_variant(spec.trigger_variant);
    if let Some(size) = spec.trigger_size {
        button_spec = button_spec.with_size(size);
    }

    let on_open = handlers.on_open_change.as_ref().map(|handler| {
        let handler = Arc::clone(handler);
        Arc::new(move || handler(true)) as Arc<dyn Fn() + Send + Sync>
    });
    let mut trigger = button(&button_spec, ctx, on_open);
    trigger.id = Some("poodle-debug-dialog-trigger".to_string());

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.md");
    }
    let mut root = root.child(trigger);
    if !open {
        return root;
    }

    // Code block: JSON value, with the max-height clamp parsed from the spec's
    // CSS string (the rem term — the vh term is viewport-relative).
    let mut code_spec = CodeSpec::new()
        .with_content(spec.value.clone().unwrap_or_default())
        .with_language("json");
    if let Some(mh) = spec.max_height_px() {
        code_spec = code_spec.with_max_height(mh);
    }

    let dialog_spec = DialogSpec::new()
        .with_title(spec.title.clone())
        .with_width(DialogWidth::Lg)
        .with_show_close_button(spec.show_close_button)
        .with_close_label(spec.close_label.clone());
    let on_close = handlers
        .on_open_change
        .map(|handler| Arc::new(move || handler(false)) as Arc<dyn Fn() + Send + Sync>);
    root.child(dialog(
        &dialog_spec,
        ctx,
        vec![code(&code_spec, ctx)],
        None,
        on_close,
    ))
}
