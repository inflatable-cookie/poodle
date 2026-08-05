//! DetailItem — label/value row: inline or stacked, simple or surface.
//!
//! Contract: `docs/contracts/components/detail-item.md`
//! Ported from: `packages/jetstream/components/src/detail_item.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node};
use poodle_specs::{DetailItemLayout, DetailItemPresentation, DetailItemSpan, DetailItemSpec};

use crate::presentation::rem_to_px;

pub fn detail_item(spec: &DetailItemSpec, theme: &dyn ThemeProvider) -> Node {
    detail_item_with_slots(spec, theme, None, None)
}

/// Variant exposing the value-content and trailing-action slots.
pub fn detail_item_with_slots(
    spec: &DetailItemSpec,
    theme: &dyn ThemeProvider,
    value_content: Option<Node>,
    action: Option<Node>,
) -> Node {
    let label_color = theme.resolve_color(spec.label_color_token());
    let value_color = theme.resolve_color(spec.value_color_token());
    let desc_color = theme.resolve_color(spec.description_color_token());
    let tertiary_color = theme.resolve_color(spec.stacked_label_color_token());
    let bg = theme.resolve_color(spec.background_token());
    let radius = theme.resolve_radius(spec.radius_token());

    let label_font = theme.resolve_space(spec.label_size_token());
    let value_font = theme.resolve_space(spec.value_size_token());
    let desc_font = rem_to_px(0.75);

    let row_gap = rem_to_px(spec.row_gap_rem());
    let inline_gap = rem_to_px(spec.inline_gap_rem());
    let pad_x = rem_to_px(spec.surface_padding_x_rem());
    let pad_y = rem_to_px(spec.surface_padding_y_rem());

    let is_stacked = spec.layout == DetailItemLayout::Stacked;
    let is_surface = spec.presentation == DetailItemPresentation::Surface;
    let is_surface_stacked = is_surface && is_stacked;

    let (eff_label_color, eff_label_font) = if is_surface_stacked {
        (tertiary_color, rem_to_px(0.75))
    } else {
        (label_color, label_font)
    };
    let (eff_value_font, value_weight) = if is_surface_stacked {
        (rem_to_px(1.0), 600u16)
    } else {
        (value_font, 400u16)
    };

    // ── Label block ──
    let mut label_block = Node::container();
    label_block.style.descriptor.layout.direction = LayoutDirection::Column;
    label_block.style.descriptor.layout.spacing.gap = row_gap;
    let mut l = Node::text(&spec.label);
    l.style.descriptor.text_color = Some(eff_label_color);
    l.style.text_size = Some(eff_label_font);
    label_block = label_block.child(l);
    if let Some(ref desc) = spec.description {
        let mut d = Node::text(desc);
        d.style.descriptor.text_color = Some(desc_color);
        d.style.text_size = Some(desc_font);
        label_block = label_block.child(d);
    }
    if !is_stacked {
        label_block.style.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(11.25));
        label_block.style.flex_shrink_zero = true;
    }

    // ── Value: slot > text > em-dash placeholder ──
    let value_block = if let Some(content) = value_content {
        let mut wrap = Node::container();
        // Explicit Row (see switch.rs), preemptively — this slot path has no
        // old-tier caller in the fixtures but the same silent-Row shape.
        wrap.style.descriptor.layout.direction = LayoutDirection::Row;
        wrap.style.descriptor.layout.width = LayoutSizing::Grow;
        wrap.child(content)
    } else if let Some(ref value) = spec.value {
        let mut v = Node::text(value);
        v.style.descriptor.text_color = Some(value_color);
        v.style.text_size = Some(eff_value_font);
        v.style.text_weight = Some(value_weight);
        v.style.descriptor.layout.width = LayoutSizing::Grow;
        if spec.truncate_value {
            v.style.text_ellipsis = true;
            v.style.no_wrap = true;
        }
        v
    } else {
        let mut v = Node::text(&spec.empty_text);
        v.style.descriptor.text_color = Some(desc_color);
        v.style.text_size = Some(eff_value_font);
        v.style.descriptor.layout.width = LayoutSizing::Grow;
        v
    };

    // ── Root ──
    let mut el = Node::container();
    {
        let s = &mut el.style;
        if is_stacked {
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = row_gap;
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = inline_gap;
        }
        if is_surface_stacked {
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        }
        if is_surface {
            s.descriptor.background = Some(bg);
            s.descriptor.corner_radii.top_left = radius;
            s.descriptor.corner_radii.top_right = radius;
            s.descriptor.corner_radii.bottom_right = radius;
            s.descriptor.corner_radii.bottom_left = radius;
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.layout.spacing.padding.top = pad_y;
            s.descriptor.layout.spacing.padding.bottom = pad_y;
        }
        if matches!(spec.span, Some(DetailItemSpan::Full)) {
            s.self_stretch = true;
        }
    }

    el = el.child(label_block).child(value_block);

    if let Some(action_el) = action {
        let mut slot = Node::container();
        {
            let s = &mut slot.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.flex_shrink_zero = true;
        }
        el = el.child(slot.child(action_el));
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}
