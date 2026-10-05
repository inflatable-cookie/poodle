//! Progress — determinate bar or indeterminate 40% affordance.
//!
//! Contract: docs/contracts/components/progress.md
//! Ported from: packages/jetstream/components/src/progress.rs.

use poodle_node::{
    AnimEasing, AnimKeyframe, AnimLoop, AnimProperty, LayoutDirection, LayoutOverflow, Node,
    NodeAnimation, NodeKind, NodeRole,
};
use poodle_specs::ProgressSpec;

use crate::color::{mix_srgb, WHITE};
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Indeterminate bar width as a fraction of the track (contract §8: 40%).
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
    let fraction = if spec.is_indeterminate {
        INDETERMINATE_BAR_WIDTH_FRAC
    } else {
        spec.normalized_progress().unwrap_or(0.0) as f32
    };

    let mut track = if spec.is_indeterminate {
        Node::container()
    } else {
        Node {
            kind: NodeKind::Progress { fraction },
            ..Node::default()
        }
    };
    {
        let s = &mut track.style;
        s.fill_width = true;
        s.min_height = Some(track_height);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.descriptor.corner_radii.top_left = 999.0;
        s.descriptor.corner_radii.top_right = 999.0;
        s.descriptor.corner_radii.bottom_right = 999.0;
        s.descriptor.corner_radii.bottom_left = 999.0;
        s.descriptor.background = Some(track_bg);
    }

    let mut indicator = Node::container();
    {
        let s = &mut indicator.style;
        s.width_pct = Some(fraction.clamp(0.0, 1.0));
        s.fill_height = true;
        s.self_stretch = true;
        s.min_height = Some(track_height);
        s.descriptor.text_color = Some(accent);
        s.descriptor.corner_radii.top_left = 999.0;
        s.descriptor.corner_radii.top_right = 999.0;
        s.descriptor.corner_radii.bottom_right = 999.0;
        s.descriptor.corner_radii.bottom_left = 999.0;
        s.gradient = Some((90.0, vec![(gradient_lead, 0.0), (accent, 1.0)]));
    }
    if spec.is_indeterminate {
        indicator.style.animation = crate::motion::loop_animation_for_policy(
            ctx.motion_policy(),
            NodeAnimation {
                key: "poodle-progress-indeterminate".to_owned(),
                // The GPUI backend maps these parent-relative offsets to
                // match Svelte's translateX(-100%) to translateX(250%) sweep
                // for an indicator that occupies 40% of the track.
                keyframes: vec![
                    AnimKeyframe {
                        at: 0.0,
                        values: vec![(AnimProperty::TranslateX, -INDETERMINATE_BAR_WIDTH_FRAC)],
                    },
                    AnimKeyframe {
                        at: 1.0,
                        values: vec![(AnimProperty::TranslateX, 1.0)],
                    },
                ],
                duration_secs: 1.2,
                easing: AnimEasing::EaseInOut,
                loop_mode: AnimLoop::Loop,
            },
            ctx.first_frame_committed(),
        );
    }
    track = track.child(indicator);

    if let Some(label) = spec.aria_label.as_deref() {
        track.a11y.label = Some(label.to_string());
    }
    track.a11y.role = Some(NodeRole::ProgressIndicator);
    if !spec.is_indeterminate {
        let safe_max = if spec.max <= 0.0 { 100.0 } else { spec.max };
        track.a11y.value_min = Some(0.0);
        track.a11y.value_max = Some(safe_max);
        track.a11y.value = spec.value.map(|value| value.clamp(0.0, safe_max));
    }
    track.a11y.value_text = spec.value_text.clone().or_else(|| {
        if spec.is_indeterminate {
            None
        } else {
            spec.normalized_progress()
                .map(|fraction| format!("{}%", (fraction * 100.0).round() as i64))
        }
    });
    track
}
