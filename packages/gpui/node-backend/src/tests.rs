//! Pure-logic tests: the color edge and the animation sampler. Element
//! construction itself needs a GPUI app/window and is proven by the native
//! visual gate, not by unit tests.

use super::*;
use poodle_node::{ShadowLayer, ShadowValue};

/// Round-trip a ColorValue through the backend's conversion and back to
/// Rgba; the path must be an sRGB identity (alpha included). This pins the
/// old tier's `resolve_color` behavior: no transfer function anywhere.
#[test]
fn color_is_a_raw_srgb_passthrough() {
    let cases = [
        ColorValue(0.0, 0.0, 0.0, 0.0),
        ColorValue(1.0, 1.0, 1.0, 1.0),
        ColorValue(0.2, 0.4, 0.6, 0.8),
        ColorValue(0.8906, 0.4062, 0.1797, 0.22), // eclipse border.default-ish
    ];
    for c in cases {
        let back: gpui::Rgba = color(c).into();
        assert!(
            (back.r - c.0).abs() < 1e-6
                && (back.g - c.1).abs() < 1e-6
                && (back.b - c.2).abs() < 1e-6
                && (back.a - c.3).abs() < 1e-6,
            "passthrough broke for {c:?}: got {back:?}"
        );
    }
}

fn spin_anim() -> NodeAnimation {
    NodeAnimation::spin("test-spin", 1.0)
}

#[test]
fn sample_property_interpolates_linearly_between_keyframes() {
    let anim = spin_anim(); // Rotate 0.0 @ 0.0 → TAU @ 1.0
    let mid = sample_property(&anim, AnimProperty::Rotate, 0.5).unwrap();
    assert!((mid - std::f32::consts::PI).abs() < 1e-4, "mid = {mid}");
    let start = sample_property(&anim, AnimProperty::Rotate, 0.0).unwrap();
    assert_eq!(start, 0.0);
    let end = sample_property(&anim, AnimProperty::Rotate, 1.0).unwrap();
    assert!((end - std::f32::consts::TAU).abs() < 1e-4);
}

#[test]
fn sample_property_clamps_outside_keyframe_range_and_skips_absent_properties() {
    let anim = NodeAnimation {
        key: "fade".into(),
        keyframes: vec![
            poodle_node::AnimKeyframe {
                at: 0.25,
                values: vec![(AnimProperty::Opacity, 0.2)],
            },
            poodle_node::AnimKeyframe {
                at: 0.75,
                values: vec![(AnimProperty::Opacity, 0.8)],
            },
        ],
        duration_secs: 0.4,
        easing: AnimEasing::Linear,
        loop_mode: AnimLoop::Once,
    };
    assert_eq!(
        sample_property(&anim, AnimProperty::Opacity, 0.0),
        Some(0.2)
    );
    assert_eq!(
        sample_property(&anim, AnimProperty::Opacity, 1.0),
        Some(0.8)
    );
    let mid = sample_property(&anim, AnimProperty::Opacity, 0.5).unwrap();
    assert!((mid - 0.5).abs() < 1e-6, "mid = {mid}");
    assert_eq!(sample_property(&anim, AnimProperty::Rotate, 0.5), None);
}

#[test]
fn tooltip_forces_element_state() {
    let mut node = Node::button("ok");
    assert!(
        !needs_state(&node),
        "a plain unfocusable button does not need element state"
    );
    node.tooltip = Some("Save".into());
    assert!(
        needs_state(&node),
        "a tooltip must take the stateful path so GPUI can attach .tooltip()"
    );
    node.tooltip = Some(String::new());
    assert!(!needs_state(&node), "an empty tooltip is not a tooltip");
}

#[test]
fn focusable_nodes_with_a_focus_patch_are_tracked() {
    let mut node = Node::button("grid");
    node.interaction.focusable = true;
    node.id = Some("segmented:grid".into());
    node.runtime_id = Some("segmented:a:option:grid".into());
    assert!(
        !tracks_focus(&node),
        "without a focus patch the backend never creates a retrievable handle"
    );
    node.style.focus = Some(StylePatch {
        border_color: Some(ColorValue(0.3, 0.6, 1.0, 1.0)),
        ..StylePatch::default()
    });
    assert!(tracks_focus(&node));
    assert_eq!(element_id_string(&node), "segmented:a:option:grid");
}

/// A declared focus ring is sufficient for focus tracking on its own — the
/// whole point of the channel: a borderless control with no focus patch still
/// gets a real, retrievable handle, so keyboard entry does not depend on a
/// prior pointer press. Bare `focusable` stays untracked.
#[test]
fn a_declared_focus_ring_is_tracked_and_takes_the_stateful_path() {
    let mut node = Node::button("rerun");
    node.interaction.focusable = true;
    assert!(!tracks_focus(&node));
    node.style.focus_ring = Some(FocusRing {
        color: ColorValue(0.3, 0.6, 1.0, 1.0),
        width: 2.0,
        offset: 2.0,
    });
    assert!(tracks_focus(&node));
    assert!(needs_state(&node));

    // Tracking does not require `focusable`: the ring declares the need.
    let mut ring_only = Node::container();
    ring_only.style.focus_ring = node.style.focus_ring;
    assert!(tracks_focus(&ring_only));
    assert!(needs_state(&ring_only));
}

