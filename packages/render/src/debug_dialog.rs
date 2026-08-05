//! DebugDialog — debug-data disclosure.
//!
//! Contract: `docs/contracts/components/debug-dialog.md`
//! Ported from: `packages/jetstream/components/src/debug_dialog.rs`.
//!
//! Renders a trigger button + a JSON code block when a value is present (the
//! open/close dialog behaviour is host-owned). Nothing renders without a
//! value.

use poodle_adapter::ThemeProvider;
use poodle_node::{LayoutDirection, Node};
use poodle_specs::{ButtonSpec, CodeSpec, DebugDialogSpec};

use crate::button::button;
use crate::code::code;
use crate::presentation::rem_to_px;

pub fn debug_dialog(spec: &DebugDialogSpec, theme: &dyn ThemeProvider) -> Node {
    if !spec.has_value() {
        return Node::container();
    }

    let mut button_spec = ButtonSpec::new()
        .with_label(spec.trigger_label.as_str())
        .with_variant(spec.trigger_variant);
    if let Some(size) = spec.trigger_size {
        button_spec = button_spec.with_size(size);
    }

    // Code block: JSON value, with the max-height clamp parsed from the spec's
    // CSS string (the rem term — the vh term is viewport-relative).
    let mut code_spec = CodeSpec::new()
        .with_content(spec.value.clone().unwrap_or_default())
        .with_language("json");
    if let Some(mh) = spec.max_height_px() {
        code_spec = code_spec.with_max_height(mh);
    }

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.75);
    }
    root.child(button(&button_spec, theme, None))
        .child(code(&code_spec, theme))
}
