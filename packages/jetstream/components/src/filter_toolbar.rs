//! FilterToolbar — Jetstream filter toolbar backed by FilterToolbarSpec.
//!
//! Layout container matching the Svelte/GPUI FilterToolbar composite:
//!   - Optional header row with summary text + actions slot
//!   - Grid of filter control children (flex-wrap)
//!   - Optional secondary slot below the grid
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::FilterToolbarSpec;

use crate::presentation::resolve_semantic_size;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// Render a filter toolbar.
///
/// - `children`: filter controls laid out in a responsive grid
/// - `actions`: optional element rendered in the header row
/// - `secondary`: optional element rendered below the grid
pub fn js_filter_toolbar(
    spec: &FilterToolbarSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
    actions: Option<JsEl>,
    secondary: Option<JsEl>,
) -> JsEl {
    let _effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let bg = resolve_color(theme, spec.background_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let stack_sm = resolve_px(theme, spec.gap_token());
    let inline_md = resolve_px(theme, spec.controls_gap_token());
    let summary_color = resolve_color(theme, spec.summary_color_token());

    let is_expanded = spec.is_grid_visible();
    let had_children = !children.is_empty();

    let mut toolbar = ui_element::div()
        .flex_col()
        .gap(stack_sm)
        .p(12.0)
        .bg(bg)
        .border(1.0)
        .border_color(border)
        .rounded(radius);

    // ── Header row ──
    let needs_header = spec.collapsible || spec.summary_text.is_some() || actions.is_some();
    if needs_header {
        let mut header = ui_element::div().flex_row().items_center().gap(inline_md);

        if let Some(ref summary) = spec.summary_text {
            header = header.child(
                ui_element::label(summary)
                    .text_color(summary_color)
                    .text_size(13.0),
            );
        }

        if let Some(actions_el) = actions {
            header = header.child(actions_el);
        }

        toolbar = toolbar.child(header);
    }

    // ── Filter controls grid ──
    if is_expanded && had_children {
        let mut grid = ui_element::div().flex_row().gap(inline_md);
        for child in children {
            grid = grid.child(child);
        }
        toolbar = toolbar.child(grid);
    }

    // ── Secondary slot ──
    if let Some(secondary_el) = secondary {
        let _ = had_children;
        toolbar = toolbar.child(secondary_el);
    }

    toolbar
}
