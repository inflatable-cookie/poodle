//! TextLink — inline navigation, rendered as a tone-coloured label.
//!
//! Contract: `docs/contracts/components/text-link.md`
//! Ported from: `packages/jetstream/components/src/text_link.rs`. Font size is
//! inherited, matching both old tiers.

use std::sync::Arc;

use poodle_node::{CursorHint, FocusRing, Node, NodeRole};
use poodle_specs::TextLinkSpec;

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn text_link(
    spec: &TextLinkSpec,
    ctx: &RenderContext<'_>,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let color = ctx.theme().resolve_color(spec.color_token());

    let mut el = Node::text(&spec.label);
    el.style.descriptor.text_color = Some(color);
    // The native GPUI tier keeps TextLink underlined at rest; the node
    // vocabulary carries that decoration through the shared backend.
    el.style.text_underline = true;
    el.style.text_underline_color = Some(with_alpha(color, color.3 * 0.55));

    // Svelte renders `<a>` when `href` is present and the link is enabled,
    // `<button type="button">` otherwise; a disabled link is never an anchor
    // (contract §3), so it never reads as a dead navigation target.
    el.a11y.role = Some(if spec.renders_anchor() {
        NodeRole::Link
    } else {
        NodeRole::Button
    });

    if spec.disabled {
        el.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
        // Svelte `:disabled` keeps the default cursor (contract §4 visual
        // rules); only enabled links show the pointer.
        el.style.descriptor.cursor = CursorHint::Default;
        el.interaction.disabled = true;
    } else {
        // Contract §4: Tab moves focus to the link; Enter/Space activates.
        // A link without a handler is still a sequential focus stop, matching
        // the native anchor/button focusability in Svelte.
        el.interaction.focusable = true;
        el.a11y.tab_index = Some(0);
        el.style.descriptor.cursor = CursorHint::Pointer;
        el.style.focus_ring = Some(FocusRing {
            color: ctx.theme().resolve_color(spec.focus_ring_color_token()),
            width: ctx
                .theme()
                .resolve_border_width(spec.focus_ring_width_token()),
            offset: rem_to_px(0.125),
        });
        if let Some(handler) = on_click {
            el.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}
