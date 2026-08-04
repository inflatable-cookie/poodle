//! Avatar — initials or image in a tone-coloured, shaped frame.
//!
//! Contract: `docs/contracts/components/avatar.md`
//! Ported from: `packages/jetstream/components/src/avatar.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node,
    NodeRole,
};
use poodle_specs::AvatarSpec;

use crate::color::mix_srgb;
use crate::presentation::rem_to_px;

pub fn avatar(spec: &AvatarSpec, theme: &dyn ThemeProvider) -> Node {
    let size = rem_to_px(spec.size_rem());
    let font_size = rem_to_px(spec.font_size_rem());

    // Tone colours via the spec token targets + mix ratio (contract §3). The
    // old tier's color_mix works in sRGB space, same as every state recipe.
    let base = theme.resolve_color(spec.background_base_token());
    let mix = theme.resolve_color(spec.background_mix_token());
    let bg = mix_srgb(base, mix, spec.background_mix_ratio());
    let fg = theme.resolve_color(spec.color_token());

    let radius = if spec.is_circle() {
        rem_to_px(spec.circle_radius_rem())
    } else {
        theme.resolve_radius(spec.radius_token())
    };

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(size);
        s.descriptor.layout.height = LayoutSizing::Fixed(size);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.flex_none = true;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.background = Some(bg);
    }

    if spec.has_image() {
        let mut image = Node {
            kind: poodle_node::NodeKind::Image {
                source: spec.src.clone().unwrap_or_default(),
            },
            ..Node::default()
        };
        image.style.descriptor.layout.width = LayoutSizing::Fixed(size);
        image.style.descriptor.layout.height = LayoutSizing::Fixed(size);
        root = root.child(image);
    } else {
        let mut initials = Node::text(spec.fallback_text());
        initials.style.descriptor.text_color = Some(fg);
        initials.style.text_size = Some(font_size);
        initials.style.text_weight = Some(600);
        root = root.child(initials);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.a11y.role = Some(NodeRole::Image);
    root
}
