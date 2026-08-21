//! ResizeHandle — a draggable divider.
//!
//! Contract: `docs/contracts/components/resize-handle.md`
//! Ported from: `packages/jetstream/components/src/resize_handle.rs`.
//!
//! `on_resize` carries the drag's per-frame delta along the handle's axis —
//! pixels, signed. The handle cannot know the panes' sizes, so an absolute
//! position would be a guess; a delta is a fact, and the host applies it to
//! the ratio it already holds. Start and end mark the gesture's bounds for
//! hosts that commit on release.
//!
//! Keyboard resize rides the same seam (contract §5–6): an arrow along the
//! axis is a one-frame gesture with an ±8px delta, Home/End a saturating one.
//! A second keyboard-only callback would give hosts two ways to hear the same
//! event and one more to forget.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeDragEvent, NodeDragPhase, NodeKey, NodeModifiers, NodePosition, NodeRole, StylePatch,
};
use poodle_specs::{Orientation, ResizeHandleSpec};

use crate::presentation::rem_to_px;

/// Where in the gesture a resize event sits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizePhase {
    Start,
    Move,
    End,
}

/// Pixels one axis arrow key moves the split. Contract §6.
const KEY_STEP_PX: f32 = 8.0;

/// Home/End delta. The handle knows no pane bounds, so "go to the end" is a
/// delta far past any of them and the host's own clamp decides where it lands
/// — the same sentinel the web implementations emit. Contract §6.
const KEY_SATURATING_PX: f32 = 9999.0;

/// The backend-state key the handle's focusable root carries.
///
/// Built from the caller's instance scope and nothing else. Orientation, name
/// and value are semantics: two handles may legitimately share all three, and
/// a name that changes — a translation, a host relabelling a pane — would move
/// the key of a control that never went anywhere. Hosts that drive focus or
/// read bounds call this instead of rebuilding the format.
pub fn resize_handle_focus_id(spec: &ResizeHandleSpec) -> String {
    format!("resize-handle:{}", spec.instance_id)
}

/// The resize delta a key asks for, or `None` when the key is not this
/// handle's business. Contract §6: a horizontal handle answers Left/Right, a
/// vertical one Up/Down, both answer Home/End, and a cross-axis arrow is left
/// alone so it can keep meaning what it means to whatever owns the surface.
fn key_step(orientation: Orientation, key: NodeKey) -> Option<f32> {
    let horizontal = matches!(orientation, Orientation::Horizontal);
    match key {
        NodeKey::ArrowLeft if horizontal => Some(-KEY_STEP_PX),
        NodeKey::ArrowRight if horizontal => Some(KEY_STEP_PX),
        NodeKey::ArrowUp if !horizontal => Some(-KEY_STEP_PX),
        NodeKey::ArrowDown if !horizontal => Some(KEY_STEP_PX),
        NodeKey::Home => Some(-KEY_SATURATING_PX),
        NodeKey::End => Some(KEY_SATURATING_PX),
        _ => None,
    }
}

