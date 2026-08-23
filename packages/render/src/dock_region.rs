//! DockRegion — a dockable panel with tabs.
//!
//! Contract: `docs/contracts/components/dock-region.md`
//! Ported from: `packages/jetstream/components/src/dock_region.rs`.
//!
//! The drag events (`onDragStart`, reorder, panel drop) are drag-with-payload
//! gestures the vocabulary does not carry; recorded as a delta.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeRole, StylePatch,
};
use poodle_specs::{
    DockCollapsedPosture, DockEdge, DockEmphasis, DockRegionSpec, DockSizing, DockTabsPlacement,
};

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, size_font_rem,
};

/// Handlers mirror the GPUI target's names.
#[derive(Default)]
pub struct DockRegionHandlers {
    /// Fires with the tab's value when one is pressed.
    pub on_tab_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with the collapsed state the region is moving **to**.
    pub on_collapse_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    /// Stable native instance scope. Two docks with the same tab values
    /// would otherwise share one backend focus handle.
    pub instance_id: Option<String>,
}

/// The backend-state id of one dock tab.
pub fn dock_tab_focus_id(instance_id: Option<&str>, value: &str) -> String {
    match instance_id {
        Some(scope) => format!("dock-region:{scope}:tab:{value}"),
        None => format!("dock-tab-{value}"),
    }
}

/// The backend-state id of the collapse control.
pub fn dock_collapse_focus_id(instance_id: Option<&str>) -> String {
    match instance_id {
        Some(scope) => format!("dock-region:{scope}:collapse"),
        None => "dock-collapse".to_string(),
    }
}

fn scoped(instance_id: Option<&str>, part: &str) -> Option<String> {
    instance_id.map(|scope| format!("dock-region:{scope}:{part}"))
}

