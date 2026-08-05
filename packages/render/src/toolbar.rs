//! Toolbar — a grouping chrome for compact action controls.
//!
//! Contract: `docs/contracts/components/toolbar.md`
//! Ported from: `packages/jetstream/components/src/toolbar.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, NodeRole,
};
use poodle_specs::{Alignment, Orientation, ToolbarSpec};

use crate::color::with_alpha;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, toolbar_density_gap_rem, toolbar_density_pad_inline_rem,
    toolbar_gap_rem, toolbar_pad_block_rem, toolbar_pad_inline_rem,
};

pub fn toolbar(spec: &ToolbarSpec, theme: &dyn ThemeProvider, children: Vec<Node>) -> Node {
    let panel_raw = theme.resolve_color(spec.bg_token());
    let bg = with_alpha(panel_raw, panel_raw.3 * 0.94);
    let border_raw = theme.resolve_color(spec.border_token());
    let border = with_alpha(border_raw, border_raw.3 * 0.78);
    let radius = theme.resolve_radius(spec.radius_token());

    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let pad_v = rem_to_px(toolbar_pad_block_rem(effective_size));
    let pad_h = rem_to_px(
        toolbar_density_pad_inline_rem(spec.density)
            .unwrap_or_else(|| toolbar_pad_inline_rem(effective_size)),
    );
    let gap = rem_to_px(
        toolbar_density_gap_rem(spec.density).unwrap_or_else(|| toolbar_gap_rem(effective_size)),
    );

    let is_vertical = spec.orientation == Orientation::Vertical;

    let mut el = Node::container();
    {
        let s = &mut el.style;
        if is_vertical {
            s.descriptor.layout.direction = LayoutDirection::Column;
            // items_stretch: taffy's default cross alignment — no call needed.
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        }
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.spacing.padding.top = pad_v;
        s.descriptor.layout.spacing.padding.bottom = pad_v;
        s.descriptor.layout.spacing.padding.left = pad_h;
        s.descriptor.layout.spacing.padding.right = pad_h;
        s.descriptor.background = Some(bg);
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        match spec.alignment {
            Alignment::Start => {}
            Alignment::Center => s.descriptor.layout.alignment.main = MainAxisAlignment::Center,
            Alignment::End => s.descriptor.layout.alignment.main = MainAxisAlignment::End,
            Alignment::Stretch => s.descriptor.layout.width = LayoutSizing::Grow,
        }
    }

    for child in children {
        el = el.child(child);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el.a11y.role = Some(NodeRole::Toolbar);
    el
}
