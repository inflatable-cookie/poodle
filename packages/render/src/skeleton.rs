//! Skeleton — loading placeholders that breathe.
//!
//! Contract: `docs/contracts/components/skeleton.md`
//! Ported from: `packages/jetstream/components/src/skeleton.rs`. The shimmer
//! sweep still isn't representable (needs animated gradient stops); the
//! ping-pong opacity pulse stands in, keyed so backends persist the clock
//! across immediate-mode rebuilds.

use poodle_node::{
    AnimEasing, AnimKeyframe, AnimLoop, AnimProperty, ColorValue, CrossAxisAlignment,
    LayoutDirection, LayoutSizing, Node, NodeAnimation,
};
use poodle_specs::{SkeletonPreset, SkeletonSpec};

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Parse a CSS-style dimension ("2rem", "200px") to logical pixels.
fn parse_dim(s: &str) -> Option<f32> {
    if let Some(rem_str) = s.strip_suffix("rem") {
        rem_str.trim().parse::<f32>().ok().map(rem_to_px)
    } else if let Some(px_str) = s.strip_suffix("px") {
        px_str.trim().parse::<f32>().ok()
    } else {
        s.trim().parse::<f32>().ok()
    }
}

/// Contract §8 shimmer fill: flat mid-tone between the base outer stop and
/// the centre highlight — one flat tone standing in for the gradient.
pub fn shimmer_fill(spec: &SkeletonSpec, ctx: &RenderContext<'_>) -> ColorValue {
    let base = ctx.theme().resolve_color(spec.shimmer_base_token());
    let base = with_alpha(base, base.3 * 0.88);
    let surface = ctx.theme().resolve_color(spec.shimmer_highlight_token());
    let white = ColorValue(1.0, 1.0, 1.0, 1.0);
    let highlight = mix_srgb(surface, white, 0.92);
    mix_srgb(highlight, base, 0.5)
}

