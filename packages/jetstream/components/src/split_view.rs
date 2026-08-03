//! SplitView — Jetstream split view backed by SplitViewSpec.
//!
//! Contract: `docs/contracts/components/split-view.md`
//! Reference: `packages/svelte/components/src/SplitView.svelte`
//!
//! Composes the real `ResizeHandle` divider and (when enabled) the real
//! `js_collapse_toggle` buttons. The handle takes the split orientation
//! unchanged — its own contract maps horizontal orientation to a vertical
//! line. Fixed/collapsed pane sizing is applied on the correct axis per
//! orientation. Min-size constraints are applied inline when set and the
//! pane is not collapsed.
//!
//! The collapse toggles forward `on_primary_collapse`/`on_secondary_collapse`
//! and the divider forwards its drag as `on_resize(phase, axis_delta)`.
//! Keyboard resize stays host-side (the runtime raises no key events).
//! Rail-collapse (`*CollapsedSize`, `collapse*BelowSize`) is not in
//! `SplitViewSpec`; legacy collapse hides the pane (`0`-size), matching the
//! contract's non-railed collapse state.
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    CollapseDirection, CollapseToggleSpec, Orientation, ResizeHandleSpec, SplitOrientation,
    SplitViewSpec,
};

use crate::collapse_toggle::js_collapse_toggle;
use crate::presentation::resolve_semantic_size;

/// SplitView — two panes with collapse toggles.
///
/// Mirrors the GPUI target's names: `on_primary_collapse` and
/// `on_secondary_collapse`, forwarded to the composed `CollapseToggle`s.
///
/// The divider forwards the composed `ResizeHandle`'s gesture as
/// `on_resize(phase, axis_delta)` rather than the contract's
/// `onRatioChange(ratio)`: converting a pixel delta into a ratio needs the
/// rendered axis extent, which the host owns and this immediate-mode build
/// never sees. The host applies `delta / extent` to the ratio it holds —
/// recorded as a Known Delta in the contract.
pub struct SplitView {
    spec: SplitViewSpec,
    theme: JetstreamThemeProvider,
    primary: Option<JsEl>,
    secondary: Option<JsEl>,
    on_primary_collapse: Option<crate::element::ToggleHandler>,
    on_secondary_collapse: Option<crate::element::ToggleHandler>,
    on_resize: Option<std::sync::Arc<dyn Fn(crate::resize_handle::ResizePhase, f32) + Send + Sync>>,
}

