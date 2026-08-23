//! StatusBar — status row (contract: status-bar). A footer-equivalent strip
//! with a leading region (slot content or summary fallback) and a trailing
//! region (only rendered when trailing items exist), arranged space-between.
//!
//! Ported from: `packages/jetstream/components/src/shell_status_bar.rs`.

use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::ShellStatusBarSpec;

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn shell_status_bar(
    spec: &ShellStatusBarSpec,
    ctx: &RenderContext<'_>,
    leading: Vec<Node>,
    trailing: Vec<Node>,
) -> Node {
    let text_color = ctx.theme().resolve_color(spec.text_color_token());

    // Contract §8: font-size scales by size; padding 0.375rem 0.75rem scaled
    // by size (block) / density (inline); root gap = space.inline.md
    // overridden by density; inner gap = space.inline.sm. No fixed height —
    // content + padding drive it.
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(spec.font_size_rem(base_size));
    let pad_block = rem_to_px(spec.padding_block_rem(base_size));
    let pad_inline = rem_to_px(spec.padding_inline_rem(density));
    let root_gap = match spec.density_gap_rem(density) {
        Some(rem) => rem_to_px(rem),
        None => ctx.theme().resolve_space(spec.root_gap_token(density)),
    };
    let inner_gap = ctx.theme().resolve_space(spec.inner_gap_token());

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.fill_width = true;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = root_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_inline;
        pad.right = pad_inline;
        pad.top = pad_block;
        pad.bottom = pad_block;
    }

    // Chrome modifier: 94% panel background + border-top. Without chrome the
    // bar is transparent and blends into its container.
    if spec.chrome {
        let panel = ctx.theme().resolve_color(spec.chrome_background_token());
        // color-mix toward same-rgb transparent = alpha scale only.
        let chrome_bg = with_alpha(panel, panel.3 * spec.chrome_background_opacity());
        let border = ctx.theme().resolve_color(spec.chrome_border_token());
        // Border width token resolves to 0.0625rem (1px at 16px base); the
        // runtime exposes a fixed 1px top border, so width is approximated at
        // 1px with the per-side top color set explicitly.
        let s = &mut el.style;
        s.descriptor.background = Some(chrome_bg);
        s.border_top_width = Some(1.0);
        s.border_color_top = Some(border);
    }

    let region = |items: Vec<Node>| -> Node {
        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = inner_gap;
        }
        let mut row = row;
        for item in items {
            row = row.child(item);
        }
        row
    };

    // Leading region: slot content, or summary fallback when no slot content.
    let leading_row = if !leading.is_empty() {
        region(leading)
    } else if let Some(ref summary) = spec.summary {
        let mut label = Node::text(summary);
        label.style.descriptor.text_color = Some(text_color);
        label.style.text_size = Some(font_size);
        region(vec![label])
    } else {
        region(vec![])
    };
    let mut el = el.child(leading_row);

    // Trailing region: only rendered when trailing slot content exists.
    if !trailing.is_empty() {
        el = el.child(region(trailing));
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
