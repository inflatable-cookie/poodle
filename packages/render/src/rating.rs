//! Rating — a row of stars.
//!
//! Contract: `docs/contracts/components/rating.md`
//! Ported from: `packages/jetstream/components/src/rating.rs`.
//!
//! `on_change` fires with the rating the pressed star sets — 1-based.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodePosition, NodeRole, NodeToggled,
};
use poodle_specs::{ControlDensity, ControlSize, RatingSpec};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::{control_height_rem, rem_to_px};

/// Per-size glyph font-size in rem (contract §8 size table).
fn glyph_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Per-density inter-item gap in rem (contract §8).
fn item_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.0625,
        ControlDensity::Default => 0.125,
        ControlDensity::Comfortable => 0.25,
    }
}

pub fn rating(
    spec: &RatingSpec,
    ctx: &RenderContext<'_>,
    on_change: Option<Arc<dyn Fn(u32) + Send + Sync>>,
) -> Node {
    let active = ctx.theme().resolve_color(spec.active_color_token());
    // Contract §8: unfilled color = color-mix(text-secondary 48%, transparent).
    let inactive_base = ctx.theme().resolve_color(spec.inactive_color_token());
    let inactive = with_alpha(inactive_base, inactive_base.3 * spec.inactive_color_alpha());

    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    // Contract §8 glyph font-size; the reference renders the SVG at 1.125em.
    let glyph_px = rem_to_px(glyph_font_rem(effective_size)) * 1.125;
    // Contract §7/§8: each item is a size-scaled square touch target.
    let item_px = rem_to_px(control_height_rem(effective_size));
    // Gap between stars: density-driven (contract §8).
    let gap = rem_to_px(item_gap_rem(density));

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
    }

    for i in 0..spec.max {
        // Contract §2/§4: clipped accent overlay sized by per-star fill ratio.
        let ratio = spec.fill_ratio(i) as f32;
        let fill_w = (glyph_px * ratio).clamp(0.0, glyph_px);

        // Base (unfilled) glyph layer.
        let mut base = Node::icon("star", glyph_px);
        base.style.descriptor.text_color = Some(inactive);

        // Glyph wrapper: relative base + absolute clipped fill overlay.
        // Contract: the stars are mutually exclusive choices, so each is a
        // `radio` naming the value it sets.
        let mut glyph = Node::container();
        glyph.a11y.role = Some(NodeRole::RadioButton);
        glyph.a11y.label = Some(format!("{} of {}", i + 1, spec.max));
        glyph.a11y.toggled = Some(if spec.fill_ratio(i) > 0.0 {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
        glyph.position = NodePosition::Relative;
        {
            let s = &mut glyph.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(glyph_px);
            s.descriptor.layout.height = LayoutSizing::Fixed(glyph_px);
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }
        let mut glyph = glyph.child(base);

        if fill_w > 0.0 {
            let mut fill = Node::container();
            fill.position = NodePosition::Absolute {
                top: Some(0.0),
                left: Some(0.0),
                right: None,
                bottom: None,
            };
            {
                let s = &mut fill.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.height = LayoutSizing::Fixed(glyph_px);
                s.descriptor.layout.width = LayoutSizing::Fixed(fill_w);
                s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
                s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
            }
            let mut filled = Node::icon("star", glyph_px);
            filled.style.descriptor.text_color = Some(active);
            glyph = glyph.child(fill.child(filled));
        }

        // Touch-target wrapper: fixed square hit area per effective size.
        let mut target = Node::container();
        {
            let s = &mut target.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(item_px);
            s.descriptor.layout.height = LayoutSizing::Fixed(item_px);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }
        let mut target = target.child(glyph);

        if let (false, false, Some(handler)) = (spec.is_disabled, spec.is_readonly, &on_change) {
            let handler = Arc::clone(handler);
            let value = (i + 1) as u32;
            target.style.descriptor.cursor = CursorHint::Pointer;
            target.interaction.on_activate = Some(Arc::new(move || handler(value)));
        }

        el = el.child(target);
    }

    if spec.is_disabled {
        el.style.descriptor.opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    // Contract: a rating is a `radiogroup` of stars.
    el.a11y.role = Some(NodeRole::RadioGroup);
    el
}
