//! SplitView — Jetstream split view backed by SplitViewSpec.
//!
//! Contract: `docs/contracts/components/split-view.md`
//! Reference: `packages/svelte/components/src/SplitView.svelte`
//!
//! Composes the real `js_resize_handle` divider and (when enabled) the real
//! `js_collapse_toggle` buttons. Orientation maps the split axis to the
//! handle's line axis (Horizontal split → Vertical line). Fixed/collapsed
//! pane sizing is applied on the correct axis per orientation. Min-size
//! constraints are applied inline when set and the pane is not collapsed.
//!
//! Interaction (drag-to-resize, keyboard resize, drag-to-collapse, rail
//! collapse) lives in the preview event loop — the handle and toggles render
//! the affordances but emit no callbacks here. Rail-collapse (`*CollapsedSize`,
//! `collapse*BelowSize`) is not in `SplitViewSpec`; legacy collapse hides the
//! pane (`0`-size), matching the contract's non-railed collapse state.
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    CollapseDirection, CollapseToggleSpec, Orientation, ResizeHandleSpec, SplitOrientation,
    SplitViewSpec,
};

use crate::collapse_toggle::js_collapse_toggle;
use crate::presentation::resolve_semantic_size;
use crate::resize_handle::js_resize_handle;

pub fn js_split_view(
    spec: &SplitViewSpec,
    theme: &JetstreamThemeProvider,
    primary: Option<JsEl>,
    secondary: Option<JsEl>,
) -> JsEl {
    let _effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let ratio = spec.current_ratio();
    let is_horizontal = spec.orientation == SplitOrientation::Horizontal;

    // Apply a fixed/collapsed pixel size on the correct axis for the
    // orientation: horizontal splits size panes by width, vertical by height.
    let axis_fixed = |el: JsEl, size: f32| -> JsEl {
        if is_horizontal {
            el.w(size)
        } else {
            el.h(size)
        }
    };
    // Apply a min-size constraint on the cross axis matching the split axis.
    let axis_min = |el: JsEl, min: f32| -> JsEl {
        if is_horizontal {
            el.min_w(min)
        } else {
            el.min_h(min)
        }
    };

    // ── Primary pane ──────────────────────────────────────────────────────────
    let primary_pane = {
        let mut pane = if spec.is_primary_collapsed {
            // Legacy collapse: hide the pane (flex 0 0 0) on the split axis.
            axis_fixed(ui_element::div().min_w(0.0).min_h(0.0), 0.0)
        } else if let Some(size) = spec.primary_size {
            // Fixed primary size overrides ratio.
            axis_fixed(ui_element::div().min_w(0.0).min_h(0.0), size)
        } else {
            let mut p = ui_element::div().min_w(0.0).min_h(0.0);
            // Secondary fixed → primary fills remaining space; otherwise ratio.
            p.layout.flex_grow = if spec.secondary_size.is_some() {
                1.0
            } else {
                ratio
            };
            p.layout.flex_shrink = 1.0;
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
            axis_fixed(ui_element::div().min_w(0.0).min_h(0.0), 0.0)
        } else if let Some(size) = spec.secondary_size {
            axis_fixed(ui_element::div().min_w(0.0).min_h(0.0), size)
        } else {
            let mut p = ui_element::div().min_w(0.0).min_h(0.0);
            p.layout.flex_grow = secondary_ratio;
            p.layout.flex_shrink = 1.0;
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
    // The resize handle's line axis is the inverse of the split axis:
    // Horizontal split (side-by-side panes) → vertical divider line;
    // Vertical split (stacked panes) → horizontal divider line.
    let handle_orientation = match spec.orientation {
        SplitOrientation::Horizontal => Orientation::Vertical,
        SplitOrientation::Vertical => Orientation::Horizontal,
    };
    let handle_spec = ResizeHandleSpec::new()
        .with_orientation(handle_orientation)
        .with_disabled(spec.is_disabled);
    let handle = js_resize_handle(&handle_spec, theme);

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

    let divider = if has_toggles {
        // Overlay the toggle cluster on the handle. Immediate-mode layout has
        // no absolute positioning here, so the cluster sits inline within the
        // divider container, stacked along the divider's long axis. Approximate
        // vs the Svelte absolute-centered overlay — noted in parity doc.
        let mut cluster = if is_horizontal {
            // Horizontal split: divider runs vertically → stack toggles in a column.
            ui_element::div().flex_col().items_center().justify_center()
        } else {
            // Vertical split: divider runs horizontally → stack toggles in a row.
            ui_element::div().flex_row().items_center().justify_center()
        };
        if show_primary_toggle {
            let toggle_spec = CollapseToggleSpec::new()
                .with_direction(primary_dir)
                .with_collapsed(spec.is_primary_collapsed)
                .with_disabled(spec.is_disabled);
            cluster = cluster.child(js_collapse_toggle(&toggle_spec, theme));
        }
        if show_secondary_toggle {
            let toggle_spec = CollapseToggleSpec::new()
                .with_direction(secondary_dir)
                .with_collapsed(spec.is_secondary_collapsed)
                .with_disabled(spec.is_disabled);
            cluster = cluster.child(js_collapse_toggle(&toggle_spec, theme));
        }

        let divider_container = if is_horizontal {
            ui_element::div().flex_col().items_center().justify_center()
        } else {
            ui_element::div().flex_row().items_center().justify_center()
        };
        divider_container.child(handle).child(cluster)
    } else {
        handle
    };

    // ── Root ──────────────────────────────────────────────────────────────────
    let mut el = match spec.orientation {
        SplitOrientation::Horizontal => ui_element::div().flex_row().grow(),
        SplitOrientation::Vertical => ui_element::div().flex_col().grow(),
    };

    el = el.child(primary_pane);
    el = el.child(divider);
    el = el.child(secondary_pane);

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::SplitOrientation;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn split_view_horizontal_two_panes_and_divider() {
        let th = theme();
        let spec = SplitViewSpec::new(SplitOrientation::Horizontal);
        let primary = ui_element::label("Primary");
        let secondary = ui_element::label("Secondary");
        let el = js_split_view(&spec, &th, Some(primary), Some(secondary));
        let tree = probe(&el, 600.0, 400.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(tree.has_text("Primary"), "primary pane missing: {:?}", tree.texts());
        assert!(tree.has_text("Secondary"), "secondary pane missing: {:?}", tree.texts());
        // Root is a row (panes side by side): primary sits left of secondary.
        let primary_node = tree.nodes.iter().find(|n| n.text.as_deref() == Some("Primary")).unwrap();
        let secondary_node = tree.nodes.iter().find(|n| n.text.as_deref() == Some("Secondary")).unwrap();
        assert!(
            primary_node.x < secondary_node.x,
            "horizontal split should place primary left of secondary: p.x={} s.x={}",
            primary_node.x, secondary_node.x
        );
        // Divider exists between the panes (handle is a panel with a line child).
        assert!(tree.count_kind("Panel") >= 3, "divider/panes missing panels");
    }

    #[test]
    fn split_view_vertical_stacks_panes() {
        let th = theme();
        let spec = SplitViewSpec::new(SplitOrientation::Vertical);
        let primary = ui_element::label("Top");
        let secondary = ui_element::label("Bottom");
        let el = js_split_view(&spec, &th, Some(primary), Some(secondary));
        let tree = probe(&el, 600.0, 400.0);

        let top = tree.nodes.iter().find(|n| n.text.as_deref() == Some("Top")).unwrap();
        let bottom = tree.nodes.iter().find(|n| n.text.as_deref() == Some("Bottom")).unwrap();
        // Vertical split stacks panes: top is above bottom.
        assert!(
            top.y < bottom.y,
            "vertical split should stack top above bottom: t.y={} b.y={}",
            top.y, bottom.y
        );
    }

    #[test]
    fn split_view_renders_collapse_toggles_when_enabled() {
        let th = theme();
        let spec = SplitViewSpec::new(SplitOrientation::Horizontal)
            .with_show_collapse_primary(true)
            .with_show_collapse_secondary(true);
        let el = js_split_view(&spec, &th, Some(ui_element::label("P")), Some(ui_element::label("S")));
        let tree = probe(&el, 600.0, 400.0);

        // CollapseToggle renders chevron icons. Horizontal → left + right.
        let icons: Vec<&str> = tree
            .nodes
            .iter()
            .filter(|n| n.kind == "Icon")
            .filter_map(|n| n.text.as_deref())
            .collect();
        assert!(
            icons.iter().any(|i| i.contains("chevron")),
            "collapse toggle chevrons missing: {:?}",
            icons
        );
        // Two toggles → at least two chevron icons.
        let chevron_count = icons.iter().filter(|i| i.contains("chevron")).count();
        assert!(chevron_count >= 2, "expected 2 collapse chevrons, got {}", chevron_count);
    }

    #[test]
    fn split_view_vertical_fixed_primary_uses_height_axis() {
        let th = theme();
        // Fixed primary of 120px in a vertical split should size the pane by
        // height (not width). The probe lets us assert the rendered rect.
        let spec = SplitViewSpec::new(SplitOrientation::Vertical).with_primary_size(120.0);
        let el = js_split_view(&spec, &th, Some(ui_element::label("Top")), Some(ui_element::label("Bottom")));
        let tree = probe(&el, 600.0, 400.0);

        // The primary pane is the node whose subtree contains "Top". Its height
        // should be the fixed 120px (axis-correct), not stretched to full width.
        let top = tree.nodes.iter().find(|n| n.text.as_deref() == Some("Top")).unwrap();
        // Walk up: the pane is "Top"'s parent panel — find a panel at top.x with h≈120.
        let pane_h_120 = tree
            .nodes
            .iter()
            .any(|n| n.kind == "Panel" && (n.h - 120.0).abs() < 1.0);
        assert!(
            pane_h_120,
            "vertical fixed-primary pane should be 120px tall; top node rect=({},{},{},{})",
            top.x, top.y, top.w, top.h
        );
    }
}
