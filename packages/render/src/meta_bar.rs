//! MetaBar — wrapping inline metadata row with dot separators.
//!
//! Contract: `docs/contracts/components/meta-bar.md`
//! Ported from: `packages/jetstream/components/src/meta_bar.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node};
use poodle_specs::MetaBarSpec;

use crate::color::with_alpha;
use crate::presentation::rem_to_px;

/// Separator-dot alpha factor — a contract literal, named not inlined.
const SEPARATOR_DOT_MIX: f32 = 0.72;

pub fn meta_bar(spec: &MetaBarSpec, theme: &dyn ThemeProvider, children: Vec<Node>) -> Node {
    meta_bar_sep(spec, theme, children.into_iter().map(|c| (c, true)).collect())
}

/// Children paired with their per-child separator opt-in.
pub fn meta_bar_sep(
    spec: &MetaBarSpec,
    theme: &dyn ThemeProvider,
    children: Vec<(Node, bool)>,
) -> Node {
    let gap = theme.resolve_space("space.inline.sm");
    let separator_color = theme.resolve_color("color.text.secondary");
    let dot_color = with_alpha(separator_color, separator_color.3 * SEPARATOR_DOT_MIX);
    let dot_size = rem_to_px(0.25);
    let dot_radius = theme.resolve_radius("radius.pill");

    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.min_width = Some(0.0);
    }

    for (idx, (child, separator)) in children.into_iter().enumerate() {
        if idx > 0 && spec.show_separators && separator {
            let mut dot = Node::container();
            {
                let s = &mut dot.style;
                s.descriptor.layout.width = LayoutSizing::Fixed(dot_size);
                s.descriptor.layout.height = LayoutSizing::Fixed(dot_size);
                s.descriptor.corner_radii.top_left = dot_radius;
                s.descriptor.corner_radii.top_right = dot_radius;
                s.descriptor.corner_radii.bottom_right = dot_radius;
                s.descriptor.corner_radii.bottom_left = dot_radius;
                s.descriptor.background = Some(dot_color);
            }
            row = row.child(dot);
        }
        let mut wrapper = Node::container();
        // Explicit Row (see switch.rs).
        wrapper.style.descriptor.layout.direction = LayoutDirection::Row;
        wrapper.style.min_width = Some(0.0);
        row = row.child(wrapper.child(child));
    }

    if let Some(label) = spec.aria_label.as_deref() {
        row.a11y.label = Some(label.to_string());
    }
    row
}
