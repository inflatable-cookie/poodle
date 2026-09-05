//! SplitView — two panes with a resize divider and collapse toggles.
//!
//! Contract: `docs/contracts/components/split-view.md`
//! Ported from: `packages/jetstream/components/src/split_view.rs`.
//!
//! Composes the real `resize_handle` divider and (when enabled) the real
//! `collapse_toggle` buttons. The handle takes the split orientation
//! unchanged — its own contract maps horizontal orientation to a vertical
//! line. The divider forwards its gesture as `on_resize(phase, axis_delta)`
//! rather than the contract's `onRatioChange(ratio)`: converting a pixel
//! delta into a ratio needs the rendered axis extent, which the host owns —
//! recorded as a Known Delta in the contract.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node,
    StylePatch,
};
use poodle_specs::{
    CollapseDirection, CollapseToggleSpec, Orientation, ResizeHandleSpec, SplitOrientation,
    SplitViewSpec,
};

use crate::collapse_toggle::collapse_toggle;
use crate::context::RenderContext;
use crate::resize_handle::{resize_handle, ResizePhase};

/// Handlers mirror the GPUI target's names. Collapse handlers fire with the
/// collapsed state the pane is moving **to**.
#[derive(Default)]
pub struct SplitViewHandlers {
    pub on_primary_collapse: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub on_secondary_collapse: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    /// The divider's drag gesture: `Start`/`End` bracket it, `Move` carries
    /// the axis delta in logical px (x for a horizontal split, y for a
    /// vertical one).
    pub on_resize: Option<Arc<dyn Fn(ResizePhase, f32) + Send + Sync>>,
}

