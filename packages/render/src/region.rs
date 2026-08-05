//! Region — presentational placeholder block.
//!
//! Contract: `docs/contracts/components/region.md`
//! Ported from: `packages/jetstream/components/src/region.rs`.
//! Decorative, non-interactive: a dashed border with an optional centred,
//! uppercase label. Per contract §3 it does NOT accept child content.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::RegionSpec;

use crate::color::hex_color;
use crate::presentation::rem_to_px;

pub fn region(spec: &RegionSpec, theme: &dyn ThemeProvider) -> Node {
    let radius = theme.resolve_radius(spec.radius_token());
    let padding = theme.resolve_space(spec.padding_token());

    // Custom color (contract §5) overrides both border and label; otherwise
    // resolve the default semantic tokens. Hex parses at the sRGB edge.
    let border_color = match &spec.color {
        Some(hex) => hex_color(hex).unwrap_or_else(|| theme.resolve_color(hex)),
        None => theme.resolve_color(spec.border_color_token()),
    };
    let label_color = match &spec.color {
        Some(hex) => hex_color(hex).unwrap_or_else(|| theme.resolve_color(hex)),
        None => theme.resolve_color(spec.label_color_token()),
    };

    // Contract §2/Svelte: dashed border at 0.125rem (2px).
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.min_height = Some(spec.min_height_px);
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.border.width = rem_to_px(0.125);
        s.border_dashed = true;
        s.descriptor.border.color = border_color;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = padding;
        pad.right = padding;
        pad.top = padding;
        pad.bottom = padding;
    }

    if !spec.label.is_empty() {
        let label_size = theme.resolve_space(spec.label_text_size_token());
        let mut label = Node::text(spec.label.to_uppercase());
        label.style.descriptor.text_color = Some(label_color);
        label.style.text_size = Some(label_size);
        el = el.child(label);
    }

    el
}
