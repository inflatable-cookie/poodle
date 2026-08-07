//! SidebarNav — a grouped navigation rail.
//!
//! Contract: `docs/contracts/components/sidebar-nav.md`
//! Ported from: `packages/jetstream/components/src/sidebar_nav.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodePosition, StylePatch,
};
use poodle_specs::SidebarNavSpec;

use crate::color::with_alpha;
use crate::presentation::rem_to_px;

// ── Active-state alpha factors (contract color-mix percentages) ──
const ACTIVE_BG_ALPHA: f32 = 0.10; // accent-base @ 10%
const ACTIVE_RING_ALPHA: f32 = 0.20; // inset ring accent-base @ 20%
const HOVER_BG_ALPHA: f32 = 0.60; // elevated @ 60%
const SEPARATOR_ALPHA: f32 = 0.54; // border-subtle @ 54%

/// `on_change` fires with the value of the item that was chosen.
pub fn sidebar_nav(
    spec: &SidebarNavSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    // ── Size / density geometry (contract §8 tables, token-resolved rem) ──
    let item_height = rem_to_px(spec.item_height_rem());
    let item_font = rem_to_px(spec.item_font_rem());
    let title_font = rem_to_px(spec.title_font_rem());

    let group_gap = rem_to_px(spec.group_gap_rem());
    let item_px = rem_to_px(spec.item_pad_inline_rem());
    let title_gap = rem_to_px(spec.title_gap_rem());
    let group_internal_gap = rem_to_px(0.3125); // contract group `gap`
    let list_gap = rem_to_px(0.125); // contract list `gap`
    let separator_mt = rem_to_px(0.125); // contract separator margin-top
    let rail_w = rem_to_px(0.1875); // contract left border 3px
    let nav_pad_x = rem_to_px(0.375); // contract root horizontal padding
                                      // Root vertical padding = space-panel-y (density-driven).
    let panel_y = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.5,
        poodle_specs::ControlDensity::Default => 0.75,
        poodle_specs::ControlDensity::Comfortable => 1.0,
    });

    // ── Token resolution ──────────────────────────────────────
    let item_color = theme.resolve_color(spec.item_color_token());
    let item_active_color = theme.resolve_color(spec.item_active_color_token());
    let group_title_color = theme.resolve_color(spec.group_title_color_token());
    let separator_color = theme.resolve_color(spec.separator_color_token());
    let accent = theme.resolve_color(spec.active_indicator_color_token());
    let hover_fill = theme.resolve_color(spec.hover_fill_token());
    let focus_ring = theme.resolve_color(spec.focus_ring_color_token());
    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    let ctrl_radius = theme.resolve_radius("radius.control");
    let item_radius = (ctrl_radius - rem_to_px(0.125)).max(0.0);

    let active_bg = with_alpha(accent, accent.3 * ACTIVE_BG_ALPHA);
    let active_ring = with_alpha(accent, accent.3 * ACTIVE_RING_ALPHA);
    let hover_bg = with_alpha(hover_fill, hover_fill.3 * HOVER_BG_ALPHA);

    let visible_groups = spec.visible_groups();
    let has_multiple_groups = visible_groups.len() > 1;

    // ── Root: <nav> as a flex column with panel padding ──────
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = group_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = panel_y;
        pad.bottom = panel_y;
        pad.left = nav_pad_x;
        pad.right = nav_pad_x;
    }

    for (gi, group) in visible_groups.iter().enumerate() {
        let mut group_el = Node::container();
        {
            let s = &mut group_el.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = group_internal_gap;

            // Inter-group separator: top border + top padding on the group
            // element (matches the Svelte adjacent-sibling rule).
            if has_multiple_groups && gi > 0 {
                s.descriptor.layout.spacing.margin.top = separator_mt;
                s.descriptor.layout.spacing.padding.top = group_gap - separator_mt;
                s.border_top_width = Some(1.0);
                s.border_color_top = Some(with_alpha(
                    separator_color,
                    separator_color.3 * SEPARATOR_ALPHA,
                ));
            }
        }

        // Group title — uppercase, caption-sized, accent-tinted.
        if let Some(ref label_text) = group.label {
            let mut title = Node::text(label_text.to_uppercase());
            {
                let s = &mut title.style;
                s.descriptor.text_color = Some(group_title_color);
                s.text_size = Some(title_font);
                s.text_weight = Some(700);
                s.line_height = Some(1.2); // contract §8 title line-height
                s.letter_spacing_em = Some(0.18); // contract §8 title tracking
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = item_px;
                pad.right = item_px;
                s.descriptor.layout.spacing.margin.bottom = title_gap;
            }
            group_el = group_el.child(title);
        }

        // Item list
        let mut list = Node::container();
        {
            let s = &mut list.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = list_gap;
        }

        for item in &group.items {
            let is_active = spec.is_active(&item.value);

            // Item box: min-height drives the row height; vertical centring
            // stands in for the contract padding-block on single-line labels.
            let mut item_el = Node::button(&item.label);
            {
                let s = &mut item_el.style;
                s.min_height = Some(item_height);
                s.self_stretch = true;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.text_size = Some(item_font);
                s.line_height = Some(1.3); // contract §8 item line-height
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = item_px;
                pad.right = item_px;
                let c = &mut s.descriptor.corner_radii;
                c.top_left = item_radius;
                c.top_right = item_radius;
                c.bottom_right = item_radius;
                c.bottom_left = item_radius;
                // Reserve a 3px transparent left rail on every item so
                // active ↔ inactive does not shift horizontally.
                s.border_left_width = Some(rail_w);
                s.border_color_left = Some(ColorValue(0.0, 0.0, 0.0, 0.0));

                if is_active {
                    // Active: accent left rail + bg fill + bolder weight. The
                    // inset ring is a separate child below, NOT a uniform
                    // border here: gpui has one border colour per element, so
                    // an accent left rail plus a ring-coloured box on the same
                    // node collapses to one colour on all four sides and the
                    // rail disappears into a full accent outline.
                    s.descriptor.text_color = Some(item_active_color);
                    s.text_weight = Some(600);
                    s.descriptor.background = Some(active_bg);
                    s.border_color_left = Some(accent);
                } else {
                    s.descriptor.text_color = Some(item_color);
                    s.text_weight = Some(500);
                }
            }

            if is_active {
                // Inset ring: a full-bleed 1px accent@20% overlay, matching the
                // old tier's emulation of an inset box-shadow.
                let mut ring = Node::container();
                {
                    let s = &mut ring.style;
                    s.descriptor.border.width = 1.0;
                    s.descriptor.border.color = active_ring;
                    let c = &mut s.descriptor.corner_radii;
                    c.top_left = item_radius;
                    c.top_right = item_radius;
                    c.bottom_right = item_radius;
                    c.bottom_left = item_radius;
                }
                ring.position = NodePosition::Absolute {
                    top: Some(0.0),
                    left: Some(0.0),
                    right: Some(0.0),
                    bottom: Some(0.0),
                };
                item_el.position = NodePosition::Relative;
                item_el = item_el.child(ring);
            }

            if item.is_disabled {
                item_el.style.descriptor.opacity = disabled_opacity;
            } else {
                if let Some(handler) = &on_change {
                    let handler = Arc::clone(handler);
                    let value = item.value.clone();
                    item_el.interaction.on_activate = Some(Arc::new(move || handler(&value)));
                }

                item_el.interaction.focusable = true;
                let s = &mut item_el.style;
                s.descriptor.cursor = CursorHint::Pointer;
                // Hover: text-primary + elevated@60% bg (contract §4/§8).
                s.hover = Some(StylePatch {
                    background: Some(hover_bg),
                    border_color: None,
                    text_color: Some(item_active_color),
                    opacity: None,
                });
                // Focus-visible: accent focus ring (contract §6/§8).
                s.active = Some(StylePatch {
                    background: None,
                    border_color: Some(focus_ring),
                    text_color: None,
                    opacity: None,
                });
            }

            list = list.child(item_el);
        }

        group_el = group_el.child(list);
        el = el.child(group_el);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
