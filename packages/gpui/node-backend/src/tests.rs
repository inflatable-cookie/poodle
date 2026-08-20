//! Pure-logic tests: the color edge and the animation sampler. Element
//! construction itself needs a GPUI app/window and is proven by the native
//! visual gate, not by unit tests.

use super::*;

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
fn paint_icon_name_falls_back_to_a_real_asset() {
    assert_eq!(paint_icon_name("audio-waveform"), "audio-waveform");
    assert_eq!(paint_icon_name("piano"), "piano");
    assert_eq!(paint_icon_name("spinner"), "loader-circle");
    assert_eq!(paint_icon_name("not-a-real-icon"), "circle-x");
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
