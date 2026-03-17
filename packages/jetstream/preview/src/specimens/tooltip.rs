//! Tooltip specimen — small informational overlay on hover.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    // Dark tooltip background (inverted)
    let tooltip_bg = glam::Vec4::new(0.15, 0.15, 0.18, 0.95);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Basic tooltip ──
    section_label(tree, root, "Basic Tooltip", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        let trigger = tree.create(Widget::Button {
            label: "Hover me".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            corner_radii: [6.0; 4], background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(col, trigger);

        tooltip_bubble(tree, col, "This is a tooltip", tooltip_bg, text_inverse);
    }

    // ── Placement variants ──
    section_label(tree, root, "Placement Variants (mockup)", text_secondary);
    {
        let grid = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 12.0, ..UiStyle::default()
        });
        tree.add_child(root, grid);

        for &(label, placement) in &[
            ("Top", "↑ Tooltip above"),
            ("Bottom", "↓ Tooltip below"),
            ("Left", "← Tooltip left"),
            ("Right", "→ Tooltip right"),
        ] {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 8.0, align: Align::Center, ..UiStyle::default()
            });
            tree.add_child(grid, row);

            let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0),
                width: Sizing::Fixed(50.0), ..UiStyle::default()
            });
            tree.add_child(row, lbl);

            tooltip_bubble(tree, row, placement, tooltip_bg, text_inverse);
        }
    }

    // ── Multi-line tooltip ──
    section_label(tree, root, "Multi-line Tooltip", text_secondary);
    {
        let tip = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges { top: 6.0, right: 10.0, bottom: 6.0, left: 10.0 },
            gap: 2.0,
            corner_radii: [6.0; 4],
            background: Some(tooltip_bg),
            ..UiStyle::default()
        });
        tree.add_child(root, tip);

        let line1 = tree.create(Widget::Label { text: "Keyboard shortcut".to_string() }, UiStyle {
            text_color: Some(text_inverse), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(tip, line1);

        let line2 = tree.create(Widget::Label { text: "⌘ + S to save".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_inverse, 0.7)),
            text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(tip, line2);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn tooltip_bubble(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, fg: glam::Vec4) {
    let tip = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        padding: Edges { top: 4.0, right: 8.0, bottom: 4.0, left: 8.0 },
        corner_radii: [4.0; 4],
        background: Some(bg),
        text_color: Some(fg),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(parent, tip);
}