pub fn dock_region(
    spec: &DockRegionSpec,
    ctx: &RenderContext<'_>,
    content: Option<Node>,
    handlers: DockRegionHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let tab_font = rem_to_px(size_font_rem(effective_size));
    // Density → spacing (contract: density controls horizontal padding / gaps).
    let density = ctx.resolve_density(spec.density);
    let space_x = rem_to_px(control_space_x_rem(density));
    let space_y = rem_to_px(panel_space_y_rem(density));
    let tab_gap = space_x * 0.5;
    let border_w = ctx.theme().resolve_border_width("border.width.default");

    let fill = ctx.theme().resolve_color(spec.strip_fill_token());
    let panel_fill = ctx.theme().resolve_color("color.background.panel");
    let border_subtle = ctx.theme().resolve_color("color.border.subtle");
    let radius_control = ctx.theme().resolve_radius("radius.control");
    let text_muted = ctx.theme().resolve_color("color.text.secondary");
    let accent = ctx.theme().resolve_color("color.accent.base");
    let hover_bg = ctx.theme().resolve_color("color.background.hover");
    // Active-tab fill: accent mixed into the strip fill (matches GPUI's
    // `accent.opacity(0.10)` and the Svelte active-tab tint). TOKEN GAP: no
    // semantic opacity token for the selected-tab tint strength, so the ratio
    // is a single named contract-exact constant.
    const ACTIVE_TAB_ACCENT_RATIO: f32 = 0.10;
    let active_bg = mix_srgb(accent, fill, ACTIVE_TAB_ACCENT_RATIO);

    let is_side_edge = matches!(spec.edge, DockEdge::Left | DockEdge::Right);
    let is_tabs_on_edge = spec.tabs_placement == DockTabsPlacement::Edge && is_side_edge;
    let active = spec.current_value().map(|s| s.to_string());

    // ── Root emphasis treatment ────────────────────────────────
    // Standard: panel fill + subtle border. Quiet: transparent. Strong: accent
    // mixed 32% into the subtle border.
    let (root_bg, root_border) = match spec.emphasis {
        DockEmphasis::Standard => (panel_fill, border_subtle),
        DockEmphasis::Quiet => (with_alpha(panel_fill, 0.0), with_alpha(border_subtle, 0.0)),
        DockEmphasis::Strong => (panel_fill, mix_srgb(accent, border_subtle, 0.32)),
    };

    // A dock borders only the edge it docks against — a left dock rules its
    // right side, a top dock its bottom. A box on all four sides reads as a
    // detached card rather than a region attached to the shell.
    let apply_edge_border = |s: &mut poodle_node::NodeStyle| {
        // Only the width is per-side; the colour rides the uniform channel,
        // which every side falls back to. The uniform WIDTH stays zero so no
        // box is drawn.
        s.descriptor.border.color = root_border;
        match spec.edge {
            DockEdge::Left => s.border_right_width = Some(border_w),
            DockEdge::Right => s.border_left_width = Some(border_w),
            DockEdge::Top => s.border_bottom_width = Some(border_w),
            DockEdge::Bottom => s.border_top_width = Some(border_w),
        }
        s.descriptor.background = Some(root_bg);
    };

    // Build a single tab. `compact` → icon-only (label suppressed); `vertical`
    // → full-width stacked entry.
    let build_tab =
        |value: &str, label: &str, icon: Option<&str>, compact: bool, vertical: bool| -> Node {
            let is_active = active.as_deref() == Some(value);

            let mut tab_btn = Node::button("");
            tab_btn.id = Some(format!("dock-tab-{value}"));
            tab_btn.runtime_id = scoped(handlers.instance_id.as_deref(), &format!("tab:{value}"));
            {
                let s = &mut tab_btn.style;
                // Icon and label are separate children behind a gap, not one
                // interpolated string — a joined string collapses the gap to a
                // single space and shifts every tab's centring.
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = space_x * 0.5;
                // Active tabs read as an accent-tinted pill: accent text on an
                // accent-into-strip fill, at the same weight as the rest. The
                // underline-plus-bold treatment this used to carry belongs to
                // TabStrip, not to a dock's panel tabs.
                s.descriptor.text_color = Some(if is_active { accent } else { text_muted });
                s.text_size = Some(tab_font);
                s.text_weight = Some(400);
                s.descriptor.cursor = CursorHint::Pointer;
                let c = &mut s.descriptor.corner_radii;
                c.top_left = radius_control;
                c.top_right = radius_control;
                c.bottom_right = radius_control;
                c.bottom_left = radius_control;
                let pad = &mut s.descriptor.layout.spacing.padding;
                if vertical {
                    s.fill_width = true;
                    pad.top = space_y * 0.5;
                    pad.bottom = space_y * 0.5;
                    pad.left = space_x;
                    pad.right = space_x;
                } else if compact {
                    pad.left = space_x * 0.5;
                    pad.right = space_x * 0.5;
                    pad.top = space_y * 0.5;
                    pad.bottom = space_y * 0.5;
                } else {
                    pad.left = space_x;
                    pad.right = space_x;
                    pad.top = space_y * 0.5;
                    pad.bottom = space_y * 0.5;
                }
                if is_active {
                    s.descriptor.background = Some(active_bg);
                } else {
                    s.hover = Some(poodle_node::StylePatch {
                        background: Some(hover_bg),
                        ..poodle_node::StylePatch::default()
                    });
                }
            }
            // Icon-only compact / icon-strip tabs render the icon glyph;
            // otherwise icon (when present) then label.
            let mut tab_btn = tab_btn;
            if let Some(ic) = icon {
                let mut glyph = Node::text(ic.to_string());
                glyph.style.descriptor.text_color =
                    Some(if is_active { accent } else { text_muted });
                tab_btn = tab_btn.child(glyph);
            }
            if !compact || icon.is_none() {
                tab_btn = tab_btn.child(Node::text(label.to_string()));
            }
            let mut tab_btn = tab_btn;
            tab_btn.interaction.focusable = true;
            tab_btn.style.focus = Some(StylePatch {
                background: None,
                border_color: Some(accent),
                text_color: None,
                opacity: None,
            });
            if let Some(handler) = &handlers.on_tab_change {
                let handler = Arc::clone(handler);
                let value = value.to_string();
                tab_btn.interaction.on_activate = Some(Arc::new(move || handler(&value)));
            }

            tab_btn
        };

    // Collapse toggle (only when collapsible).
    let build_toggle = |vertical: bool| -> Node {
        let glyph = match spec.edge {
            DockEdge::Left | DockEdge::Top => "‹",
            DockEdge::Right | DockEdge::Bottom => "›",
        };
        let mut t = Node::button(glyph);
        t.id = Some("dock-collapse".to_string());
        t.runtime_id = scoped(handlers.instance_id.as_deref(), "collapse");
        {
            let s = &mut t.style;
            s.descriptor.text_color = Some(text_muted);
            s.text_size = Some(tab_font);
            s.descriptor.cursor = CursorHint::Pointer;
            let pad = &mut s.descriptor.layout.spacing.padding;
            if vertical {
                s.fill_width = true;
                pad.top = space_y * 0.5;
                pad.bottom = space_y * 0.5;
            } else {
                pad.left = space_x * 0.5;
                pad.right = space_x * 0.5;
                pad.top = space_y * 0.5;
                pad.bottom = space_y * 0.5;
            }
        }
        t.interaction.focusable = true;
        t.style.focus = Some(StylePatch {
            background: None,
            border_color: Some(accent),
            text_color: None,
            opacity: None,
        });
        if let Some(handler) = &handlers.on_collapse_toggle {
            let handler = Arc::clone(handler);
            let next = !spec.is_collapsed;
            t.interaction.on_activate = Some(Arc::new(move || handler(next)));
        }
        t
    };

    // Drop-zone overlay affordance — dashed accent border + tinted fill
    // (contract .dock-region__drop-zone: 0.125rem dashed accent).
    let drop_zone = || -> Node {
        let mut z = Node::container();
        let s = &mut z.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.descriptor.border.width = border_w * 2.0;
        s.border_dashed = true;
        s.descriptor.border.color = accent;
        s.descriptor.background = Some(with_alpha(accent, accent.3 * 0.08));
        z
    };

    // ── Static mode: stacked panels, no tabs / collapse ─────────
    if spec.sizing == DockSizing::Static {
        let mut stack = Node::container();
        {
            let s = &mut stack.style;
            s.descriptor.layout.direction = if is_side_edge {
                LayoutDirection::Row
            } else {
                LayoutDirection::Column
            };
            s.descriptor.background = Some(root_bg);
            apply_edge_border(s);
            s.descriptor.layout.width = LayoutSizing::Grow;
        }
        let mut stack = stack;

        for item in &spec.items {
            let mut cell = Node::container();
            cell.id = Some(format!("dock-stack-{}", item.value));
            {
                let s = &mut cell.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.flex_grow = Some(1.0);
                s.flex_basis = Some(0.0);
                s.min_width = Some(0.0);
                if spec.can_accept_panel {
                    // Drop-target ring affordance per stack item.
                    s.descriptor.border.width = border_w;
                    s.descriptor.border.color = accent;
                }
            }
            stack = stack.child(cell.child(Node::text(&item.label)));
        }
        if let Some(c) = content {
            stack = stack.child(c);
        }
        if spec.can_accept_panel {
            stack = stack.child(drop_zone());
        }
        return stack;
    }

    // ── Collapsed: hidden posture (toggle only) ─────────────────
    if spec.is_collapsed && spec.collapsed_posture == DockCollapsedPosture::Hidden {
        let mut region = Node::container();
        {
            let s = &mut region.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }
        if spec.is_collapsible {
            let mut toggle = build_toggle(false);
            toggle.style.descriptor.layout.spacing.padding.top = space_y;
            toggle.style.descriptor.layout.spacing.padding.bottom = space_y;
            return region.child(toggle);
        }
        return region;
    }

    // ── Collapsed: icon-strip posture ───────────────────────────
    if spec.is_collapsed && spec.collapsed_posture == DockCollapsedPosture::IconStrip {
        if is_side_edge {
            // Vertical icon strip: toggle on top, icon-only tabs stacked.
            let mut strip = Node::container();
            {
                let s = &mut strip.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = space_y * 0.5;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.top = space_y;
                pad.bottom = space_y;
                s.descriptor.background = Some(fill);
                apply_edge_border(s);
            }
            let mut strip = strip;
            if spec.is_collapsible {
                strip = strip.child(build_toggle(true));
            }
            for item in &spec.items {
                strip = strip.child(build_tab(
                    &item.value,
                    &item.label,
                    item.icon.as_deref(),
                    true,
                    true,
                ));
            }
            return strip;
        } else {
            // Horizontal compact icon strip: icon-only tabs + toggle, no body.
            let mut strip = Node::container();
            {
                let s = &mut strip.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = tab_gap;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = space_x;
                pad.right = space_x;
                pad.top = space_y * 0.5;
                pad.bottom = space_y * 0.5;
                s.descriptor.background = Some(fill);
                apply_edge_border(s);
            }
            let mut strip = strip;
            for item in &spec.items {
                strip = strip.child(build_tab(
                    &item.value,
                    &item.label,
                    item.icon.as_deref(),
                    true,
                    false,
                ));
            }
            if spec.is_collapsible {
                strip = strip.child(build_toggle(false));
            }
            return strip;
        }
    }

    // ── Expanded flexible mode: strip (tabs + toggle) + body ────
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(root_bg);
        apply_edge_border(s);
        // A side dock runs its strip down the edge and puts the body beside it;
        // a top/bottom dock stacks strip over body.
        if is_tabs_on_edge {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.fill_height = true;
        } else {
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.fill_width = true;
        }
        s.descriptor.layout.width = LayoutSizing::Grow;
    }
    el.a11y.role = Some(NodeRole::TabList);

    let mut tab_bar = Node::container();
    {
        let s = &mut tab_bar.style;
        s.descriptor.background = Some(fill);
        if is_tabs_on_edge {
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = space_y * 0.5;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = space_y;
            pad.bottom = space_y;
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = tab_gap;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = space_x;
            pad.right = space_x;
            pad.top = space_y * 0.5;
            pad.bottom = space_y * 0.5;
            s.border_bottom_width = Some(border_w);
            s.descriptor.border.color = border_subtle;
        }
    }

    // Tab list grows; toggle pinned at the end.
    let mut tab_list = Node::container();
    {
        let s = &mut tab_list.style;
        if is_tabs_on_edge {
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = space_y * 0.5;
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = tab_gap;
        }
        s.flex_grow = Some(1.0);
        s.flex_basis = Some(0.0);
        s.min_width = Some(0.0);
    }
    let mut tab_list = tab_list;
    for item in &spec.items {
        tab_list = tab_list.child(build_tab(
            &item.value,
            &item.label,
            item.icon.as_deref(),
            false,
            is_tabs_on_edge,
        ));
    }
    let mut tab_bar = tab_bar.child(tab_list);
    if spec.is_collapsible {
        tab_bar = tab_bar.child(build_toggle(is_tabs_on_edge));
    }

    let body = content.map(|c| {
        let mut body = Node::container();
        // Explicit Row (see switch.rs).
        body.style.descriptor.layout.direction = LayoutDirection::Row;
        body.style.flex_grow = Some(1.0);
        body.style.flex_basis = Some(0.0);
        body.style.min_width = Some(0.0);
        body.style.min_height = Some(0.0);
        body.style.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        body.style.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        body.child(c)
    });

    // A right-edge dock keeps its strip against the shell edge, so the body
    // comes first.
    let mut el = if is_tabs_on_edge && spec.edge == DockEdge::Right {
        let el = match body {
            Some(b) => el.child(b),
            None => el,
        };
        el.child(tab_bar)
    } else {
        let el = el.child(tab_bar);
        match body {
            Some(b) => el.child(b),
            None => el,
        }
    };

    // Cross-region drop-zone affordance.
    if spec.can_accept_panel {
        el = el.child(drop_zone());
    }

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
    use poodle_specs::PanelTabItem;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> DockRegionSpec {
        DockRegionSpec::new(
            DockEdge::Left,
            vec![PanelTabItem::new("search", "Search")],
        )
        .with_collapsible(true)
    }

    #[test]
    fn an_instance_scope_isolates_backend_state_ids() {
        let scoped = |scope: &str| DockRegionHandlers {
            instance_id: Some(scope.to_string()),
            ..DockRegionHandlers::default()
        };
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let first = dock_region(&spec(), &ctx, None, scoped("first"));
        let second = dock_region(&spec(), &ctx, None, scoped("second"));
        let tab = dock_tab_focus_id(Some("first"), "search");
        let collapse = dock_collapse_focus_id(Some("first"));
        assert!(first
            .find(&|n| n.runtime_id.as_deref() == Some(tab.as_str()))
            .is_some());
        assert!(first
            .find(&|n| n.runtime_id.as_deref() == Some(collapse.as_str()))
            .is_some());
        assert!(first
            .find(&|n| n.runtime_id.as_deref()
                == Some(dock_tab_focus_id(Some("second"), "search").as_str()))
            .is_none());
        assert!(first
            .find(&|n| n.id.as_deref() == Some("dock-tab-search"))
            .is_some());
        assert!(second
            .find(&|n| n.runtime_id.as_deref()
                == Some(dock_collapse_focus_id(Some("second")).as_str()))
            .is_some());
    }
}
