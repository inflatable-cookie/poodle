//! FilterToolbar — collapsible filter header + responsive controls grid.
//!
//! Contract: `docs/contracts/components/filter-toolbar.md`
//! Ported from: `packages/jetstream/components/src/filter_toolbar.rs`.
//!
//! - `children`: filter controls laid out in a responsive grid
//! - `actions`: optional element rendered in the header row
//! - `secondary`: optional element rendered below the grid

use std::sync::Arc;

use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeRole};
use poodle_specs::{CollapseToggleSpec, FilterToolbarSpec};

use crate::context::{RenderContext, SlotBuilder};
use crate::presentation::rem_to_px;

/// `on_toggle` fires with the expanded state the toolbar is moving **to**.
pub fn filter_toolbar(
    spec: &FilterToolbarSpec,
    ctx: &RenderContext<'_>,
    children: Vec<SlotBuilder<'_>>,
    actions: Option<SlotBuilder<'_>>,
    secondary: Option<SlotBuilder<'_>>,
    on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
) -> Node {
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    // The web pair wraps the whole toolbar — summary, actions, the filter-
    // controls grid, and secondary — in a UiPresentationProvider publishing
    // the raw (not role-mapped) base size and resolved density
    // (FilterToolbar.svelte): host children build inside that scope.
    let host_scope = ctx.scoped(base_size, density);
    let children: Vec<Node> = children
        .into_iter()
        .map(|build| build(&host_scope))
        .collect();
    let actions = actions.map(|build| build(&host_scope));
    let secondary = secondary.map(|build| build(&host_scope));
    // Contract §8 summary size table (size-scaled label-size).
    let font_size = rem_to_px(spec.summary_font_size_rem(base_size));

    // Contract §8 density table: distinct root padding-block / padding-inline,
    // root gap, and controls-grid gap per density.
    let pad_block = rem_to_px(spec.padding_block_rem(density));
    let pad_inline = rem_to_px(spec.padding_inline_rem(density));
    let root_gap = match spec.density_gap_rem(density) {
        Some(rem) => rem_to_px(rem),
        None => ctx.theme().resolve_space(spec.gap_token(density)),
    };
    let header_gap = ctx.theme().resolve_space("space.inline.sm");
    let controls_gap = match spec.density_controls_gap_rem(density) {
        Some(rem) => rem_to_px(rem),
        None => ctx.theme().resolve_space(spec.controls_gap_token(density)),
    };
    let actions_gap = ctx.theme().resolve_space(spec.actions_gap_token());

    let bg = ctx.theme().resolve_color(spec.background_token());
    let border = ctx.theme().resolve_color(spec.border_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    let summary_color = ctx.theme().resolve_color(spec.summary_color_token());
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
        header.a11y.role = Some(NodeRole::Group);
        header.style.descriptor.layout.direction = LayoutDirection::Row;
        header.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        header.style.descriptor.layout.spacing.gap = header_gap;
        let mut header = header;

        // The native CollapseToggle mirrors Svelte's named, keyboard control.
        if spec.collapsible {
            let aria_label = if spec.collapsed {
                match spec.summary_text.as_deref() {
                    Some(summary) => format!("Show filters. {summary}"),
                    None => "Show filters".to_owned(),
                }
            } else {
                "Hide filters".to_owned()
            };
            let toggle_spec = CollapseToggleSpec::new()
                .with_collapsed(spec.collapsed)
                .with_aria_label(aria_label);
            let toggle = crate::collapse_toggle::collapse_toggle(
                &toggle_spec,
                &host_scope,
                on_toggle.clone(),
            );
            header = header.child(toggle);
        }

        // Svelte also lets a click on the surrounding header toggle collapse.
        // The nested toggle and actions slot stop activation before it reaches
        // this pointer convenience handler.
        if spec.collapsible {
            if let Some(handler) = &on_toggle {
                let handler = Arc::clone(handler);
                let next_collapsed = !spec.collapsed;
                header.style.descriptor.cursor = CursorHint::Pointer;
                header.interaction.on_activate = Some(Arc::new(move || handler(next_collapsed)));
            }
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
            // Stop the header's pointer convenience handler for action clicks,
            // including disabled child actions that have no handler of their own.
            slot.interaction.on_activate = Some(Arc::new(|| {}));
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
