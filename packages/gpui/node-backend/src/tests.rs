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
        "a tooltip must take the stateful path so the backend can attach its lifecycle runtime"
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

// ── Shadow projection ───────────────────────────────────────────────
// crates.io gpui 0.2.2 `BoxShadow` has no inset flag, so the refinement
// carries drop layers only and `inset_shadow` paints the inset ones. The two
// halves are complementary: no layer is lost. `inset_shadow`'s own tests pin
// the band geometry; these pin the split.

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

/// Drop layers project into the shadow refinement exactly. Inset layers do
/// not appear there — not because they are lost, but because
/// `inset_shadow::apply` paints them, which the next tests cover.
#[test]
fn the_shadow_refinement_carries_drop_layers_only() {
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
        .expect("the drop layer projects into the refinement");
    assert_eq!(shadows.len(), 1, "the inset layer is not a drop shadow");
    assert_eq!(f32::from(shadows[0].offset.y), 2.0);
    assert_eq!(f32::from(shadows[0].blur_radius), 8.0);
    assert_eq!(f32::from(shadows[0].spread_radius), 1.0);
}

/// An all-inset stack declares no drop shadow at all, so the refinement stays
/// absent rather than becoming an empty shadow list.
#[test]
fn an_all_inset_shadow_stack_declares_no_drop_shadow() {
    let mut node = Node::button("ok");
    node.style.shadow_layers = vec![ShadowLayer {
        offset_x: 0.0,
        offset_y: 1.0,
        blur: 0.0,
        spread: 0.0,
        color: ColorValue(1.0, 1.0, 1.0, 0.4),
        inset: true,
    }];
    let mut el = apply_paint(div(), &node);
    assert!(
        el.style().box_shadow.is_none(),
        "no drop layer means no shadow refinement"
    );
}

#[test]
fn the_fallback_descriptor_shadow_projects_its_single_drop_layer() {
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
    assert_eq!(f32::from(shadows[0].offset.y), 3.0);
    assert_eq!(f32::from(shadows[0].spread_radius), 0.0);
}

// ── Inset shadow band geometry (g16.005) ────────────────────────────
//
// Every inset layer Poodle declares has `blur == 0`, which makes a CSS inset
// shadow exactly a solid band inside the padding box. These pin the geometry
// against the five real declarations in `poodle-render`.

fn inset(offset_x: f32, offset_y: f32, spread: f32) -> ShadowLayer {
    ShadowLayer {
        offset_x,
        offset_y,
        blur: 0.0,
        spread,
        color: ColorValue(1.0, 1.0, 1.0, 0.4),
        inset: true,
    }
}

/// Tabs drop target, ListCard highlighted, ActionDiscoveryPanel active: a
/// spread-only layer is an even ring on all four sides.
#[test]
fn a_spread_only_inset_layer_is_an_even_inner_ring() {
    let bands = crate::inset_shadow::band_widths(&inset(0.0, 0.0, 2.0));
    assert_eq!(bands.left, 2.0);
    assert_eq!(bands.right, 2.0);
    assert_eq!(bands.top, 2.0);
    assert_eq!(bands.bottom, 2.0);
}

/// Popover and Accordion: a downward offset with no spread is a band along
/// the TOP inner edge only — the highlight line both contracts declare.
#[test]
fn a_downward_offset_inset_layer_is_a_top_edge_band() {
    let bands = crate::inset_shadow::band_widths(&inset(0.0, 1.0, 0.0));
    assert_eq!(bands.top, 1.0);
    assert_eq!(bands.bottom, 0.0, "a negative band never paints");
    assert_eq!(bands.left, 0.0);
    assert_eq!(bands.right, 0.0);
}

/// ListCard active: a leading-edge bar, clipped by the card's radius. That
/// clipping is why the contract uses an inset shadow rather than a child.
#[test]
fn a_rightward_offset_inset_layer_is_a_leading_edge_band() {
    let bands = crate::inset_shadow::band_widths(&inset(3.0, 0.0, 0.0));
    assert_eq!(bands.left, 3.0);
    assert_eq!(bands.right, 0.0);
    assert_eq!(bands.top, 0.0);
    assert_eq!(bands.bottom, 0.0);
}

