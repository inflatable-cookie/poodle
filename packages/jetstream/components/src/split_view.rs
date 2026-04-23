//! SplitView — Jetstream split view backed by SplitViewSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{Orientation, ResizeHandleSpec, SplitOrientation, SplitViewSpec};

use crate::presentation::resolve_semantic_size;
use crate::resize_handle::js_resize_handle;

pub fn js_split_view(spec: &SplitViewSpec, theme: &JetstreamThemeProvider, primary: Option<JsEl>, secondary: Option<JsEl>) -> JsEl {
    let _effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let ratio = spec.current_ratio();

    // Build panes: each pane wraps its content in a flex container sized by ratio.
    // Taffy distributes space proportionally when flex_grow values differ — set them
    // directly on the layout field since the fluent API only exposes flex_grow = 1.0.
    // min_w/min_h = 0.0 so panes can shrink below their natural size.
    let primary_pane = {
        let mut pane = if let Some(px) = spec.primary_size {
            // Fixed primary size overrides ratio
            ui_element::div().w(px).min_w(0.0).min_h(0.0)
        } else if spec.is_primary_collapsed {
            ui_element::div().w(0.0).min_w(0.0).min_h(0.0)
        } else {
            let mut p = ui_element::div().min_w(0.0).min_h(0.0);
            p.layout.flex_grow = ratio;
            p.layout.flex_shrink = 1.0;
            p
        };
        if let Some(content) = primary {
            pane = pane.child(content);
        }
        pane
    };

    let secondary_pane = {
        let secondary_ratio = 1.0 - ratio;
        let mut pane = if let Some(px) = spec.secondary_size {
            // Fixed secondary size overrides ratio
            ui_element::div().w(px).min_w(0.0).min_h(0.0)
        } else if spec.is_secondary_collapsed {
            ui_element::div().w(0.0).min_w(0.0).min_h(0.0)
        } else {
            let mut p = ui_element::div().min_w(0.0).min_h(0.0);
            p.layout.flex_grow = secondary_ratio;
            p.layout.flex_shrink = 1.0;
            p
        };
        if let Some(content) = secondary {
            pane = pane.child(content);
        }
        pane
    };

    // The resize handle orientation is the axis of the divider line:
    // Horizontal split (side-by-side panes) → vertical divider line
    // Vertical split (stacked panes) → horizontal divider line
    let handle_orientation = match spec.orientation {
        SplitOrientation::Horizontal => Orientation::Vertical,
        SplitOrientation::Vertical => Orientation::Horizontal,
    };

    let handle_spec = ResizeHandleSpec::new()
        .with_orientation(handle_orientation)
        .with_disabled(spec.is_disabled);

    let handle = js_resize_handle(&handle_spec, theme);

    let mut el = match spec.orientation {
        SplitOrientation::Horizontal => ui_element::div().flex_row().grow(),
        SplitOrientation::Vertical => ui_element::div().flex_col().grow(),
    };

    el = el.child(primary_pane);
    el = el.child(handle);
    el = el.child(secondary_pane);

    el
}
