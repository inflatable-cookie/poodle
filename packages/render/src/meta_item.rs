//! MetaItem — uppercase label + value slot, for use inside MetaBar.
//!
//! Contract: `docs/contracts/components/meta-item.md`
//! Ported from: `packages/jetstream/components/src/meta_item.rs`.

use poodle_node::{CrossAxisAlignment, LayoutDirection, Node};
use poodle_specs::MetaItemSpec;

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn meta_item(spec: &MetaItemSpec, ctx: &RenderContext<'_>, value: Option<Node>) -> Node {
    let label_color = ctx.theme().resolve_color(spec.label_color_token());
    let value_color = ctx.theme().resolve_color(spec.value_color_token());
    let label_size = rem_to_px(spec.label_font_size_rem());
    let value_size = rem_to_px(spec.value_font_size_rem());
    let label_weight = spec.label_font_weight();
    let gap = rem_to_px(spec.gap_rem());

    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.min_width = Some(0.0);
    }

    if let Some(ref text) = spec.label {
        let mut label = Node::text(text.to_uppercase());
        let s = &mut label.style;
        s.descriptor.text_color = Some(label_color);
        s.text_size = Some(label_size);
        s.text_weight = Some(label_weight);
        s.letter_spacing_em = Some(0.08);
        row = row.child(label);
    }

    let value_el = value.unwrap_or_else(|| {
        let mut n = Node::container();
        // Explicit Row (see switch.rs).
        n.style.descriptor.layout.direction = LayoutDirection::Row;
        n.style.text_size = Some(value_size);
        n.style.descriptor.text_color = Some(value_color);
        n
    });

    let mut value_wrap = Node::container();
    {
        let s = &mut value_wrap.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.min_width = Some(0.0);
        s.text_size = Some(value_size);
        s.descriptor.text_color = Some(value_color);
    }
    row = row.child(value_wrap.child(value_el));

    if let Some(aria) = spec.aria_label.as_deref() {
        row.a11y.label = Some(aria.to_string());
    }
    row
}