/// The other three directions, so the derivation is pinned rather than the
/// two shapes that happen to exist today.
#[test]
fn inset_band_widths_follow_the_shadow_rect_in_every_direction() {
    let up = crate::inset_shadow::band_widths(&inset(0.0, -1.5, 0.0));
    assert_eq!(up.bottom, 1.5);
    assert_eq!(up.top, 0.0);

    let left = crate::inset_shadow::band_widths(&inset(-4.0, 0.0, 0.0));
    assert_eq!(left.right, 4.0);
    assert_eq!(left.left, 0.0);

    // Offset and spread combine: the shadow rect moves AND shrinks, so the
    // near side grows by both and the far side by their difference.
    let both = crate::inset_shadow::band_widths(&inset(0.0, 2.0, 1.0));
    assert_eq!(both.top, 3.0);
    assert_eq!(both.bottom, 0.0, "spread 1 - offset 2 clamps at zero");

    let offset_smaller = crate::inset_shadow::band_widths(&inset(0.0, 1.0, 3.0));
    assert_eq!(offset_smaller.top, 4.0);
    assert_eq!(offset_smaller.bottom, 2.0);
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

// ── g16.008 painted text-state identity ────────────────────────────────────

fn caret_text(id: &str, content: &str) -> Node {
    let mut node = Node::text(content).with_caret(
        (0, 0),
        ColorValue(1.0, 1.0, 1.0, 1.0),
        ColorValue(0.3, 0.6, 1.0, 0.4),
    );
    node.id = Some(id.to_owned());
    node
}

/// Transient text state and undo history are keyed by the node that *paints*
/// the value, and the two shapes disagree about which node that is: a
/// composite field paints a derived value child, a childless input paints
/// itself. Keys and focus land on the root in both cases, so the root has to
/// resolve the difference — in one helper, from the tree's own shape.
#[test]
fn the_painted_text_key_follows_the_node_that_draws_the_value() {
    // Composite field: affixes and a counter around a derived value child.
    let mut composite = Node::input("kick", "Name it");
    composite.id = Some("poodle-input-name".into());
    composite = composite
        .child(Node::icon("search", 12.0))
        .child(caret_text("poodle-input-name-value", "kick"))
        .child(Node::text("4/6"));
    assert_eq!(
        input_text::painted_key(&composite, "poodle-input-name"),
        "poodle-input-name-value",
        "the value child paints, so it owns the measured line and the history"
    );

    // Childless input: native EditableLabel's editing field, which draws its
    // own value. Deriving `-value` here addressed a node that never existed.
    let mut childless = Node::input("kick", "Name it");
    childless.id = Some("track-label".into());
    assert_eq!(
        input_text::painted_key(&childless, "track-label"),
        "track-label",
        "a childless input is its own painted value node"
    );

    // Nothing paints a value: the root id is the only honest answer, and no
    // state is recorded under it either.
    let mut inert = Node::container();
    inert.id = Some("code-input-row".into());
    let inert = inert.child(Node::text("1")).child(Node::text("2"));
    assert_eq!(
        input_text::painted_key(&inert, "code-input-row"),
        "code-input-row"
    );
}

/// The helper resolves an element id the way the rest of the backend does, so
/// a runtime-stamped value node is addressed by the id it actually painted
/// under rather than by the `-value` naming convention.
#[test]
fn the_painted_text_key_uses_the_element_id_the_backend_paints_under() {
    let mut value = caret_text("row-value", "kick");
    value.runtime_id = Some("list:row-7:value".into());
    let mut field = Node::input("kick", "");
    field.id = Some("row".into());
    let field = field.child(value);
    assert_eq!(input_text::painted_key(&field, "row"), "list:row-7:value");

    // A multiline value falls back to the plain wrapped text child, which
    // measures nothing — so it is not a painted text node.
    let mut multiline = Node::input("one\ntwo", "");
    multiline.id = Some("body".into());
    assert_eq!(input_text::painted_key(&multiline, "body"), "body");
}

/// `forget` is the blur reset. It drops everything that describes the field
/// as it is being edited, and keeps the undo history, which reaches back
/// across a focus excursion for as long as the field is mounted.
#[test]
fn blur_clears_transient_text_state_and_keeps_undo_history() {
    let id = "blur-reset-field";
    input_text::record(id, "kick", (4, 4));
    input_text::record(id, "kicks", (5, 5));
    input_text::set_marked(id, (0, 2));
    input_text::set_composing(id, "ki".to_owned());
    let before = painted_text_state_for(id);
    assert!(before.history && before.marked && before.composing);

    input_text::forget(id);
    let after = painted_text_state_for(id);
    assert_eq!(
        after,
        PaintedTextState {
            history: true,
            ..PaintedTextState::default()
        },
        "blur clears the transient entries and nothing else"
    );
    assert_eq!(
        input_text::undo(id).map(|snapshot| snapshot.value),
        Some("kick".to_owned()),
        "history survives the focus excursion"
    );
}

#[test]
fn tooltip_contract_delay_is_300ms() {
    assert_eq!(
        crate::tooltip::TOOLTIP_DELAY,
        std::time::Duration::from_millis(300),
        "Poodle tooltip open delay contract is exactly 300ms"
    );
}

#[test]
fn tooltip_state_reset_increments_generation_and_clears_fields() {
    let mut state = crate::tooltip::WindowTooltipState {
        target_id: Some("target-1".into()),
        text: Some("Tooltip text".into()),
        target_bounds: Some(gpui::Bounds {
            origin: gpui::point(gpui::px(10.0), gpui::px(20.0)),
            size: gpui::size(gpui::px(100.0), gpui::px(40.0)),
        }),
        generation: 41,
        is_visible: true,
        is_hovered: true,
        is_focused: true,
        painted_this_frame: true,
        task: None,
    };

    state.reset();

    assert_eq!(state.target_id, None);
    assert_eq!(state.text, None);
    assert_eq!(state.target_bounds, None);
    assert_eq!(state.generation, 42);
    assert!(!state.is_visible);
    assert!(!state.is_hovered);
    assert!(!state.is_focused);
    assert!(!state.painted_this_frame);
    assert!(state.task.is_none());
}
