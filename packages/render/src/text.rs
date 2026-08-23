//! Text — the text primitive: tone, size, weight, wrap, compact spacing.
//!
//! Contract: `docs/contracts/components/text.md`
//! Ported from: `packages/jetstream/components/src/text.rs`.

use poodle_node::{LayoutDirection, LayoutOverflow, Node};
use poodle_specs::{TextSpec, TextWeight};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn text(spec: &TextSpec, ctx: &RenderContext<'_>) -> Node {
    let color = ctx.theme().resolve_color(spec.color_token());
    let weight: u16 = match spec.weight {
        TextWeight::Normal => 400,
        TextWeight::Medium => 500,
        TextWeight::Semibold => 600,
        TextWeight::Bold => 700,
    };

    let mut el = Node::text(&spec.content);
    {
        let s = &mut el.style;
        s.descriptor.text_color = Some(color);
        s.text_size = Some(rem_to_px(spec.font_size_rem()));
        s.text_weight = Some(weight);
        s.line_height = Some(spec.line_height());
        s.text_wrap = true;
        // `clamp` degrades to wrapped text clipped at the box, as on both old
        // native tiers — the exact N-line cap + ellipsis stays a backend gap.
        if spec.clamp.is_some() {
            s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        }
    }

    // spacing="compact": stack in a column with the resolved gap.
    if let Some(token) = spec.spacing_gap_token() {
        let gap = ctx.theme().resolve_space(token);
        let mut column = Node::container();
        column.style.descriptor.layout.direction = LayoutDirection::Column;
        column.style.descriptor.layout.spacing.gap = gap;
        return column.child(el);
    }
    el
}
