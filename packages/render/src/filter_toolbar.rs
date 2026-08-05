//! FilterToolbar — collapsible filter header + responsive controls grid.
//!
//! Contract: `docs/contracts/components/filter-toolbar.md`
//! Ported from: `packages/jetstream/components/src/filter_toolbar.rs`.
//!
//! - `children`: filter controls laid out in a responsive grid
//! - `actions`: optional element rendered in the header row
//! - `secondary`: optional element rendered below the grid

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole,
};
use poodle_specs::FilterToolbarSpec;

use crate::presentation::rem_to_px;

/// `on_toggle` fires with the expanded state the toolbar is moving **to**.
pub fn filter_toolbar(
    spec: &FilterToolbarSpec,
    theme: &dyn ThemeProvider,
    children: Vec<Node>,
    actions: Option<Node>,
    secondary: Option<Node>,
    on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
) -> Node {
    // Contract §8 summary size table (size-scaled label-size).
    let font_size = rem_to_px(spec.summary_font_size_rem());

    // Contract §8 density table: distinct root padding-block / padding-inline,
    // root gap, and controls-grid gap per density.
    let pad_block = rem_to_px(spec.padding_block_rem());
    let pad_inline = rem_to_px(spec.padding_inline_rem());
    let root_gap = match spec.density_gap_rem() {
        Some(rem) => rem_to_px(rem),
        None => theme.resolve_space(spec.gap_token()),
    };
    let header_gap = theme.resolve_space("space.inline.sm");
    let controls_gap = match spec.density_controls_gap_rem() {
        Some(rem) => rem_to_px(rem),
        None => theme.resolve_space(spec.controls_gap_token()),
    };
    let actions_gap = theme.resolve_space(spec.actions_gap_token());

    let bg = theme.resolve_color(spec.background_token());
    let border = theme.resolve_color(spec.border_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let summary_color = theme.resolve_color(spec.summary_color_token());
    let icon_muted = theme.resolve_color("color.icon.muted");
    let toggle_size = theme.resolve_space(spec.toggle_size_token());
    let toggle_radius = theme.resolve_radius(spec.toggle_radius_token());

    let is_expanded = spec.is_grid_visible();
    let had_children = !children.is_empty();

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };

    let mut toolbar = Node::container();
    {
        let s = &mut toolbar.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_inline;
        pad.right = pad_inline;
        pad.top = pad_block;
        pad.bottom = pad_block;
        s.descriptor.background = Some(bg);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
    }
    all_radius(&mut toolbar, radius);
    let mut toolbar = toolbar;

    // ── Header row ──
    let needs_header = spec.collapsible || spec.summary_text.is_some() || actions.is_some();
    if needs_header {
        let mut header = Node::container();
        header.style.descriptor.layout.direction = LayoutDirection::Row;
        header.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        header.style.descriptor.layout.spacing.gap = header_gap;
        let mut header = header;

        // Collapse toggle chevron (chevron-down when expanded, -right when
        // collapsed). Interaction is host-wired via `on_toggle`.
        if spec.collapsible {
            let chevron_name = if is_expanded {
                "chevron-down"
            } else {
                "chevron-right"
            };
            let mut toggle = Node::container();
            {
                let s = &mut toggle.style;
                s.descriptor.layout.width = LayoutSizing::Fixed(toggle_size);
                s.descriptor.layout.height = LayoutSizing::Fixed(toggle_size);
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            }
            all_radius(&mut toggle, toggle_radius);
            if let Some(handler) = &on_toggle {
                let handler = Arc::clone(handler);
                let next = !is_expanded;
                toggle.style.descriptor.cursor = CursorHint::Pointer;
                toggle.interaction.on_activate = Some(Arc::new(move || handler(next)));
            }
            let mut chevron = Node::icon(chevron_name, toggle_size);
            chevron.style.descriptor.text_color = Some(icon_muted);
            header = header.child(toggle.child(chevron));
        }

        // Summary text — grows so the actions slot anchors right (Svelte
        // summary `flex: 1`, actions `margin-left: auto`).
        if let Some(ref summary) = spec.summary_text {
            let mut label = Node::text(summary);
            label.style.descriptor.text_color = Some(summary_color);
            label.style.text_size = Some(font_size);
            label.style.descriptor.layout.width = LayoutSizing::Grow;
            header = header.child(label);
        } else {
            // Reserve the grow space so actions still anchor right.
            let mut spacer = Node::container();
            // Explicit Row (see switch.rs).
            spacer.style.descriptor.layout.direction = LayoutDirection::Row;
            spacer.style.descriptor.layout.width = LayoutSizing::Grow;
            header = header.child(spacer);
        }

        if let Some(actions_el) = actions {
            let mut slot = Node::container();
            slot.style.descriptor.layout.direction = LayoutDirection::Row;
            slot.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            slot.style.descriptor.layout.spacing.gap = actions_gap;
            header = header.child(slot.child(actions_el));
        }

        toolbar = toolbar.child(header);
    }

    // ── Filter controls grid ──
    if is_expanded && had_children {
        let mut grid = Node::container();
        grid.style.descriptor.layout.direction = LayoutDirection::Row;
        grid.style.flex_wrap = true;
        grid.style.descriptor.layout.spacing.gap = controls_gap;
        for child in children {
            let mut cell = Node::container();
            // Explicit Row (see switch.rs).
            cell.style.descriptor.layout.direction = LayoutDirection::Row;
            // flex-grow without cross stretch (old `.flex_grow()`).
            cell.style.flex_fill = true;
            cell.style.min_width = Some(rem_to_px(spec.min_item_width_rem));
            grid = grid.child(cell.child(child));
        }
        toolbar = toolbar.child(grid);
    }

    // ── Secondary slot ──
    if let Some(secondary_el) = secondary {
        toolbar = toolbar.child(secondary_el);
    }

    if !spec.aria_label.is_empty() {
        toolbar.a11y.label = Some(spec.aria_label.clone());
    }
    toolbar.a11y.role = Some(NodeRole::Toolbar);
    toolbar
}
