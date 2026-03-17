//! Skeleton specimen — placeholder loading shapes.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_secondary = theme_bridge::text_secondary(theme);
    let shimmer = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Text lines ──
    label(tree, root, "Text Lines", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 6.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        skeleton_rect(tree, col, 240.0, 12.0, 4.0, shimmer);
        skeleton_rect(tree, col, 200.0, 12.0, 4.0, shimmer);
        skeleton_rect(tree, col, 160.0, 12.0, 4.0, shimmer);
    }

    // ── Heading + paragraph ──
    label(tree, root, "Heading + Paragraph", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        skeleton_rect(tree, col, 180.0, 20.0, 4.0, shimmer);
        skeleton_rect(tree, col, 280.0, 10.0, 4.0, shimmer);
        skeleton_rect(tree, col, 260.0, 10.0, 4.0, shimmer);
        skeleton_rect(tree, col, 220.0, 10.0, 4.0, shimmer);
    }

    // ── Avatar + text (card pattern) ──
    label(tree, root, "Card Pattern", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 12.0, align: Align::Start, ..UiStyle::default()
        });
        tree.add_child(root, row);

        // Avatar circle
        skeleton_rect(tree, row, 40.0, 40.0, 20.0, shimmer);

        let lines = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 6.0, ..UiStyle::default()
        });
        tree.add_child(row, lines);

        skeleton_rect(tree, lines, 120.0, 12.0, 4.0, shimmer);
        skeleton_rect(tree, lines, 180.0, 10.0, 4.0, shimmer);
    }

    // ── Image placeholder ──
    label(tree, root, "Image Placeholder", text_secondary);
    skeleton_rect(tree, root, 280.0, 120.0, 8.0, shimmer);

    // ── List items ──
    label(tree, root, "List Items", text_secondary);
    {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0, ..UiStyle::default()
        });
        tree.add_child(root, col);

        for _ in 0..3 {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 10.0, align: Align::Center, ..UiStyle::default()
            });
            tree.add_child(col, row);

            skeleton_rect(tree, row, 32.0, 32.0, 4.0, shimmer);

            let lines = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column, gap: 4.0, ..UiStyle::default()
            });
            tree.add_child(row, lines);

            skeleton_rect(tree, lines, 140.0, 10.0, 3.0, shimmer);
            skeleton_rect(tree, lines, 100.0, 8.0, 3.0, shimmer);
        }
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn skeleton_rect(tree: &mut UiTree, parent: UiNodeId, width: f32, height: f32, radius: f32, color: glam::Vec4) {
    let rect = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(width),
        height: Sizing::Fixed(height),
        corner_radii: [radius; 4],
        background: Some(color),
        ..UiStyle::default()
    });
    tree.add_child(parent, rect);
}