/// Generated identities belong to one GPUI render thread. A second headless
/// app may reset its own frame while this one is still walking a tree; that
/// must not rewind either counter in this app.
#[test]
fn generated_identity_counters_are_isolated_per_thread() {
    use std::sync::{Arc, Barrier};

    let node = Node::button("proof");
    reset_element_ids();
    assert_eq!(element_id_text(&element_id(&node)), "poodle-node-0");
    assert_eq!(next_gesture_id(), "gesture-0");

    let before_worker_reset = Arc::new(Barrier::new(2));
    let after_main_progress = Arc::new(Barrier::new(2));
    let worker_before = Arc::clone(&before_worker_reset);
    let worker_after = Arc::clone(&after_main_progress);
    let worker = std::thread::spawn(move || {
        let node = Node::button("worker");
        reset_element_ids();
        assert_eq!(element_id_text(&element_id(&node)), "poodle-node-0");
        assert_eq!(next_gesture_id(), "gesture-0");
        worker_before.wait();
        worker_after.wait();
        reset_element_ids();
        assert_eq!(element_id_text(&element_id(&node)), "poodle-node-0");
        assert_eq!(next_gesture_id(), "gesture-0");
    });

    before_worker_reset.wait();
    assert_eq!(element_id_text(&element_id(&node)), "poodle-node-1");
    assert_eq!(next_gesture_id(), "gesture-1");
    after_main_progress.wait();
    worker.join().expect("worker identity proof");

    assert_eq!(element_id_text(&element_id(&node)), "poodle-node-2");
    assert_eq!(next_gesture_id(), "gesture-2");
}

// ── Shadow projection (g15.045) ─────────────────────────────────────
// The adopted GPUI revision's `BoxShadow` carries a real `inset` flag, so the
// backend projects inset (highlight) layers faithfully instead of dropping
// them — the gpui 0.2.2 approximation is gone.

/// The ring is its own paint channel: declaring one must not disturb the
/// element's shadow stack (or any other refinement) — composition, not
/// replacement.
#[test]
fn a_focus_ring_leaves_the_shadow_stack_untouched() {
    let mut node = Node::button("ok");
    node.style.shadow_layers = vec![ShadowLayer {
        offset_x: 0.0,
        offset_y: 2.0,
        blur: 8.0,
        spread: 1.0,
        color: ColorValue(0.0, 0.0, 0.0, 0.2),
        inset: false,
    }];
    node.style.focus_ring = Some(FocusRing {
        color: ColorValue(0.3, 0.6, 1.0, 1.0),
        width: 2.0,
        offset: 2.0,
    });
    let mut el = apply_paint(div(), &node);
    let shadows = el
        .style()
        .box_shadow
        .as_ref()
        .expect("the shadow stack still projects with a ring declared");
    assert_eq!(shadows.len(), 1);
    assert_eq!(f32::from(shadows[0].offset.y), 2.0);
}

#[test]
fn inset_shadow_layers_project_with_the_inset_flag() {
    let mut node = Node::button("ok");
    node.style.shadow_layers = vec![
        ShadowLayer {
            offset_x: 0.0,
            offset_y: 2.0,
            blur: 8.0,
            spread: 1.0,
            color: ColorValue(0.0, 0.0, 0.0, 0.2),
            inset: false,
        },
        ShadowLayer {
            offset_x: 0.0,
            offset_y: 1.0,
            blur: 0.0,
            spread: 0.0,
            color: ColorValue(1.0, 1.0, 1.0, 0.4),
            inset: true,
        },
    ];
    let mut el = apply_paint(div(), &node);
    let shadows = el
        .style()
        .box_shadow
        .as_ref()
        .expect("shadow layers project into the refinement");
    assert_eq!(
        shadows.len(),
        2,
        "inset layers are projected, not filtered out"
    );
    assert!(!shadows[0].inset);
    assert_eq!(f32::from(shadows[0].offset.y), 2.0);
    assert_eq!(f32::from(shadows[0].blur_radius), 8.0);
    assert_eq!(f32::from(shadows[0].spread_radius), 1.0);
    assert!(shadows[1].inset, "the inset layer keeps its inset flag");
    assert_eq!(f32::from(shadows[1].offset.y), 1.0);
}

#[test]
fn fallback_descriptor_shadow_stays_outset() {
    let mut node = Node::button("ok");
    node.style.descriptor.shadow = Some(ShadowValue {
        offset_x: 0.0,
        offset_y: 3.0,
        blur: 6.0,
        color: ColorValue(0.0, 0.0, 0.0, 0.3),
    });
    let mut el = apply_paint(div(), &node);
    let shadows = el
        .style()
        .box_shadow
        .as_ref()
        .expect("the descriptor shadow projects into the refinement");
    assert_eq!(shadows.len(), 1);
    assert!(
        !shadows[0].inset,
        "the one-token descriptor shadow is always a drop shadow"
    );
    assert_eq!(f32::from(shadows[0].offset.y), 3.0);
    assert_eq!(f32::from(shadows[0].spread_radius), 0.0);
}

#[test]
fn gpui_animation_maps_loop_modes_and_easing() {
    let mut once = spin_anim();
    once.loop_mode = AnimLoop::Once;
    assert!(gpui_animation(&once).oneshot);

    let mut looping = spin_anim();
    looping.loop_mode = AnimLoop::Loop;
    assert!(!gpui_animation(&looping).oneshot);

    // PingPong degrades to Loop (documented gpui 0.2.2 gap).
    let mut pingpong = spin_anim();
    pingpong.loop_mode = AnimLoop::PingPong;
    assert!(!gpui_animation(&pingpong).oneshot);

    // Easing endpoints stay pinned regardless of the curve.
    let mut eased = spin_anim();
    eased.easing = AnimEasing::EaseInOut;
    let a = gpui_animation(&eased);
    assert!(((a.easing)(0.0) - 0.0).abs() < 1e-6);
    assert!(((a.easing)(1.0) - 1.0).abs() < 1e-6);
    assert!(((a.easing)(0.5) - 0.5).abs() < 1e-6);
}