pub fn resize_handle(
    spec: &ResizeHandleSpec,
    theme: &dyn ThemeProvider,
    on_resize: Option<Arc<dyn Fn(ResizePhase, f32) + Send + Sync>>,
) -> Node {
    let handle_color = theme.resolve_color(spec.border_color_token());
    // Contract §8 hover/dragging: line recolors to accent-base.
    let hover_color = theme.resolve_color(spec.hover_color_token());
    let focus_color = theme.resolve_color(spec.focus_ring_color_token());
    let is_disabled = spec.is_disabled;

    // Contract §7: the root is only as thick as the line (0.125rem), so the
    // divider costs no layout space beyond the hairline. The grab area
    // (0.5rem) is an absolutely positioned overlay centred on the line, which
    // overlaps the neighbouring regions instead of widening the gap.
    let visual_size = rem_to_px(spec.thickness_rem());
    let hit_size = rem_to_px(spec.hit_size_rem());
    let hit_offset = rem_to_px(spec.hit_offset_rem());

    // The affordance line is the root itself. Contract §7 already has the line
    // at `inset: 0` — the two are the same pixels — and only the node that
    // holds focus can show a focus state, so a separate line child would take
    // the paint out of reach of the focus channel. Contract §8 hover/dragging
    // recolors it to accent-base; focus-visible recolors it to the accent
    // focus ring (see §10: gpui has no outline that costs no layout).
    let paint = |el: &mut Node| {
        el.style.descriptor.background = Some(handle_color);
        let c = &mut el.style.descriptor.corner_radii;
        c.top_left = 999.0;
        c.top_right = 999.0;
        c.bottom_right = 999.0;
        c.bottom_left = 999.0;
        if !is_disabled {
            let patch = StylePatch {
                background: Some(hover_color),
                border_color: None,
                text_color: None,
                opacity: None,
            };
            el.style.hover = Some(patch);
            el.style.active = Some(patch);
            el.style.focus = Some(StylePatch {
                background: Some(focus_color),
                border_color: None,
                text_color: None,
                opacity: None,
            });
        }
    };

    // Drags do not bubble: the gesture starts only if the node under the
    // pointer carries the handler, and the pointer lands on the grab overlay
    // or the root's own hairline. Same lesson the sliders taught — every hit
    // target gets the handler.
    let drag_handler: Option<Arc<dyn Fn(&NodeDragEvent) + Send + Sync>> = if is_disabled {
        None
    } else if let Some(handler) = &on_resize {
        let handler = Arc::clone(handler);
        let horizontal = matches!(spec.orientation, Orientation::Horizontal);
        Some(Arc::new(move |event: &NodeDragEvent| match event.phase {
            NodeDragPhase::Start => handler(ResizePhase::Start, 0.0),
            NodeDragPhase::Move => {
                // A horizontal handle is a vertical line: it moves along x.
                let delta = if horizontal {
                    event.delta_x
                } else {
                    event.delta_y
                };
                handler(ResizePhase::Move, delta);
            }
            NodeDragPhase::End => handler(ResizePhase::End, 0.0),
        }))
    } else {
        None
    };

    let arm = |mut el: Node| -> Node {
        if let Some(handler) = &drag_handler {
            let handler = Arc::clone(handler);
            el.interaction.on_drag = Some(Arc::new(move |event| handler(event)));
        }
        el
    };

    let mut el = match spec.orientation {
        Orientation::Horizontal => {
            // Contract §7: horizontal orientation = vertical line.
            // Root: width 0.125rem (the line), height 100% (stretch to parent —
            // NOT flex-grow, which would fill the whole row). col-resize
            // cursor. Grab overlay: 0.5rem wide, centred, absolute.
            let mut root = Node::container();
            {
                let s = &mut root.style;
                s.descriptor.layout.width = LayoutSizing::Fixed(visual_size);
                s.self_stretch = true;
                s.flex_shrink_zero = true;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.descriptor.cursor = CursorHint::ColResize;
            }
            root.position = NodePosition::Relative;

            let mut overlay = Node::container();
            {
                let s = &mut overlay.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.width = LayoutSizing::Fixed(hit_size);
                s.fill_height = true;
            }
            overlay.position = NodePosition::Absolute {
                top: Some(0.0),
                left: Some(hit_offset),
                right: None,
                bottom: None,
            };

            root.child(arm(overlay))
        }
        Orientation::Vertical => {
            // Contract §7: vertical orientation = horizontal line.
            // Root: height 0.125rem (the line), width 100% (stretch).
            // row-resize. Grab overlay: 0.5rem tall, centred, absolute.
            let mut root = Node::container();
            {
                let s = &mut root.style;
                s.descriptor.layout.height = LayoutSizing::Fixed(visual_size);
                s.self_stretch = true;
                s.flex_shrink_zero = true;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.descriptor.cursor = CursorHint::RowResize;
            }
            root.position = NodePosition::Relative;

            let mut overlay = Node::container();
            {
                let s = &mut overlay.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.height = LayoutSizing::Fixed(hit_size);
                s.fill_width = true;
            }
            overlay.position = NodePosition::Absolute {
                top: Some(hit_offset),
                left: Some(0.0),
                right: None,
                bottom: None,
            };

            root.child(arm(overlay))
        }
    };

    paint(&mut el);

    if is_disabled {
        // Contract §8 disabled: default cursor + 0.4 opacity, no interaction,
        // and out of the focus order entirely.
        el.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        el.style.descriptor.cursor = CursorHint::Default;
        el.interaction.disabled = true;
    } else {
        // Contract §6: an enabled handle is a focus stop, and the focus style
        // above is also the backend's focus-observation channel.
        el.interaction.focusable = true;
        if let Some(handler) = &on_resize {
            let handler = Arc::clone(handler);
            let orientation = spec.orientation;
            el.interaction.on_key = Some(Arc::new(move |key: NodeKey, _mods: NodeModifiers| {
                let delta = key_step(orientation, key)?;
                // One keystroke is one whole gesture, so a host that opens on
                // Start and commits on End hears the same shape it hears from
                // the mouse.
                handler(ResizePhase::Start, 0.0);
                handler(ResizePhase::Move, delta);
                handler(ResizePhase::End, 0.0);
                None
            }));
        }
    }

    let mut el = arm(el);
    // `runtime_id`, not `id`: this is the backend's key for focus and gesture
    // state, and it must be unique per mounted instance. The semantic id may
    // repeat across instances by design (`poodle_node::Node`).
    el.runtime_id = Some(resize_handle_focus_id(spec));
    // Contract §6: the separator states its axis and its position within the
    // host's range. `aria_value_now` is the host's to update; the range is
    // declared either way, because a current value without one says nothing.
    el.a11y.role = Some(NodeRole::Splitter);
    el.a11y.label = Some(spec.effective_aria_label().to_string());
    el.a11y.orientation = Some(spec.aria_orientation().to_owned());
    el.a11y.value = spec.aria_value_now.map(|value| value as f64);
    el.a11y.value_min = Some(spec.aria_value_min as f64);
    el.a11y.value_max = Some(spec.aria_value_max as f64);
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    type Trace = Arc<Mutex<Vec<(ResizePhase, f32)>>>;

    fn armed(spec: &ResizeHandleSpec) -> (Node, Trace) {
        let trace: Trace = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&trace);
        let handler: Arc<dyn Fn(ResizePhase, f32) + Send + Sync> =
            Arc::new(move |phase, delta| sink.lock().expect("trace lock").push((phase, delta)));
        (resize_handle(spec, &theme(), Some(handler)), trace)
    }

    fn press(node: &Node, key: NodeKey) {
        let handler = node
            .interaction
            .on_key
            .as_ref()
            .expect("enabled handle routes keys");
        handler(key, NodeModifiers::default());
    }

    fn deltas(trace: &Trace) -> Vec<f32> {
        trace
            .lock()
            .expect("trace lock")
            .iter()
            .filter(|(phase, _)| *phase == ResizePhase::Move)
            .map(|(_, delta)| *delta)
            .collect()
    }

    /// Contract §6: an enabled separator is a focus stop, a disabled one is
    /// not reachable at all. Without this the keyboard route below is wired to
    /// a node no one can reach.
    #[test]
    fn only_an_enabled_handle_takes_focus() {
        let (enabled, _) = armed(&ResizeHandleSpec::new("split"));
        assert!(enabled.interaction.focusable);
        assert!(!enabled.interaction.disabled);
        assert!(
            enabled.style.focus.is_some(),
            "the focus state is both the visible treatment and the backend's focus channel"
        );

        let (disabled, _) = armed(&ResizeHandleSpec::new("split").with_disabled(true));
        assert!(!disabled.interaction.focusable);
        assert!(disabled.interaction.disabled);
        assert!(disabled.style.focus.is_none());
        assert!(disabled.interaction.on_key.is_none());
        assert!(disabled.interaction.on_drag.is_none());
    }

    /// A focus treatment that repaints nothing is not a focus treatment. The
    /// root carries the affordance paint, so focus is visible on the pixels
    /// the reader is looking at.
    #[test]
    fn focus_recolors_the_visible_hairline() {
        let (node, _) = armed(&ResizeHandleSpec::new("split"));
        let idle = node
            .style
            .descriptor
            .background
            .expect("the hairline paints at rest");
        let focus = node
            .style
            .focus
            .expect("focus patch")
            .background
            .expect("focus repaints the hairline");
        assert_ne!(idle, focus);
        assert_eq!(focus, theme().resolve_color("color.accent.focusRing"));
    }

    /// Contract §6 exactly: axis arrows step ±8, Home/End saturate, and a
    /// cross-axis arrow is not this handle's key.
    #[test]
    fn horizontal_answers_left_and_right_only() {
        let (node, trace) =
            armed(&ResizeHandleSpec::new("split").with_orientation(Orientation::Horizontal));
        press(&node, NodeKey::ArrowLeft);
        press(&node, NodeKey::ArrowRight);
        press(&node, NodeKey::Home);
        press(&node, NodeKey::End);
        assert_eq!(deltas(&trace), [-8.0, 8.0, -9999.0, 9999.0]);

        trace.lock().expect("trace lock").clear();
        press(&node, NodeKey::ArrowUp);
        press(&node, NodeKey::ArrowDown);
        press(&node, NodeKey::Space);
        assert!(
            trace.lock().expect("trace lock").is_empty(),
            "a cross-axis arrow keeps whatever meaning the surface gives it"
        );
    }

    #[test]
    fn vertical_answers_up_and_down_only() {
        let (node, trace) =
            armed(&ResizeHandleSpec::new("split").with_orientation(Orientation::Vertical));
        press(&node, NodeKey::ArrowUp);
        press(&node, NodeKey::ArrowDown);
        press(&node, NodeKey::Home);
        press(&node, NodeKey::End);
        assert_eq!(deltas(&trace), [-8.0, 8.0, -9999.0, 9999.0]);

        trace.lock().expect("trace lock").clear();
        press(&node, NodeKey::ArrowLeft);
        press(&node, NodeKey::ArrowRight);
        assert!(trace.lock().expect("trace lock").is_empty());
    }

    /// One keystroke reads as one complete gesture, so a host that commits on
    /// release commits once per key.
    #[test]
    fn a_keystroke_is_a_whole_gesture() {
        let (node, trace) = armed(&ResizeHandleSpec::new("split"));
        press(&node, NodeKey::ArrowRight);
        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            [
                (ResizePhase::Start, 0.0),
                (ResizePhase::Move, 8.0),
                (ResizePhase::End, 0.0)
            ]
        );
    }

    /// Contract §6: orientation, name and the whole numeric range survive the
    /// spec → node crossing. A current value with no range announces nothing.
    #[test]
    fn the_separator_declares_its_axis_name_and_range() {
        let spec = ResizeHandleSpec::new("split")
            .with_orientation(Orientation::Vertical)
            .with_aria_label("Resize vertical")
            .with_aria_value_now(80.0)
            .with_aria_value_min(40.0)
            .with_aria_value_max(120.0);
        let (node, _) = armed(&spec);
        assert_eq!(node.a11y.role, Some(NodeRole::Splitter));
        assert_eq!(node.a11y.label.as_deref(), Some("Resize vertical"));
        assert_eq!(node.a11y.orientation.as_deref(), Some("vertical"));
        assert_eq!(node.a11y.value, Some(80.0));
        assert_eq!(node.a11y.value_min, Some(40.0));
        assert_eq!(node.a11y.value_max, Some(120.0));
    }

    /// The contract's default name, and the default range, apply on a bare
    /// spec — an unnamed separator is the a11y defect this prevents.
    #[test]
    fn an_unnamed_handle_still_announces_itself() {
        let (node, _) = armed(&ResizeHandleSpec::new("split"));
        assert_eq!(node.a11y.label.as_deref(), Some("Resize"));
        assert_eq!(node.a11y.orientation.as_deref(), Some("horizontal"));
        assert_eq!(node.a11y.value, None);
        assert_eq!(node.a11y.value_min, Some(0.0));
        assert_eq!(node.a11y.value_max, Some(100.0));
    }

    /// Two handles that agree on every semantic — same axis, same name, same
    /// range — are still two handles. Only the caller's scope separates them,
    /// which is why nothing derived can.
    #[test]
    fn identical_handles_keep_distinct_backend_identities() {
        let build = |scope: &str| {
            armed(
                &ResizeHandleSpec::new(scope)
                    .with_orientation(Orientation::Horizontal)
                    .with_aria_label("Resize")
                    .with_aria_value_now(50.0),
            )
            .0
        };
        let left = build("editor:left");
        let right = build("editor:right");
        assert_eq!(
            left.runtime_id.as_deref(),
            Some("resize-handle:editor:left")
        );
        assert_eq!(
            right.runtime_id.as_deref(),
            Some("resize-handle:editor:right")
        );
        assert_ne!(left.runtime_id, right.runtime_id);
        assert_eq!(
            left.a11y.label, right.a11y.label,
            "the semantics are identical; only the scope is not"
        );
    }

    /// The key is the instance, not what the instance currently says. A
    /// relabelled, revalued, re-oriented handle is the same handle, and a
    /// backend that lost its focus handle on a translation would be wrong.
    #[test]
    fn one_instance_keeps_its_identity_across_rebuilds() {
        let first = armed(&ResizeHandleSpec::new("editor:left").with_aria_label("Resize"))
            .0
            .runtime_id;
        let relabelled = armed(
            &ResizeHandleSpec::new("editor:left")
                .with_orientation(Orientation::Vertical)
                .with_aria_label("Ancho del panel")
                .with_aria_value_now(200.0)
                .with_aria_value_max(400.0),
        )
        .0
        .runtime_id;
        assert_eq!(first, relabelled);
        assert_eq!(first.as_deref(), Some("resize-handle:editor:left"));
        assert_eq!(
            resize_handle_focus_id(&ResizeHandleSpec::new("editor:left")).as_str(),
            "resize-handle:editor:left",
            "hosts derive the same key from the scope alone",
        );
    }

    /// The disabled handle is identified too: a host that re-enables it must
    /// find the same node, not a new one.
    #[test]
    fn a_disabled_handle_is_still_identified() {
        let (node, _) = armed(&ResizeHandleSpec::new("editor:left").with_disabled(true));
        assert_eq!(
            node.runtime_id.as_deref(),
            Some("resize-handle:editor:left")
        );
    }

    /// Pointer drag is untouched by the keyboard work: the per-frame delta is
    /// still the axis delta, and both hit targets still start a gesture.
    #[test]
    fn drag_still_reports_the_per_frame_axis_delta() {
        for (orientation, delta_x, delta_y, expected) in [
            (Orientation::Horizontal, 12.0, 3.0, 12.0),
            (Orientation::Vertical, 3.0, 12.0, 12.0),
        ] {
            let (node, trace) =
                armed(&ResizeHandleSpec::new("split").with_orientation(orientation));
            let targets: Vec<&Node> = std::iter::once(&node).chain(node.children.iter()).collect();
            assert_eq!(targets.len(), 2, "the hairline and its grab overlay");
            for target in targets {
                let drag = target
                    .interaction
                    .on_drag
                    .as_ref()
                    .expect("every hit target starts a gesture — drags do not bubble");
                drag(&NodeDragEvent {
                    phase: NodeDragPhase::Start,
                    delta_x: 0.0,
                    delta_y: 0.0,
                });
                drag(&NodeDragEvent {
                    phase: NodeDragPhase::Move,
                    delta_x,
                    delta_y,
                });
                drag(&NodeDragEvent {
                    phase: NodeDragPhase::End,
                    delta_x: 0.0,
                    delta_y: 0.0,
                });
            }
            assert_eq!(deltas(&trace), [expected, expected]);
        }
    }
}
