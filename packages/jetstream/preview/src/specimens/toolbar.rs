//! Toolbar specimen — horizontal action bar with items, separator, and alignment.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(20.0)),
        NodeStyle::default());

    // ── Basic toolbar ──
    label(tree, root, "Basic Toolbar", text_secondary);
    {
        let bar = toolbar_container(tree, bg_surface, border);
        tree.add_child(root, bar);

        toolbar_btn(tree, bar, "New", bg_surface, border, text_primary);
        toolbar_btn(tree, bar, "Open", bg_surface, border, text_primary);
        toolbar_btn(tree, bar, "Save", accent, border, text_inverse);

        // Separator
        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(1.0))
            .with_height(LayoutSizing::Fixed(20.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(bar, sep);

        toolbar_btn(tree, bar, "Undo", bg_surface, border, text_primary);
        toolbar_btn(tree, bar, "Redo", bg_surface, border, text_primary);
    }

    // ── With spacer (actions pushed right) ──
    label(tree, root, "Toolbar with Spacer", text_secondary);
    {
        let bar = toolbar_container(tree, bg_surface, border);
        tree.add_child(root, bar);

        toolbar_btn(tree, bar, "Back", bg_surface, border, text_primary);

        let spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)),
            NodeStyle::default());
        tree.add_child(bar, spacer);

        toolbar_btn(tree, bar, "Settings", bg_surface, border, text_secondary);
        toolbar_btn(tree, bar, "Help", bg_surface, border, text_secondary);
    }

    // ── Icon toolbar ──
    label(tree, root, "Icon Toolbar", text_secondary);
    {
        let bar = toolbar_container(tree, bg_surface, border);
        tree.add_child(root, bar);

        for &icon in &["\u{270E}", "\u{2702}", "\u{1F4CB}", "\u{27F3}"] {
            let btn = tree.create_node(Widget::Button {
                label: icon.to_string(),
                pressed: false,
                hovered: false,
            }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(32.0))
                .with_height(LayoutSizing::Fixed(28.0))
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
                NodeStyle {
                    corner_radii: [4.0; 4],
                    text_color: Some(text_primary),
                    text_size: Some(14.0),
                    focusable: true,
                    ..NodeStyle::default()
                });
            tree.add_child(bar, btn);
        }

        // Separator
        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(1.0))
            .with_height(LayoutSizing::Fixed(20.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(bar, sep);

        for &icon in &["\u{1F50D}", "\u{2699}"] {
            let btn = tree.create_node(Widget::Button {
                label: icon.to_string(),
                pressed: false,
                hovered: false,
            }, pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Fixed(32.0))
                .with_height(LayoutSizing::Fixed(28.0))
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
                NodeStyle {
                    corner_radii: [4.0; 4],
                    text_color: Some(text_secondary),
                    text_size: Some(14.0),
                    focusable: true,
                    ..NodeStyle::default()
                });
            tree.add_child(bar, btn);
        }
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}

fn toolbar_container(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_width(LayoutSizing::Grow)
        .with_height(LayoutSizing::Fixed(40.0))
        .with_padding(LayoutEdges { top: 4.0, right: 8.0, bottom: 4.0, left: 8.0 })
        .with_gap(4.0)
        .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
        NodeStyle {
            background: Some(bg),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        })
}

fn toolbar_btn(
    tree: &mut UiTree,
    parent: UiNodeId,
    text: &str,
    _bg: glam::Vec4,
    border: glam::Vec4,
    fg: glam::Vec4,
) {
    let btn = tree.create_node(Widget::Button {
        label: text.to_string(),
        pressed: false,
        hovered: false,
    }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_height(LayoutSizing::Fixed(28.0))
        .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
        NodeStyle {
            corner_radii: [4.0; 4],
            border_color: Some(border),
            border_width: 1.0,
            text_color: Some(fg),
            text_size: Some(11.0),
            focusable: true,
            ..NodeStyle::default()
        });
    tree.add_child(parent, btn);
}
