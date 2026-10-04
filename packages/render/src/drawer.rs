//! Drawer — an edge-anchored slide-out panel over an optional scrim.
//!
//! Contract: `docs/contracts/components/drawer.md`
//! Ported from: `packages/jetstream/components/src/drawer.rs`. No close
//! affordance — the contract anatomy has none; the backdrop is the only
//! dismissal route the component draws.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, DismissReason, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodePosition, NodeRole,
};
use poodle_specs::{DrawerEdge, DrawerSpec};

use crate::context::RenderContext;
use crate::presentation::{
    drawer_title_font_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, size_font_rem,
};

pub fn drawer(
    spec: &DrawerSpec,
    ctx: &RenderContext<'_>,
    content: Option<Node>,
    actions: Option<Node>,
    on_request_close: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let title_font = rem_to_px(drawer_title_font_rem(effective_size));
    let body_font = rem_to_px(size_font_rem(effective_size));
    let space_x = rem_to_px(panel_space_x_rem(density));
    let space_y = rem_to_px(panel_space_y_rem(density));
    let header_gap = rem_to_px(0.375);
    let panel_gap = ctx.theme().resolve_space("space.stack.sm");
    let stack_md = ctx.theme().resolve_space("space.stack.md");
    let actions_gap = ctx.theme().resolve_space("space.inline.sm");

    let fill = ctx.theme().resolve_color(spec.surface_fill_token());
    let backdrop = ctx.theme().resolve_color(spec.backdrop_fill_token());
    let border = ctx.theme().resolve_color("color.border.default");
    let title_color = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");

    let side_width = rem_to_px(28.0);
    let edge_height = rem_to_px(24.0);

    // ── Panel: edge-specific sizing and border edge ──
    let mut panel = Node::container();
    panel.id = Some("poodle-drawer-surface".to_string());
    panel.a11y.role = Some(NodeRole::Dialog);
    panel.a11y.initial_focus = spec.is_modal;
    if spec.is_modal {
        panel.interaction.focusable = true;
        panel.a11y.tab_index = Some(-1);
    }
    {
        let s = &mut panel.style;
        s.descriptor.background = Some(fill);
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = panel_gap;
        s.descriptor.layout.spacing.padding.left = space_x;
        s.descriptor.layout.spacing.padding.right = space_x;
        s.descriptor.layout.spacing.padding.top = space_y;
        s.descriptor.layout.spacing.padding.bottom = space_y;
        s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_DIALOG);
        s.descriptor.border.color = border;
        match spec.edge {
            DrawerEdge::Right => {
                s.fill_height = true;
                s.descriptor.layout.width = LayoutSizing::Fixed(side_width);
                s.border_left_width = Some(1.0);
            }
            DrawerEdge::Left => {
                s.fill_height = true;
                s.descriptor.layout.width = LayoutSizing::Fixed(side_width);
                s.border_right_width = Some(1.0);
            }
            DrawerEdge::Bottom => {
                s.fill_width = true;
                s.descriptor.layout.height = LayoutSizing::Fixed(edge_height);
                s.border_top_width = Some(1.0);
            }
            DrawerEdge::Top => {
                s.fill_width = true;
                s.descriptor.layout.height = LayoutSizing::Fixed(edge_height);
                s.border_bottom_width = Some(1.0);
            }
        }
    }

    // ── Header (no close button, per contract) ──
    if spec.title.is_some() || spec.description.is_some() {
        let mut header = Node::container();
        {
            let s = &mut header.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = header_gap;
            s.descriptor.layout.spacing.margin.bottom = stack_md;
        }
        if let Some(ref title) = spec.title {
            let mut t = Node::text(title);
            t.id = Some("poodle-drawer-title".to_string());
            t.style.descriptor.text_color = Some(title_color);
            t.style.text_size = Some(title_font);
            t.style.text_weight = Some(600);
            header = header.child(t);
        }
        if let Some(ref description) = spec.description {
            let mut d = Node::text(description);
            d.style.descriptor.text_color = Some(text_secondary);
            d.style.text_size = Some(body_font);
            header = header.child(d);
        }
        panel = panel.child(header);
    }

    // ── Body grows so actions pin to the bottom ──
    if let Some(content_el) = content {
        let mut body = Node::container();
        // Explicit Row (see switch.rs).
        body.style.descriptor.layout.direction = LayoutDirection::Row;
        body.style.descriptor.layout.width = LayoutSizing::Grow;
        panel = panel.child(body.child(content_el));
    }

    // ── Actions footer ──
    if let Some(actions_el) = actions {
        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_wrap = true;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            s.descriptor.layout.spacing.gap = actions_gap;
            s.descriptor.layout.spacing.margin.top = stack_md;
        }
        panel = panel.child(row.child(actions_el));
    }

    if spec.title.is_some() {
        panel.a11y.labelled_by = Some("poodle-drawer-title".to_string());
        panel.a11y.label = spec.title.clone();
    } else if let Some(label) = spec.aria_label.as_deref() {
        panel.a11y.label = Some(label.to_string());
    }

    // ── Overlay: window-hosted backdrop, edge controls the anchor ──
    let mut overlay = Node::container();
    overlay.id = Some("poodle-drawer-backdrop".to_string());
    overlay.position = NodePosition::Absolute {
        top: Some(0.0),
        left: Some(0.0),
        right: Some(0.0),
        bottom: Some(0.0),
    };
    {
        let s = &mut overlay.style;
        s.overlay = true;
        if spec.is_modal {
            s.descriptor.background = Some(backdrop);
        }
        match spec.edge {
            DrawerEdge::Right => {
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            }
            DrawerEdge::Left => {
                s.descriptor.layout.direction = LayoutDirection::Row;
                // justify_start is taffy's default: silence.
            }
            DrawerEdge::Bottom => {
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            }
            DrawerEdge::Top => {
                s.descriptor.layout.direction = LayoutDirection::Column;
            }
        }
    }

    if on_request_close.is_some() {
        panel.interaction.dismiss_layer = Some("poodle-drawer-layer".to_string());
        panel.interaction.on_activate = Some(Arc::new(|| {}));
        if spec.dismiss_on_escape {
            let handler = Arc::clone(on_request_close.as_ref().unwrap());
            let outside = spec.dismiss_on_outside_interact;
            panel.interaction.on_dismiss = Some(Arc::new(move |reason| match reason {
                DismissReason::Escape => handler(),
                DismissReason::Outside if outside => handler(),
                _ => {}
            }));
        }
    }

    if spec.is_modal {
        let mut backdrop_button = Node::container();
        backdrop_button.id = Some("poodle-drawer-backdrop-dismiss".to_string());
        backdrop_button.a11y.role = Some(NodeRole::Button);
        backdrop_button.a11y.label = Some("Dismiss drawer backdrop".to_string());
        backdrop_button.interaction.focusable = true;
        backdrop_button.position = NodePosition::Absolute {
            top: Some(0.0),
            left: Some(0.0),
            right: Some(0.0),
            bottom: Some(0.0),
        };
        backdrop_button.style.descriptor.layout.direction = LayoutDirection::Row;
        if let (true, Some(handler)) = (spec.dismiss_on_backdrop, &on_request_close) {
            let handler = Arc::clone(handler);
            backdrop_button.interaction.on_activate = Some(Arc::new(move || handler()));
        }
        overlay.child(backdrop_button).child(panel)
    } else {
        overlay.child(panel)
    }
}
