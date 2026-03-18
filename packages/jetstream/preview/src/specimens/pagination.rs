//! Pagination specimen — page navigation controls.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, CrossAxisAlignment, MainAxisAlignment};

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
        .with_gap(16.0)), NodeStyle::default());

    // ── Standard pagination ──
    section_label(tree, root, "Standard", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(4.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle::default());
        tree.add_child(root, row);

        page_btn(tree, row, "◂", bg_surface, border, text_primary, false);
        page_btn(tree, row, "1", accent, border, text_inverse, true);
        page_btn(tree, row, "2", bg_surface, border, text_primary, false);
        page_btn(tree, row, "3", bg_surface, border, text_primary, false);
        page_btn(tree, row, "…", bg_surface, border, text_secondary, false);
        page_btn(tree, row, "10", bg_surface, border, text_primary, false);
        page_btn(tree, row, "▸", bg_surface, border, text_primary, false);
    }

    // ── Middle page active ──
    section_label(tree, root, "Middle Page Active", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(4.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle::default());
        tree.add_child(root, row);

        page_btn(tree, row, "◂", bg_surface, border, text_primary, false);
        page_btn(tree, row, "1", bg_surface, border, text_primary, false);
        page_btn(tree, row, "…", bg_surface, border, text_secondary, false);
        page_btn(tree, row, "4", bg_surface, border, text_primary, false);
        page_btn(tree, row, "5", accent, border, text_inverse, true);
        page_btn(tree, row, "6", bg_surface, border, text_primary, false);
        page_btn(tree, row, "…", bg_surface, border, text_secondary, false);
        page_btn(tree, row, "10", bg_surface, border, text_primary, false);
        page_btn(tree, row, "▸", bg_surface, border, text_primary, false);
    }

    // ── Simple prev/next ──
    section_label(tree, root, "Simple Prev/Next", text_secondary);
    {
        let row = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(8.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)), NodeStyle::default());
        tree.add_child(root, row);

        let prev = tree.create_node(Widget::Button {
            label: "← Previous".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(12.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(row, prev);

        let info = tree.create_node(Widget::Label { text: "Page 3 of 10".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(row, info);

        let next = tree.create_node(Widget::Button {
            label: "Next →".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(32.0))
            .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(12.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(row, next);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn page_btn(tree: &mut UiTree, parent: UiNodeId, label: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4, active: bool) {
    let btn = tree.create_node(Widget::Button {
        label: label.to_string(), pressed: false, hovered: false,
    }, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_width(LayoutSizing::Fixed(32.0))
        .with_height(LayoutSizing::Fixed(32.0))
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
        corner_radii: [6.0; 4], background: Some(bg),
        border_color: if active { None } else { Some(border) },
        border_width: if active { 0.0 } else { 1.0 },
        text_color: Some(fg), text_size: Some(12.0),
        focusable: true, ..NodeStyle::default()
    });
    tree.add_child(parent, btn);
}