pub fn split_view(
    spec: &SplitViewSpec,
    ctx: &RenderContext<'_>,
    primary: Option<Node>,
    secondary: Option<Node>,
    handlers: SplitViewHandlers,
) -> Node {
    let ratio = spec.current_ratio();
    let is_horizontal = spec.orientation == SplitOrientation::Horizontal;

    // A pane base: min sizes zeroed on both axes so panes can actually shrink.
    // Explicit Row (see switch.rs) — the old tier's bare divs.
    let pane_base = || {
        let mut p = Node::container();
        p.style.descriptor.layout.direction = LayoutDirection::Row;
        p.style.min_width = Some(0.0);
        p.style.min_height = Some(0.0);
        // Old tier: panes clip and fill the cross axis (`h_full` on a
        // horizontal split, `w_full` on a vertical one).
        p.style.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        p.style.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        if is_horizontal {
            p.style.fill_height = true;
        } else {
            p.style.fill_width = true;
        }
        p
    };
    // Apply a fixed/collapsed pixel size on the correct axis for the
    // orientation: horizontal splits size panes by width, vertical by height.
    let axis_fixed = |mut p: Node, size: f32| -> Node {
        if is_horizontal {
            p.style.descriptor.layout.width = LayoutSizing::Fixed(size);
        } else {
            p.style.descriptor.layout.height = LayoutSizing::Fixed(size);
        }
        p
    };
    // Apply a min-size constraint on the axis matching the split axis.
    let axis_min = |mut p: Node, min: f32| -> Node {
        if is_horizontal {
            p.style.min_width = Some(min);
        } else {
            p.style.min_height = Some(min);
        }
        p
    };

    // ── Primary pane ──────────────────────────────────────────────────────────
    let primary_pane = {
        let mut pane = if spec.is_primary_collapsed {
            // Legacy collapse: hide the pane (flex 0 0 0) on the split axis.
            axis_fixed(pane_base(), 0.0)
        } else if let Some(size) = spec.primary_size {
            // Fixed primary size overrides ratio.
            axis_fixed(pane_base(), size)
        } else {
            let mut p = pane_base();
            // Secondary fixed → primary fills remaining space; otherwise the
            // ratio seeds the basis and grow/shrink settle the divider's
            // thickness (old tier: `flex_basis(relative(ratio))`).
            if spec.secondary_size.is_some() {
                p.style.flex_grow = Some(1.0);
            } else {
                p.style.flex_grow = Some(1.0);
                p.style.flex_basis_pct = Some(ratio);
            }
            if let Some(min) = spec.min_primary_size {
                p = axis_min(p, min);
            }
            p
        };
        if let Some(content) = primary {
            pane = pane.child(content);
        }
        pane
    };

    // ── Secondary pane ────────────────────────────────────────────────────────
    let secondary_pane = {
        let secondary_ratio = 1.0 - ratio;
        let mut pane = if spec.is_secondary_collapsed {
            axis_fixed(pane_base(), 0.0)
        } else if let Some(size) = spec.secondary_size {
            axis_fixed(pane_base(), size)
        } else {
            let mut p = pane_base();
            p.style.flex_grow = Some(1.0);
            p.style.flex_basis_pct = Some(secondary_ratio);
            if let Some(min) = spec.min_secondary_size {
                p = axis_min(p, min);
            }
            p
        };
        if let Some(content) = secondary {
            pane = pane.child(content);
        }
        pane
    };

    // ── Divider — ResizeHandle + optional CollapseToggles ─────────────────────
    // The handle takes the SPLIT orientation unchanged — its own contract (§7)
    // does the inversion: horizontal orientation = left/right resize = a
    // vertical line.
    let handle_orientation = match spec.orientation {
        SplitOrientation::Horizontal => Orientation::Horizontal,
        SplitOrientation::Vertical => Orientation::Vertical,
    };
    // The divider's scope is the split's own, so two identical splits do not
    // hand the backend one focus handle for two dividers.
    let handle_spec = ResizeHandleSpec::new(spec.divider_instance_id())
        .with_orientation(handle_orientation)
        // SplitView's Svelte counterpart leaves the handle uncontrolled, so
        // ResizeHandle exposes its minimum as the current value (zero).
        .with_aria_value_now(0.0)
        .with_disabled(spec.is_disabled);
    let handle = resize_handle(
        &handle_spec,
        ctx,
        handlers.on_resize.as_ref().map(Arc::clone),
    );

    // Contract toggle visibility: primary toggle shows when secondary is not
    // collapsed; secondary toggle shows when primary is not collapsed.
    let show_primary_toggle = spec.show_collapse_primary && !spec.is_secondary_collapsed;
    let show_secondary_toggle = spec.show_collapse_secondary && !spec.is_primary_collapsed;
    let has_toggles = show_primary_toggle || show_secondary_toggle;

    // Toggle chevron direction by orientation (contract §8):
    // horizontal → primary left / secondary right;
    // vertical   → primary up   / secondary down.
    let (primary_dir, secondary_dir) = if is_horizontal {
        (CollapseDirection::Left, CollapseDirection::Right)
    } else {
        (CollapseDirection::Up, CollapseDirection::Down)
    };

    let centered = |direction: LayoutDirection| -> Node {
        let mut c = Node::container();
        let s = &mut c.style;
        s.descriptor.layout.direction = direction;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        c
    };

    let divider = if has_toggles {
        // Overlay the toggle cluster on the handle. Immediate-mode layout has
        // no absolute positioning here, so the cluster sits inline within the
        // divider container, stacked along the divider's long axis. Approximate
        // vs the Svelte absolute-centered overlay — noted in parity doc.
        let cluster_dir = if is_horizontal {
            // Horizontal split: divider runs vertically → stack in a column.
            LayoutDirection::Column
        } else {
            // Vertical split: divider runs horizontally → stack in a row.
            LayoutDirection::Row
        };
        let mut cluster = centered(cluster_dir);
        if show_primary_toggle {
            cluster = cluster.child(collapse_toggle(
                &CollapseToggleSpec::new()
                    .with_direction(primary_dir)
                    .with_collapsed(spec.is_primary_collapsed)
                    .with_aria_label(if spec.is_primary_collapsed {
                        "Expand primary"
                    } else {
                        "Collapse primary"
                    })
                    .with_disabled(spec.is_disabled),
                ctx,
                handlers.on_primary_collapse.as_ref().map(Arc::clone),
            ));
        }
        if show_secondary_toggle {
            cluster = cluster.child(collapse_toggle(
                &CollapseToggleSpec::new()
                    .with_direction(secondary_dir)
                    .with_collapsed(spec.is_secondary_collapsed)
                    .with_aria_label(if spec.is_secondary_collapsed {
                        "Expand secondary"
                    } else {
                        "Collapse secondary"
                    })
                    .with_disabled(spec.is_disabled),
                ctx,
                handlers.on_secondary_collapse.as_ref().map(Arc::clone),
            ));
        }

        // Hover-reveal: the cluster rests at opacity 0 and its own hover
        // brings it back. Opacity is paint-only in both backends, so the
        // cluster still hit-tests while invisible — the reveal region is the
        // pill's own bounds, which is the seam. A collapsed pane opts out
        // (`toggles_hidden_until_hover`): its expand toggle is the only way
        // back and must not need a hover to be found.
        if spec.toggles_hidden_until_hover() {
            cluster.style.descriptor.opacity = 0.0;
            cluster.style.hover = Some(StylePatch {
                background: None,
                border_color: None,
                text_color: None,
                opacity: Some(1.0),
            });
        }

        centered(cluster_dir).child(handle).child(cluster)
    } else {
        handle
    };

    // ── Root ──────────────────────────────────────────────────────────────────
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = if is_horizontal {
            LayoutDirection::Row
        } else {
            LayoutDirection::Column
        };
        // Old tier `.grow()`: flex props + stretch + min-size 0.
        s.descriptor.layout.width = LayoutSizing::Grow;
        // Old tier `.size_full()`. Without the height the split collapses to
        // its panes' content height instead of filling the host's frame, which
        // is the whole point of a split layout.
        s.fill_width = true;
        s.fill_height = true;
        // Old tier dims the whole split when disabled; the divider's own
        // disabled treatment is separate.
        if spec.is_disabled {
            s.descriptor.opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
        }
    }

    let mut el = el.child(primary_pane).child(divider).child(secondary_pane);

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::SplitToggleVisibility;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn toggling_spec() -> SplitViewSpec {
        SplitViewSpec::new("split-view-test", SplitOrientation::Horizontal)
            .with_show_collapse_primary(true)
            .with_show_collapse_secondary(true)
    }

    /// The toggle cluster is the divider's second child (the handle is first).
    fn cluster(node: &Node) -> &Node {
        let divider = &node.children[1];
        &divider.children[1]
    }

    fn render(spec: &SplitViewSpec) -> Node {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        split_view(spec, &ctx, None, None, SplitViewHandlers::default())
    }

    #[test]
    fn always_visibility_leaves_the_cluster_opaque_and_unpatched() {
        let node = render(&toggling_spec());
        let cluster = cluster(&node);
        assert_eq!(cluster.style.descriptor.opacity, 1.0);
        assert!(cluster.style.hover.is_none());
    }

    #[test]
    fn divider_and_toggle_expose_the_split_accessibility_contract() {
        let node = render(&toggling_spec());
        let divider = &node.children[1];
        let handle = &divider.children[0];
        assert_eq!(handle.a11y.value, Some(0.0));
        assert_eq!(handle.a11y.orientation.as_deref(), Some("horizontal"));

        let primary_toggle = &cluster(&node).children[0];
        assert_eq!(primary_toggle.a11y.label.as_deref(), Some("Collapse primary"));
    }

    #[test]
    fn hover_visibility_rests_the_cluster_at_zero_and_reveals_on_hover() {
        let spec = toggling_spec().with_toggle_visibility(SplitToggleVisibility::Hover);
        let node = render(&spec);
        let cluster = cluster(&node);
        assert_eq!(cluster.style.descriptor.opacity, 0.0);
        assert_eq!(cluster.style.hover.expect("hover patch").opacity, Some(1.0));
    }

    #[test]
    fn a_collapsed_pane_keeps_its_expand_toggle_visible_under_hover_visibility() {
        // The expand toggle is the only way back; hiding it behind a hover on
        // a seam that has been pushed to the container edge strands the pane.
        for spec in [
            toggling_spec()
                .with_toggle_visibility(SplitToggleVisibility::Hover)
                .with_primary_collapsed(true),
            toggling_spec()
                .with_toggle_visibility(SplitToggleVisibility::Hover)
                .with_secondary_collapsed(true),
        ] {
            let node = render(&spec);
            let cluster = cluster(&node);
            assert_eq!(cluster.style.descriptor.opacity, 1.0);
            assert!(cluster.style.hover.is_none());
        }
    }

    #[test]
    fn horizontal_and_vertical_root_and_pane_layout_postures() {
        let horizontal_spec = SplitViewSpec::new("split-h", SplitOrientation::Horizontal)
            .with_ratio(0.35)
            .with_aria_label("Horizontal split");
        let h_node = render(&horizontal_spec);
        assert_eq!(
            h_node.style.descriptor.layout.direction,
            LayoutDirection::Row
        );
        assert!(h_node.style.fill_width);
        assert!(h_node.style.fill_height);
        assert_eq!(
            h_node.style.descriptor.layout.width,
            LayoutSizing::Grow
        );
        assert_eq!(
            h_node.a11y.label.as_deref(),
            Some("Horizontal split")
        );

        let p_pane = &h_node.children[0];
        assert_eq!(p_pane.style.descriptor.layout.direction, LayoutDirection::Row);
        assert!(p_pane.style.fill_height);
        assert!(!p_pane.style.fill_width);
        assert_eq!(p_pane.style.min_width, Some(0.0));
        assert_eq!(p_pane.style.min_height, Some(0.0));
        assert_eq!(p_pane.style.descriptor.layout.overflow_x, LayoutOverflow::Hidden);
        assert_eq!(p_pane.style.descriptor.layout.overflow_y, LayoutOverflow::Hidden);
        assert_eq!(p_pane.style.flex_grow, Some(1.0));
        assert_eq!(p_pane.style.flex_basis_pct, Some(0.35));

        let s_pane = &h_node.children[2];
        assert_eq!(s_pane.style.descriptor.layout.direction, LayoutDirection::Row);
        assert!(s_pane.style.fill_height);
        assert_eq!(s_pane.style.flex_grow, Some(1.0));
        assert!((s_pane.style.flex_basis_pct.unwrap() - 0.65).abs() < 1e-5);

        let vertical_spec = SplitViewSpec::new("split-v", SplitOrientation::Vertical)
            .with_ratio(0.4);
        let v_node = render(&vertical_spec);
        assert_eq!(
            v_node.style.descriptor.layout.direction,
            LayoutDirection::Column
        );
        assert!(v_node.style.fill_width);
        assert!(v_node.style.fill_height);
        assert_eq!(v_node.children[0].style.fill_width, true);
        assert_eq!(v_node.children[0].style.flex_basis_pct, Some(0.4));
        assert!((v_node.children[2].style.flex_basis_pct.unwrap() - 0.6).abs() < 1e-5);
    }

    #[test]
    fn pane_min_sizes_apply_on_matching_axis() {
        let h_spec = SplitViewSpec::new("split-min-h", SplitOrientation::Horizontal)
            .with_min_primary_size(120.0)
            .with_min_secondary_size(180.0);
        let h_node = render(&h_spec);
        assert_eq!(h_node.children[0].style.min_width, Some(120.0));
        assert_eq!(h_node.children[2].style.min_width, Some(180.0));

        let v_spec = SplitViewSpec::new("split-min-v", SplitOrientation::Vertical)
            .with_min_primary_size(100.0)
            .with_min_secondary_size(160.0);
        let v_node = render(&v_spec);
        assert_eq!(v_node.children[0].style.min_height, Some(100.0));
        assert_eq!(v_node.children[2].style.min_height, Some(160.0));
    }

    #[test]
    fn fixed_pane_sizes_and_collapses() {
        let fixed_p = SplitViewSpec::new("fixed-p", SplitOrientation::Horizontal)
            .with_primary_size(240.0);
        let node_p = render(&fixed_p);
        assert_eq!(
            node_p.children[0].style.descriptor.layout.width,
            LayoutSizing::Fixed(240.0)
        );
        assert_eq!(node_p.children[2].style.flex_grow, Some(1.0));

        let fixed_s = SplitViewSpec::new("fixed-s", SplitOrientation::Vertical)
            .with_secondary_size(300.0);
        let node_s = render(&fixed_s);
        assert_eq!(
            node_s.children[2].style.descriptor.layout.height,
            LayoutSizing::Fixed(300.0)
        );
        assert_eq!(node_s.children[0].style.flex_grow, Some(1.0));

        let collapsed_p = SplitViewSpec::new("collapsed-p", SplitOrientation::Horizontal)
            .with_primary_collapsed(true);
        let node_cp = render(&collapsed_p);
        assert_eq!(
            node_cp.children[0].style.descriptor.layout.width,
            LayoutSizing::Fixed(0.0)
        );

        let collapsed_s = SplitViewSpec::new("collapsed-s", SplitOrientation::Vertical)
            .with_secondary_collapsed(true);
        let node_cs = render(&collapsed_s);
        assert_eq!(
            node_cs.children[2].style.descriptor.layout.height,
            LayoutSizing::Fixed(0.0)
        );
    }

    #[test]
    fn disabled_split_dims_root_and_passes_disabled_to_handle_and_toggles() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let disabled_spec = toggling_spec().with_disabled(true);
        let node = split_view(&disabled_spec, &ctx, None, None, SplitViewHandlers::default());
        assert_eq!(
            node.style.descriptor.opacity,
            ctx.theme().resolve_opacity("state.opacity.disabled")
        );
        let divider = &node.children[1];
        let handle = &divider.children[0];
        assert!(handle.interaction.disabled);
        let cluster = &divider.children[1];
        let primary_toggle = &cluster.children[0];
        assert!(primary_toggle.interaction.disabled);
    }

    #[test]
    fn toggle_chevron_directions_by_orientation() {
        use poodle_node::NodeKind;

        let h_node = render(&toggling_spec());
        let h_cluster = cluster(&h_node);
        let h_primary_icon = &h_cluster.children[0].children[0];
        let h_secondary_icon = &h_cluster.children[1].children[0];
        match &h_primary_icon.kind {
            NodeKind::Icon { name, size } => {
                assert_eq!(name, "chevron-left");
                assert_eq!(*size, 12.0);
            }
            _ => panic!("expected icon kind"),
        }
        match &h_secondary_icon.kind {
            NodeKind::Icon { name, size } => {
                assert_eq!(name, "chevron-right");
                assert_eq!(*size, 12.0);
            }
            _ => panic!("expected icon kind"),
        }

        let v_spec = SplitViewSpec::new("split-v", SplitOrientation::Vertical)
            .with_show_collapse_primary(true)
            .with_show_collapse_secondary(true);
        let v_node = render(&v_spec);
        let v_cluster = cluster(&v_node);
        let v_primary_icon = &v_cluster.children[0].children[0];
        let v_secondary_icon = &v_cluster.children[1].children[0];
        match &v_primary_icon.kind {
            NodeKind::Icon { name, size } => {
                assert_eq!(name, "chevron-up");
                assert_eq!(*size, 12.0);
            }
            _ => panic!("expected icon kind"),
        }
        match &v_secondary_icon.kind {
            NodeKind::Icon { name, size } => {
                assert_eq!(name, "chevron-down");
                assert_eq!(*size, 12.0);
            }
            _ => panic!("expected icon kind"),
        }
    }

    #[test]
    fn handlers_are_forwarded_to_handle_and_toggles() {
        use std::sync::Mutex;
        let resize_events = Arc::new(Mutex::new(Vec::<(ResizePhase, f32)>::new()));
        let p_collapse = Arc::new(Mutex::new(Vec::<bool>::new()));
        let s_collapse = Arc::new(Mutex::new(Vec::<bool>::new()));

        let resize_sink = Arc::clone(&resize_events);
        let p_sink = Arc::clone(&p_collapse);
        let s_sink = Arc::clone(&s_collapse);

        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = split_view(
            &toggling_spec(),
            &ctx,
            None,
            None,
            SplitViewHandlers {
                on_resize: Some(Arc::new(move |phase, delta| {
                    resize_sink.lock().unwrap().push((phase, delta));
                })),
                on_primary_collapse: Some(Arc::new(move |next| {
                    p_sink.lock().unwrap().push(next);
                })),
                on_secondary_collapse: Some(Arc::new(move |next| {
                    s_sink.lock().unwrap().push(next);
                })),
            },
        );

        let divider = &node.children[1];
        let handle = &divider.children[0];
        let key_handler = handle.interaction.on_key.as_ref().expect("on_key");
        key_handler(poodle_node::NodeKey::ArrowRight, poodle_node::NodeModifiers::default());
        assert_eq!(
            *resize_events.lock().unwrap(),
            [
                (ResizePhase::Start, 0.0),
                (ResizePhase::Move, 8.0),
                (ResizePhase::End, 0.0)
            ]
        );

        let cluster = &divider.children[1];
        let p_toggle = &cluster.children[0];
        p_toggle.interaction.on_activate.as_ref().expect("p_toggle on_activate")();
        assert_eq!(*p_collapse.lock().unwrap(), [true]);

        let s_toggle = &cluster.children[1];
        s_toggle.interaction.on_activate.as_ref().expect("s_toggle on_activate")();
        assert_eq!(*s_collapse.lock().unwrap(), [true]);
    }
}
