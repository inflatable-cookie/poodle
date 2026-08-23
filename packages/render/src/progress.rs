//! Progress — determinate bar or indeterminate 40% affordance.
//!
//! Contract: `docs/contracts/components/progress.md`
//! Ported from: `packages/jetstream/components/src/progress.rs`.

use poodle_node::{LayoutDirection, Node, NodeKind, NodeRole};
use poodle_specs::ProgressSpec;

use crate::color::{mix_srgb, WHITE};
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Indeterminate bar width as a fraction of the track (contract §8: 40%),
/// expressed as flex-grow against a trailing spacer.
const INDETERMINATE_BAR_WIDTH_FRAC: f32 = 0.4;

pub fn progress(spec: &ProgressSpec, ctx: &RenderContext<'_>) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let accent = ctx.theme().resolve_color(spec.indicator_fill_token());

    // Contract §8 Root: track bg = color-mix(surface 96%, text-primary) —
    // sRGB-space, like every other recipe.
    let surface = ctx.theme().resolve_color(spec.track_fill_token());
    let track_mix = ctx.theme().resolve_color(spec.track_mix_token());
    let track_bg = mix_srgb(surface, track_mix, spec.track_mix_ratio());

    // Contract §8 Indicator gradient: color-mix(accent 88%, white) → accent.
    let gradient_lead = mix_srgb(accent, WHITE, spec.indicator_gradient_accent_ratio());

    let track_height = rem_to_px(ProgressSpec::min_height_rem(effective_size));

    let mut root = match spec.normalized_progress() {
        Some(frac) => {
            // Determinate: the backend's progress widget fills proportionally.
            // The gradient stays off this path for the same reason as the old
            // tier: fill fraction and gradient share one quad.
            let mut bar = Node {
                kind: NodeKind::Progress {
                    fraction: frac as f32,
                },
                ..Node::default()
            };
            {
                let s = &mut bar.style;
                s.min_height = Some(track_height);
                s.self_stretch = true;
                s.descriptor.corner_radii.top_left = 999.0;
                s.descriptor.corner_radii.top_right = 999.0;
                s.descriptor.corner_radii.bottom_right = 999.0;
                s.descriptor.corner_radii.bottom_left = 999.0;
                s.descriptor.background = Some(track_bg);
            }
            bar
        }
        None => {
            // Indeterminate: a 40% bar + 60% spacer via flex-grow factors.
            let mut bar = Node::container();
            {
                let s = &mut bar.style;
                s.min_height = Some(track_height);
                s.descriptor.corner_radii.top_left = 999.0;
                s.descriptor.corner_radii.top_right = 999.0;
                s.descriptor.corner_radii.bottom_right = 999.0;
                s.descriptor.corner_radii.bottom_left = 999.0;
                s.gradient = Some((90.0, vec![(gradient_lead, 0.0), (accent, 1.0)]));
                s.flex_grow = Some(INDETERMINATE_BAR_WIDTH_FRAC);
            }
            let mut spacer = Node::container();
            spacer.style.min_height = Some(track_height);
            spacer.style.flex_grow = Some(1.0 - INDETERMINATE_BAR_WIDTH_FRAC);

            let mut track = Node::container();
            {
                let s = &mut track.style;
                s.min_height = Some(track_height);
                s.self_stretch = true;
                s.descriptor.corner_radii.top_left = 999.0;
                s.descriptor.corner_radii.top_right = 999.0;
                s.descriptor.corner_radii.bottom_right = 999.0;
                s.descriptor.corner_radii.bottom_left = 999.0;
                s.descriptor.background = Some(track_bg);
                s.descriptor.layout.direction = LayoutDirection::Row;
            }
            track.child(bar).child(spacer)
        }
    };

    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.a11y.role = Some(NodeRole::ProgressIndicator);
    root
}
