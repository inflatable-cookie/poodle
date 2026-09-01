//! Native animation declarations under architecture 012.
//!
//! `poodle-node` still owns the property vocabulary. This module decides
//! whether a declaration may schedule a clock for the effective policy.

use poodle_headless::motion_policy::{
    filter_motion_properties, gpui_motion_plan, should_run_motion_loop, GpuiApproximation,
    MotionPolicy, MotionProperty,
};
use poodle_node::{AnimLoop, AnimProperty, NodeAnimation};

fn map_property(property: AnimProperty) -> MotionProperty {
    match property {
        AnimProperty::Opacity => MotionProperty::Opacity,
        AnimProperty::Rotate => MotionProperty::Rotate,
        AnimProperty::TranslateX => MotionProperty::TranslateX,
        AnimProperty::TranslateY => MotionProperty::TranslateY,
        AnimProperty::ScaleX => MotionProperty::ScaleX,
        AnimProperty::ScaleY => MotionProperty::ScaleY,
    }
}

fn requested_properties(animation: &NodeAnimation) -> Vec<MotionProperty> {
    let mut requested = Vec::new();
    for frame in &animation.keyframes {
        for (property, _) in &frame.values {
            let mapped = map_property(*property);
            if !requested.contains(&mapped) {
                requested.push(mapped);
            }
        }
    }
    requested
}

/// Filter a declared animation through the effective host policy.
///
/// Frozen and disallowed reduced loops return `None`. Reduced may keep a
/// short non-looping opacity phase when `reduced_opacity` is set.
pub fn animation_for_policy(
    policy: MotionPolicy,
    mut animation: NodeAnimation,
    reduced_opacity: bool,
) -> Option<NodeAnimation> {
    let looped = animation.loop_mode != AnimLoop::Once;
    let allowed = filter_motion_properties(
        policy,
        &requested_properties(&animation),
        looped,
        reduced_opacity,
    );
    if allowed.is_empty() {
        return None;
    }
    for frame in &mut animation.keyframes {
        frame
            .values
            .retain(|(property, _)| allowed.contains(&map_property(*property)));
    }
    animation.keyframes.retain(|frame| !frame.values.is_empty());
    if animation.keyframes.len() < 2 {
        return None;
    }
    Some(animation)
}

/// Loops may attach only in full after the host commits the baseline frame.
pub fn loop_animation_for_policy(
    policy: MotionPolicy,
    animation: NodeAnimation,
    first_frame_committed: bool,
) -> Option<NodeAnimation> {
    if !should_run_motion_loop(policy, true, first_frame_committed) {
        None
    } else {
        animation_for_policy(policy, animation, false)
    }
}

pub fn named_gpui_approximation(animation: &NodeAnimation) -> GpuiApproximation {
    let requested = requested_properties(animation);
    gpui_motion_plan(&requested).approximation
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::{AnimEasing, AnimKeyframe};

    fn toast_enter() -> NodeAnimation {
        NodeAnimation {
            key: "poodle-toast-1".into(),
            keyframes: vec![
                AnimKeyframe {
                    at: 0.0,
                    values: vec![
                        (AnimProperty::Opacity, 0.0),
                        (AnimProperty::TranslateY, 8.0),
                    ],
                },
                AnimKeyframe {
                    at: 1.0,
                    values: vec![
                        (AnimProperty::Opacity, 1.0),
                        (AnimProperty::TranslateY, 0.0),
                    ],
                },
            ],
            duration_secs: 0.18,
            easing: AnimEasing::EaseOut,
            loop_mode: AnimLoop::Once,
        }
    }

    #[test]
    fn frozen_schedules_no_clock() {
        assert!(animation_for_policy(
            MotionPolicy::Frozen,
            NodeAnimation::spin("ring", 0.8),
            false
        )
        .is_none());
    }

    #[test]
    fn reduced_drops_loops_and_keeps_toast_opacity() {
        assert!(animation_for_policy(
            MotionPolicy::Reduced,
            NodeAnimation::spin("ring", 0.8),
            false
        )
        .is_none());
        let reduced = animation_for_policy(MotionPolicy::Reduced, toast_enter(), true).unwrap();
        assert!(reduced.keyframes.iter().all(|frame| {
            frame
                .values
                .iter()
                .all(|(property, _)| *property == AnimProperty::Opacity)
        }));
        assert_eq!(
            named_gpui_approximation(&toast_enter()),
            GpuiApproximation::OpacityStandIn
        );
    }

    #[test]
    fn height_has_no_native_channel_and_is_a_static_endpoint() {
        assert_eq!(
            gpui_motion_plan(&[MotionProperty::Height]).approximation,
            GpuiApproximation::StaticEndpoint
        );
    }

    #[test]
    fn loop_animation_for_policy_waits_for_first_frame() {
        let spin = NodeAnimation::spin("ring", 0.8);
        assert!(loop_animation_for_policy(MotionPolicy::Full, spin.clone(), false).is_none());
        assert!(loop_animation_for_policy(MotionPolicy::Full, spin, true).is_some());
    }
}