fn rounded(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

fn shape(fill: ColorValue, r: f32) -> Node {
    let mut n = Node::container();
    n.style.descriptor.background = Some(fill);
    rounded(&mut n, r);
    n
}

/// Ping-pong opacity pulse — the shimmer stand-in.
fn skeleton_pulse() -> NodeAnimation {
    NodeAnimation {
        key: "poodle-skeleton".to_string(),
        keyframes: vec![
            AnimKeyframe {
                at: 0.0,
                values: vec![(AnimProperty::Opacity, 0.5)],
            },
            AnimKeyframe {
                at: 0.5,
                values: vec![(AnimProperty::Opacity, 0.8)],
            },
            AnimKeyframe {
                at: 1.0,
                values: vec![(AnimProperty::Opacity, 0.5)],
            },
        ],
        duration_secs: 1.6,
        easing: AnimEasing::EaseInOut,
        loop_mode: AnimLoop::Loop,
    }
}

/// Single-shape skeleton (no preset).
fn single_shape(fill: ColorValue, radius: f32, spec: &SkeletonSpec) -> Node {
    let parsed_w = spec.width.as_deref().and_then(parse_dim);
    let parsed_h = spec.height.as_deref().and_then(parse_dim);

    let mut el = shape(fill, radius);
    let s = &mut el.style;
    match spec.shape.as_str() {
        "circle" => {
            let side = parsed_w.or(parsed_h).unwrap_or(rem_to_px(2.5));
            s.descriptor.layout.width = LayoutSizing::Fixed(side);
            s.descriptor.layout.height = LayoutSizing::Fixed(side);
        }
        "block" => {
            s.descriptor.layout.height = LayoutSizing::Fixed(parsed_h.unwrap_or(rem_to_px(6.0)));
            match parsed_w {
                Some(w) => s.descriptor.layout.width = LayoutSizing::Fixed(w),
                None => s.fill_width = true,
            }
        }
        _ => {
            s.descriptor.layout.height = LayoutSizing::Fixed(parsed_h.unwrap_or(rem_to_px(0.875)));
            match parsed_w {
                Some(w) => s.descriptor.layout.width = LayoutSizing::Fixed(w),
                None => s.fill_width = true,
            }
        }
    }
    el
}

pub fn skeleton(spec: &SkeletonSpec, ctx: &RenderContext<'_>) -> Node {
    let fill = shimmer_fill(spec, ctx);
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    let pill_radius = ctx.theme().resolve_radius("radius.pill");
    let surface_radius = ctx.theme().resolve_radius("radius.surface");

    let gap_075 = rem_to_px(0.75);
    let gap_0625 = rem_to_px(0.625);
    let gap_05 = rem_to_px(0.5);
    let gap_0375 = rem_to_px(0.375);
    let gap_025 = rem_to_px(0.25);

    let line_h = rem_to_px(0.875);
    let line_sm_h = rem_to_px(0.6875);

    let rect = |w_px: f32, h_px: f32| -> Node {
        let mut n = shape(fill, radius);
        n.style.descriptor.layout.width = LayoutSizing::Fixed(w_px);
        n.style.descriptor.layout.height = LayoutSizing::Fixed(h_px);
        n
    };
    // Proportional cell for ROW contexts: flex-basis weight in a growing row.
    let cell_pct = |pct: f32, h_px: f32| -> Node {
        let mut n = shape(fill, radius);
        n.style.descriptor.layout.height = LayoutSizing::Fixed(h_px);
        n.style.width_pct = Some(pct);
        n
    };
    // True percentage line for COLUMN contexts.
    let line_pct = |pct: f32, h_px: f32| -> Node {
        let mut n = shape(fill, radius);
        n.style.descriptor.layout.height = LayoutSizing::Fixed(h_px);
        n.style.width_pct = Some(pct);
        n
    };
    let line_full = |h_px: f32| -> Node { line_pct(1.0, h_px) };
    let circle = |side: f32| -> Node {
        let mut n = shape(fill, pill_radius);
        n.style.descriptor.layout.width = LayoutSizing::Fixed(side);
        n.style.descriptor.layout.height = LayoutSizing::Fixed(side);
        n
    };
    let row = |gap: f32| -> Node {
        let mut n = Node::container();
        n.style.descriptor.layout.direction = LayoutDirection::Row;
        n.style.descriptor.layout.spacing.gap = gap;
        n
    };
    let col = |gap: f32| -> Node {
        let mut n = Node::container();
        n.style.descriptor.layout.direction = LayoutDirection::Column;
        n.style.descriptor.layout.spacing.gap = gap;
        n
    };

    let Some(ref preset) = spec.preset else {
        let mut el = single_shape(fill, radius, spec);
        if spec.is_animated {
            el.style.animation = crate::motion::loop_animation_for_policy(
                ctx.motion_policy(),
                skeleton_pulse(),
                ctx.first_frame_committed(),
            );
        }
        return el;
    };

    let mut built = match preset {
        SkeletonPreset::AvatarLine => {
            let mut r = row(gap_075);
            r.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            r.child(circle(rem_to_px(2.25)))
                .child(rect(rem_to_px(10.0), line_h))
        }
        SkeletonPreset::ListItem => {
            let mut text_col = col(gap_0375);
            text_col.style.flex_fill = true;
            let text_col = text_col
                .child(line_pct(0.60, line_h))
                .child(line_pct(0.40, line_sm_h));
            let mut r = row(gap_075);
            r.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            r.style.fill_width = true;
            r.style.descriptor.layout.spacing.padding.top = gap_05;
            r.style.descriptor.layout.spacing.padding.bottom = gap_05;
            r.child(circle(rem_to_px(2.25))).child(text_col)
        }
        SkeletonPreset::TableRow => {
            let mut table_row = row(gap_075);
            table_row.style.fill_width = true;
            table_row.style.descriptor.layout.spacing.padding.top = gap_0625;
            table_row.style.descriptor.layout.spacing.padding.bottom = gap_0625;
            table_row
                .child(cell_pct(0.40, line_h))
                .child(cell_pct(0.60, line_h))
                .child(cell_pct(0.60, line_h))
                .child(cell_pct(0.20, line_h))
        }
        SkeletonPreset::Card => {
            let block_radius = (surface_radius - rem_to_px(0.375)).max(0.0);
            let mut header = shape(fill, block_radius);
            header.style.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(6.0));
            header.style.fill_width = true;

            let mut body = col(gap_0375)
                .child(line_pct(0.80, line_h))
                .child(line_full(line_h))
                .child(line_pct(0.60, line_h));
            body.style.fill_width = true;

            let pill = || {
                let mut p = shape(fill, pill_radius);
                p.style.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(3.5));
                p.style.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(1.25));
                p
            };
            let mut footer = row(gap_05);
            footer.style.descriptor.layout.spacing.padding.top = gap_025;
            let footer = footer.child(pill()).child(pill());

            let mut card = col(gap_075);
            card.style.fill_width = true;
            {
                let pad = &mut card.style.descriptor.layout.spacing.padding;
                let p = rem_to_px(1.0);
                pad.left = p;
                pad.right = p;
                pad.top = p;
                pad.bottom = p;
            }
            card.child(header).child(body).child(footer)
        }
        SkeletonPreset::DetailSection => {
            let mut section = col(gap_0625).child(rect(rem_to_px(8.0), rem_to_px(1.0)));
            section.style.fill_width = true;
            for _ in 0..spec.lines {
                let mut label = shape(fill, radius);
                label.style.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(6.0));
                label.style.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(0.75));
                label.style.flex_shrink_zero = true;

                let mut value = shape(fill, radius);
                value.style.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(0.75));
                value.style.flex_fill = true;
                value.style.max_width = Some(rem_to_px(14.0));

                let mut r = row(rem_to_px(1.0));
                r.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                section = section.child(r.child(label).child(value));
            }
            section
        }
    };
    // The whole skeleton breathes; opacity cascades to every shape.
    if spec.is_animated {
        built.style.animation = crate::motion::loop_animation_for_policy(
            ctx.motion_policy(),
            skeleton_pulse(),
            ctx.first_frame_committed(),
        );
    }
    built
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn static_skeleton_has_no_animation() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = skeleton(&SkeletonSpec::new().with_animated(false), &ctx);
        assert!(node.style.animation.is_none());
    }

    #[test]
    fn list_and_table_presets_keep_reference_padding() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let list = skeleton(
            &SkeletonSpec::new().with_preset(SkeletonPreset::ListItem),
            &ctx,
        );
        assert_eq!(
            list.style.descriptor.layout.spacing.padding.top,
            rem_to_px(0.5)
        );
        assert!(list.style.fill_width);

        let table = skeleton(
            &SkeletonSpec::new().with_preset(SkeletonPreset::TableRow),
            &ctx,
        );
        assert_eq!(
            table.style.descriptor.layout.spacing.padding.top,
            rem_to_px(0.625)
        );
        assert!(table.style.fill_width);
    }

    #[test]
    fn initial_construction_schedules_no_pulse() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = skeleton(&SkeletonSpec::new(), &ctx);
        assert!(node.style.animation.is_none());
        let after = skeleton(&SkeletonSpec::new(), &ctx.with_first_frame_committed(true));
        assert!(after.style.animation.is_some());
    }
}
