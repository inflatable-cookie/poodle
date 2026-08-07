//! Drawer — an edge-anchored slide-out panel over an optional scrim.
//!
//! Contract: `docs/contracts/components/drawer.md`
//! Ported from: `packages/jetstream/components/src/drawer.rs`. No close
//! affordance — the contract anatomy has none; the backdrop is the only
//! dismissal route the component draws.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, NodeRole,
};
use poodle_specs::{DrawerEdge, DrawerSpec};

use crate::presentation::{
    drawer_title_font_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};

pub fn drawer(
    spec: &DrawerSpec,
    theme: &dyn ThemeProvider,
    content: Option<Node>,
    actions: Option<Node>,
    on_request_close: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_font = rem_to_px(drawer_title_font_rem(effective_size));
    let body_font = rem_to_px(size_font_rem(effective_size));
    let space_x = rem_to_px(panel_space_x_rem(spec.density));
    let space_y = rem_to_px(panel_space_y_rem(spec.density));
    let header_gap = rem_to_px(0.375);
    let panel_gap = theme.resolve_space("space.stack.sm");
    let stack_md = theme.resolve_space("space.stack.md");
    let actions_gap = theme.resolve_space("space.inline.sm");

    let fill = theme.resolve_color(spec.surface_fill_token());
    let backdrop = theme.resolve_color(spec.backdrop_fill_token());
    let border = theme.resolve_color("color.border.default");
    let title_color = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");

    let side_width = rem_to_px(28.0);
    let edge_height = rem_to_px(24.0);

    // ── Panel: edge-specific sizing and border edge ──
    let mut panel = Node::container();
    panel.a11y.role = Some(NodeRole::Dialog);
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

    // ── Overlay: edge controls the anchor ──
    let mut overlay = Node::container();
    {
        let s = &mut overlay.style;
        s.descriptor.background = Some(backdrop);
        s.overlay = true;
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

    // Inside-clicks must end at the panel, not reach the dismissing backdrop.
    if let (true, true, Some(handler)) =
        (spec.is_modal, spec.dismiss_on_backdrop, &on_request_close)
    {
        let handler = Arc::clone(handler);
        overlay.interaction.on_activate = Some(Arc::new(move || handler()));
        panel.interaction.on_activate = Some(Arc::new(|| {}));
    }

    let mut root = overlay.child(panel);
    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root
}
