//! TextLink — inline navigation, rendered as a tone-coloured label.
//!
//! Contract: `docs/contracts/components/text-link.md`
//! Ported from: `packages/jetstream/components/src/text_link.rs`. Font size is
//! inherited, matching both old tiers.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CursorHint, Node};
use poodle_specs::TextLinkSpec;

pub fn text_link(
    spec: &TextLinkSpec,
    theme: &dyn ThemeProvider,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let color = theme.resolve_color(spec.color_token());

    let mut el = Node::text(&spec.label);
    el.style.descriptor.text_color = Some(color);

    if spec.disabled {
        el.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
    } else if let Some(handler) = on_click {
        el.style.descriptor.cursor = CursorHint::Pointer;
        el.interaction.on_activate = Some(Arc::new(move || handler()));
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}
