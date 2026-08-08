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

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node,
    StylePatch,
};
use poodle_specs::{
    CollapseDirection, CollapseToggleSpec, Orientation, ResizeHandleSpec, SplitOrientation,
    SplitViewSpec,
};

use crate::collapse_toggle::collapse_toggle;
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
    theme: &dyn ThemeProvider,
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
    let handle_spec = ResizeHandleSpec::new()
        .with_orientation(handle_orientation)
        .with_disabled(spec.is_disabled);
    let handle = resize_handle(
        &handle_spec,
        theme,
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
                    .with_disabled(spec.is_disabled),
                theme,
                handlers.on_primary_collapse.as_ref().map(Arc::clone),
            ));
        }
        if show_secondary_toggle {
            cluster = cluster.child(collapse_toggle(
                &CollapseToggleSpec::new()
                    .with_direction(secondary_dir)
                    .with_collapsed(spec.is_secondary_collapsed)
                    .with_disabled(spec.is_disabled),
                theme,
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
            s.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
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
        SplitViewSpec::new(SplitOrientation::Horizontal)
            .with_show_collapse_primary(true)
            .with_show_collapse_secondary(true)
    }

    /// The toggle cluster is the divider's second child (the handle is first).
    fn cluster(node: &Node) -> &Node {
        let divider = &node.children[1];
        &divider.children[1]
    }

    fn render(spec: &SplitViewSpec) -> Node {
        split_view(spec, &theme(), None, None, SplitViewHandlers::default())
    }

    #[test]
    fn always_visibility_leaves_the_cluster_opaque_and_unpatched() {
        let node = render(&toggling_spec());
        let cluster = cluster(&node);
        assert_eq!(cluster.style.descriptor.opacity, 1.0);
        assert!(cluster.style.hover.is_none());
    }

    #[test]
    fn hover_visibility_rests_the_cluster_at_zero_and_reveals_on_hover() {
        let spec = toggling_spec().with_toggle_visibility(SplitToggleVisibility::Hover);
        let node = render(&spec);
        let cluster = cluster(&node);
        assert_eq!(cluster.style.descriptor.opacity, 0.0);
        assert_eq!(
            cluster.style.hover.expect("hover patch").opacity,
            Some(1.0)
        );
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
}
