//! Pill specimen — removable tag/chip for filters and selections.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    // ── Basic pills ──
    label(tree, root, "Basic Pills", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(root, row);

        pill_basic(tree, row, "Design", bg_surface, border, text_primary);
        pill_basic(tree, row, "Engineering", bg_surface, border, text_primary);
        pill_basic(tree, row, "Marketing", bg_surface, border, text_primary);
    }

    // ── Colored pills ──
    label(tree, root, "Colored Pills", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(root, row);

        pill_solid(tree, row, "Active", accent, text_inverse);
        pill_solid(tree, row, "Approved", success, text_inverse);
        pill_solid(tree, row, "Rejected", danger, text_inverse);
    }

    // ── Removable pills ──
    label(tree, root, "Removable (with \u{2715})", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle::default());
        tree.add_child(root, row);

        pill_removable(tree, row, "React", bg_surface, border, text_primary, text_secondary);
        pill_removable(tree, row, "TypeScript", bg_surface, border, text_primary, text_secondary);
        pill_removable(tree, row, "Rust", bg_surface, border, text_primary, text_secondary);
    }

    // ── Disabled ──
    label(tree, root, "Disabled", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(6.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { opacity: 0.5, ..NodeStyle::default() });
        tree.add_child(root, row);

        pill_basic(tree, row, "Read-only", bg_surface, border, text_primary);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn pill_basic(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4) {
    let p = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_padding(LayoutEdges { top: 4.0, right: 10.0, bottom: 4.0, left: 10.0 })),
        NodeStyle {
            corner_radii: [12.0; 4],
            background: Some(bg),
            border_color: Some(border),
            border_width: 1.0,
            text_color: Some(fg),
            text_size: Some(11.0),
            ..NodeStyle::default()
        });
    tree.add_child(parent, p);
}

fn pill_solid(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, fg: glam::Vec4) {
    let p = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_padding(LayoutEdges { top: 4.0, right: 10.0, bottom: 4.0, left: 10.0 })),
        NodeStyle {
            corner_radii: [12.0; 4],
            background: Some(bg),
            text_color: Some(fg),
            text_size: Some(11.0),
            ..NodeStyle::default()
        });
    tree.add_child(parent, p);
}

fn pill_removable(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4, fg_muted: glam::Vec4) {
    let p = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_gap(4.0)
        .with_padding(LayoutEdges { top: 4.0, right: 6.0, bottom: 4.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle {
            corner_radii: [12.0; 4],
            background: Some(bg),
            border_color: Some(border),
            border_width: 1.0,
            ..NodeStyle::default()
        });
    tree.add_child(parent, p);

    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(fg), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(p, lbl);

    let close = tree.create_node(Widget::Label { text: "\u{2715}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(fg_muted), text_size: Some(9.0), ..NodeStyle::default() });
    tree.add_child(p, close);
}