impl SplitView {
    pub fn from_spec(spec: SplitViewSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            primary: None,
            secondary: None,
            on_primary_collapse: None,
            on_secondary_collapse: None,
            on_resize: None,
        }
    }

    pub fn primary(mut self, primary: impl crate::element::IntoJsEl) -> Self {
        self.primary = Some(primary.into_js_el());
        self
    }

    pub fn secondary(mut self, secondary: impl crate::element::IntoJsEl) -> Self {
        self.secondary = Some(secondary.into_js_el());
        self
    }

    /// Fires with the collapsed state the primary pane is moving **to**.
    pub fn on_primary_collapse(mut self, handler: impl Fn(bool) + Send + Sync + 'static) -> Self {
        self.on_primary_collapse = Some(std::sync::Arc::new(handler));
        self
    }

    /// Fires with the collapsed state the secondary pane is moving **to**.
    pub fn on_secondary_collapse(mut self, handler: impl Fn(bool) + Send + Sync + 'static) -> Self {
        self.on_secondary_collapse = Some(std::sync::Arc::new(handler));
        self
    }

    /// The divider's drag gesture, forwarded from the composed `ResizeHandle`:
    /// `Start`/`End` bracket the gesture, `Move` carries the axis delta in
    /// logical px (x for a horizontal split, y for a vertical one). The host
    /// turns the delta into a ratio with the axis extent it laid the split
    /// out at.
    pub fn on_resize(
        mut self,
        handler: impl Fn(crate::resize_handle::ResizePhase, f32) + Send + Sync + 'static,
    ) -> Self {
        self.on_resize = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for SplitView {
    fn into_js_el(self) -> JsEl {
        build(
            &self.spec,
            &self.theme,
            self.primary,
            self.secondary,
            self.on_primary_collapse,
            self.on_secondary_collapse,
            self.on_resize,
        )
    }
}

pub fn js_split_view(
    spec: &SplitViewSpec,
    theme: &JetstreamThemeProvider,
    primary: Option<JsEl>,
    secondary: Option<JsEl>,
) -> JsEl {
    build(spec, theme, primary, secondary, None, None, None)
}

fn build(
    spec: &SplitViewSpec,
    theme: &JetstreamThemeProvider,
    primary: Option<JsEl>,
    secondary: Option<JsEl>,
    on_primary_collapse: Option<crate::element::ToggleHandler>,
    on_secondary_collapse: Option<crate::element::ToggleHandler>,
    on_resize: Option<std::sync::Arc<dyn Fn(crate::resize_handle::ResizePhase, f32) + Send + Sync>>,
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
    // The handle takes the SPLIT orientation unchanged — its own contract (§7)
    // does the inversion: horizontal orientation = left/right resize = a
    // vertical line. Svelte and GPUI both pass it straight through. This used
    // to map to the opposite orientation, which drew the divider as a
    // zero-width horizontal hairline inside a horizontal split.
    let handle_orientation = match spec.orientation {
        SplitOrientation::Horizontal => Orientation::Horizontal,
        SplitOrientation::Vertical => Orientation::Vertical,
    };
    let handle_spec = ResizeHandleSpec::new()
        .with_orientation(handle_orientation)
        .with_disabled(spec.is_disabled);
    let handle = {
        let mut handle = crate::resize_handle::ResizeHandle::from_spec(handle_spec, theme);
        if let Some(handler) = &on_resize {
            let handler = std::sync::Arc::clone(handler);
            handle = handle.on_resize(move |phase, delta| handler(phase, delta));
        }
        crate::element::IntoJsEl::into_js_el(handle)
    };

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
            let mut toggle = crate::collapse_toggle::CollapseToggle::from_spec(toggle_spec, theme);
            if let Some(handler) = &on_primary_collapse {
                let handler = std::sync::Arc::clone(handler);
                toggle = toggle.on_toggle(move |next| handler(next));
            }
            cluster = cluster.child(crate::element::IntoJsEl::into_js_el(toggle));
        }
        if show_secondary_toggle {
            let toggle_spec = CollapseToggleSpec::new()
                .with_direction(secondary_dir)
                .with_collapsed(spec.is_secondary_collapsed)
                .with_disabled(spec.is_disabled);
            let mut toggle = crate::collapse_toggle::CollapseToggle::from_spec(toggle_spec, theme);
            if let Some(handler) = &on_secondary_collapse {
                let handler = std::sync::Arc::clone(handler);
                toggle = toggle.on_toggle(move |next| handler(next));
            }
            cluster = cluster.child(crate::element::IntoJsEl::into_js_el(toggle));
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

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::SplitOrientation;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
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
        assert!(
            tree.has_text("Primary"),
            "primary pane missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Secondary"),
            "secondary pane missing: {:?}",
            tree.texts()
        );
        // Root is a row (panes side by side): primary sits left of secondary.
        let primary_node = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Primary"))
            .unwrap();
        let secondary_node = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Secondary"))
            .unwrap();
        assert!(
            primary_node.x < secondary_node.x,
            "horizontal split should place primary left of secondary: p.x={} s.x={}",
            primary_node.x,
            secondary_node.x
        );
        // Divider exists between the panes (handle is a panel with a line child).
        assert!(
            tree.count_kind("Panel") >= 3,
            "divider/panes missing panels"
        );
    }

    #[test]
    fn split_view_vertical_stacks_panes() {
        let th = theme();
        let spec = SplitViewSpec::new(SplitOrientation::Vertical);
        let primary = ui_element::label("Top");
        let secondary = ui_element::label("Bottom");
        let el = js_split_view(&spec, &th, Some(primary), Some(secondary));
        let tree = probe(&el, 600.0, 400.0);

        let top = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Top"))
            .unwrap();
        let bottom = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Bottom"))
            .unwrap();
        // Vertical split stacks panes: top is above bottom.
        assert!(
            top.y < bottom.y,
            "vertical split should stack top above bottom: t.y={} b.y={}",
            top.y,
            bottom.y
        );
    }

    #[test]
    fn split_view_renders_collapse_toggles_when_enabled() {
        let th = theme();
        let spec = SplitViewSpec::new(SplitOrientation::Horizontal)
            .with_show_collapse_primary(true)
            .with_show_collapse_secondary(true);
        let el = js_split_view(
            &spec,
            &th,
            Some(ui_element::label("P")),
            Some(ui_element::label("S")),
        );
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
        assert!(
            chevron_count >= 2,
            "expected 2 collapse chevrons, got {}",
            chevron_count
        );
    }

    #[test]
    fn split_view_vertical_fixed_primary_uses_height_axis() {
        let th = theme();
        // Fixed primary of 120px in a vertical split should size the pane by
        // height (not width). The probe lets us assert the rendered rect.
        let spec = SplitViewSpec::new(SplitOrientation::Vertical).with_primary_size(120.0);
        let el = js_split_view(
            &spec,
            &th,
            Some(ui_element::label("Top")),
            Some(ui_element::label("Bottom")),
        );
        let tree = probe(&el, 600.0, 400.0);

        // The primary pane is the node whose subtree contains "Top". Its height
        // should be the fixed 120px (axis-correct), not stretched to full width.
        let top = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Top"))
            .unwrap();
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

    #[test]
    fn the_collapse_toggles_report_their_pane() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let primaries = Arc::clone(&seen);
        let secondaries = Arc::clone(&seen);

        let spec = SplitViewSpec::new(SplitOrientation::Horizontal)
            .with_show_collapse_primary(true)
            .with_show_collapse_secondary(true);

        let el = SplitView::from_spec(spec, &theme())
            .primary(ui_element::label("Primary pane"))
            .secondary(ui_element::label("Secondary pane"))
            .on_primary_collapse(move |next| {
                primaries.lock().unwrap().push(format!("primary:{next}"))
            })
            .on_secondary_collapse(move |next| {
                secondaries
                    .lock()
                    .unwrap()
                    .push(format!("secondary:{next}"))
            })
            .into_js_el();

        // Two chevron toggles in the divider cluster, in pane order.
        let tree = crate::render_probe::probe(&el, 640.0, 400.0);
        let toggles: Vec<_> = tree
            .nodes
            .iter()
            .filter(|n| {
                n.text
                    .as_deref()
                    .map(|t| t.starts_with("chevron"))
                    .unwrap_or(false)
            })
            .map(|n| (n.x + n.w / 2.0, n.y + n.h / 2.0))
            .collect();
        assert_eq!(toggles.len(), 2, "two collapse toggles: {:?}", tree.texts());
        crate::element::click_probe::click_at(&el, 640.0, 400.0, toggles[0].0, toggles[0].1);
        crate::element::click_probe::click_at(&el, 640.0, 400.0, toggles[1].0, toggles[1].1);

        assert_eq!(
            seen.lock().unwrap().as_slice(),
            ["primary:true", "secondary:true"]
        );
    }

    /// Dragging the divider forwards the handle's gesture — Start/End
    /// bracketing Move deltas that sum to the distance travelled.
    #[test]
    fn dragging_the_divider_reports_the_gesture() {
        use crate::element::IntoJsEl;
        use crate::resize_handle::ResizePhase;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<(ResizePhase, f32)>>> = Arc::new(Mutex::new(Vec::new()));
        let events = Arc::clone(&seen);

        let el = SplitView::from_spec(SplitViewSpec::new(SplitOrientation::Horizontal), &theme())
            .primary(ui_element::label("Primary pane"))
            .secondary(ui_element::label("Secondary pane"))
            .on_resize(move |phase, delta| events.lock().unwrap().push((phase, delta)))
            .into_js_el();

        // The divider sits between the panes at the ratio point — find the
        // narrow panel between the pane rects and drag from its center.
        let tree = crate::render_probe::probe(&el, 640.0, 400.0);
        let divider = tree
            .nodes
            .iter()
            .filter(|n| n.kind == "Panel" && n.w < 20.0)
            .min_by(|a, b| a.w.partial_cmp(&b.w).unwrap())
            .expect("no divider panel found");
        let (cx_, cy_) = (divider.x + divider.w / 2.0, divider.y + divider.h / 2.0);
        crate::element::click_probe::drag(&el, 640.0, 400.0, (cx_, cy_), (cx_ + 48.0, cy_));

        let events = seen.lock().unwrap();
        assert!(!events.is_empty(), "the divider drag reported nothing");
        assert_eq!(events.first().map(|e| e.0), Some(ResizePhase::Start));
        assert_eq!(events.last().map(|e| e.0), Some(ResizePhase::End));
        let moved: f32 = events
            .iter()
            .filter(|(p, _)| *p == ResizePhase::Move)
            .map(|(_, d)| d)
            .sum();
        assert!(
            moved > 30.0,
            "the deltas did not sum to the distance: {moved}"
        );
    }
}
