//! InlineListSection — compact titled list section.
//!
//! Contract: `docs/contracts/components/inline-list-section.md`
//! Ported from: `packages/jetstream/components/src/inline_list_section.rs`.
//!
//! Uppercase title + optional count pill + optional action header, then
//! either an empty message or the item list, optionally wrapped in a Card.

use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node};
use poodle_specs::{CardSpec, InlineListSectionSpec};

use crate::card::card;
use crate::color::mix_srgb;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Build an inline-list-section from its spec + item rows + an optional
/// action.
pub fn inline_list_section(
    spec: &InlineListSectionSpec,
    ctx: &RenderContext<'_>,
    items: Vec<Node>,
    action: Option<Node>,
) -> Node {
    // Colors (contract Token Usage tables).
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let border = ctx.theme().resolve_color("color.border.default");
    let elevated = ctx.theme().resolve_color("color.background.elevated");
    let surface = ctx.theme().resolve_color("color.background.surface");
    // Item chrome: color-mix(in srgb, surface 93%, text-primary).
    let row_bg = mix_srgb(surface, text_primary, 0.93);

    // Typography (contract Token Usage tables).
    let label_size = ctx.theme().resolve_space("typography.label.size");
    let body_size = ctx.theme().resolve_space("typography.body.size");

    // Spacing (token + contract-exact rem).
    let root_gap = ctx.theme().resolve_space("space.stack.md");
    let items_gap = ctx.theme().resolve_space("space.stack.sm");

    // Item-row radius: calc(radius.surface − 0.1875rem).
    let surface_radius = ctx.theme().resolve_radius("radius.surface");
    let item_radius = surface_radius - rem_to_px(0.1875);

    // Title cluster: uppercase title + optional count pill.
    let mut title_cluster = Node::container();
    {
        let s = &mut title_cluster.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.5);
        s.min_width = Some(0.0);
    }
    let mut title = Node::text(spec.title.to_uppercase());
    title.style.text_size = Some(label_size);
    title.style.text_weight = Some(600);
    title.style.letter_spacing_em = Some(0.05); // contract Title: letter-spacing 0.05em
    title.style.descriptor.text_color = Some(text_secondary);
    let mut title_cluster = title_cluster.child(title);

    if let Some(count) = &spec.count {
        let mut pill = Node::container();
        {
            let s = &mut pill.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.min_width = Some(rem_to_px(1.875));
            s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(1.375));
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(0.5);
            pad.right = rem_to_px(0.5);
            let c = &mut s.descriptor.corner_radii;
            c.top_left = 999.0;
            c.top_right = 999.0;
            c.bottom_right = 999.0;
            c.bottom_left = 999.0;
            s.descriptor.border.width = rem_to_px(0.0625);
            s.descriptor.border.color = border;
            s.descriptor.background = Some(elevated);
        }
        let mut count_label = Node::text(count.as_str());
        count_label.style.text_size = Some(label_size);
        count_label.style.text_weight = Some(600);
        count_label.style.descriptor.text_color = Some(text_secondary);
        title_cluster = title_cluster.child(pill.child(count_label));
    }

    let mut header = Node::container();
    {
        let s = &mut header.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = rem_to_px(0.75);
    }
    let mut header = header.child(title_cluster);
    if let Some(action) = action {
        header = header.child(action);
    }

    let mut body = Node::container();
    {
        let s = &mut body.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
    }
    let mut body = body.child(header);

    if items.is_empty() {
        if let Some(message) = &spec.empty_message {
            let mut msg = Node::text(message.as_str());
            msg.style.text_size = Some(body_size);
            msg.style.descriptor.text_color = Some(text_secondary);
            body = body.child(msg);
        }
    } else {
        let mut list = Node::container();
        {
            let s = &mut list.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = items_gap;
        }
        for item in items {
            let mut row = Node::container();
            {
                let s = &mut row.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = rem_to_px(0.75);
                s.min_width = Some(0.0);
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(0.625);
                pad.right = rem_to_px(0.625);
                pad.top = rem_to_px(0.5);
                pad.bottom = rem_to_px(0.5);
                let c = &mut s.descriptor.corner_radii;
                c.top_left = item_radius;
                c.top_right = item_radius;
                c.bottom_right = item_radius;
                c.bottom_left = item_radius;
                s.descriptor.background = Some(row_bg);
            }
            list = list.child(row.child(item));
        }
        body = body.child(list);
    }

    if spec.framed {
        card(&CardSpec::new(), ctx, vec![body])
    } else {
        body
    }
}
