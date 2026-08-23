//! StateTile — lightweight label-value tile.
//!
//! Contract: `docs/contracts/components/state-tile.md` (sole authority).
//! Ported from: `packages/jetstream/components/src/state_tile.rs`.
//!
//! Anatomy (contract §2): Root → Label, Value, optional Trend (glyph +
//! label), optional reserved Sparkline slot (the host renders the chart).

use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node};
use poodle_specs::StateTileSpec;

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn state_tile(spec: &StateTileSpec, ctx: &RenderContext<'_>) -> Node {
    // ── Colors (token-resolved via spec) ──
    let fill = ctx.theme().resolve_color(spec.fill_token());
    let border = ctx.theme().resolve_color(spec.border_token());
    let label_color = ctx.theme().resolve_color(spec.label_color_token());
    let value_color = ctx.theme().resolve_color(spec.value_color_token());
    let trend_color = ctx.theme().resolve_color(spec.trend_color_token());

    // ── Dimensions ──
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    let border_width = ctx.theme().resolve_space(spec.border_width_token());
    let pad_x = ctx.theme().resolve_space("space.panel.x");
    let pad_y = ctx.theme().resolve_space("space.panel.y");
    let gap = ctx.theme().resolve_space("space.stack.sm"); // label↔value↔trend↔sparkline
    let trend_gap = ctx.theme().resolve_space("space.inline.xs"); // glyph↔label
    let label_size = ctx.theme().resolve_space(spec.label_font_size_token());
    let value_size = ctx.theme().resolve_space(spec.value_font_size_token());
    let trend_size = ctx.theme().resolve_space(spec.trend_font_size_token());

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = gap;
    }

    // ── Label (contract §2: caption, text-secondary) ──
    let mut label = Node::text(&spec.label);
    label.style.descriptor.text_color = Some(label_color);
    label.style.text_size = Some(label_size);
    let el = el.child(label);

    // ── Value (contract §2: primary value, typography-heading) ──
    let mut value = Node::text(&spec.value);
    value.style.descriptor.text_color = Some(value_color);
    value.style.text_size = Some(value_size);
    value.style.text_weight = Some(700);
    let mut el = el.child(value);

    // ── Trend row (optional) ──
    // Contract §7: trend meaning lives in `trend_label` text; the glyph is
    // decorative.
    if spec.trend.is_some() || spec.trend_label.is_some() {
        let mut trend_row = Node::container();
        {
            let s = &mut trend_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = trend_gap;
        }

        if spec.trend.is_some() {
            let mut glyph = Node::text(spec.trend_glyph());
            glyph.style.descriptor.text_color = Some(trend_color);
            glyph.style.text_size = Some(trend_size);
            glyph.style.text_weight = Some(600);
            trend_row = trend_row.child(glyph);
        }

        if let Some(ref trend_label) = spec.trend_label {
            let mut t = Node::text(trend_label);
            t.style.descriptor.text_color = Some(trend_color);
            t.style.text_size = Some(trend_size);
            trend_row = trend_row.child(t);
        }

        el = el.child(trend_row);
    }

    // ── Sparkline slot (optional) ──
    // Contract §1/§2/§3: the host owns the chart; StateTile only reserves the
    // slot. Render an empty reserved area (no synthetic chart data).
    if spec.has_sparkline {
        let sparkline_h = rem_to_px(2.0); // contract slot height, no token
        let slot_bg = ctx.theme().resolve_color(spec.sparkline_slot_token());

        let mut slot = Node::container();
        slot.id = Some("state-tile-sparkline".to_string());
        {
            let s = &mut slot.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.background = Some(slot_bg);
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius;
            c.top_right = radius;
            c.bottom_right = radius;
            c.bottom_left = radius;
            s.descriptor.layout.height = LayoutSizing::Fixed(sparkline_h);
            s.descriptor.layout.width = LayoutSizing::Grow;
        }
        el = el.child(slot);
    }

    el
}
