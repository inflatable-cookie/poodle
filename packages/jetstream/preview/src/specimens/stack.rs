//! Stack specimen — demonstrates direction, gap, and alignment variants.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

/// Render the Stack specimen.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Vertical stack with gap variants ──
    section_label(tree, root, "Vertical Stack — Gap Variants", text_secondary);

    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });
    tree.add_child(root, row);

    for &(label, gap) in &[("gap: 0", 0.0), ("gap: 4", 4.0), ("gap: 8", 8.0), ("gap: 16", 16.0)] {
        let stack = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            gap,
            padding: Edges::all(8.0),
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radius: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(row, stack);

        // Title
        let title = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(10.0),
            ..UiStyle::default()
        });
        tree.add_child(stack, title);

        for _ in 0..3 {
            let item = swatch(tree, 48.0, 20.0, accent);
            tree.add_child(stack, item);
        }
    }

    // ── Horizontal stack ──
    section_label(tree, root, "Horizontal Stack", text_secondary);

    let h_stack = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        gap: 8.0,
        padding: Edges::all(8.0),
        background: Some(bg_surface),
        border_color: Some(border),
        border_width: 1.0,
        corner_radius: 6.0,
        ..UiStyle::default()
    });
    tree.add_child(root, h_stack);

    for _ in 0..5 {
        let item = swatch(tree, 48.0, 28.0, accent);
        tree.add_child(h_stack, item);
    }

    // ── Alignment variants ──
    section_label(tree, root, "Alignment Variants (vertical stack, 120px tall)", text_secondary);

    let align_row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        gap: 12.0,
        ..UiStyle::default()
    });
    tree.add_child(root, align_row);

    let aligns: &[(&str, Align)] = &[
        ("Start", Align::Start),
        ("Center", Align::Center),
        ("End", Align::End),
    ];

    for &(name, align) in aligns {
        let col = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(100.0),
            height: Sizing::Fixed(120.0),
            gap: 4.0,
            padding: Edges::all(8.0),
            align,
            background: Some(bg_surface),
            border_color: Some(border),
            border_width: 1.0,
            corner_radius: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(align_row, col);

        let lbl = tree.create(Widget::Label { text: name.to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(10.0),
            ..UiStyle::default()
        });
        tree.add_child(col, lbl);

        let s1 = swatch(tree, 60.0, 16.0, accent);
        let s2 = swatch(tree, 40.0, 16.0, accent);
        let s3 = swatch(tree, 50.0, 16.0, accent);
        tree.add_child(col, s1);
        tree.add_child(col, s2);
        tree.add_child(col, s3);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn swatch(tree: &mut UiTree, w: f32, h: f32, color: glam::Vec4) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(w),
        height: Sizing::Fixed(h),
        background: Some(theme_bridge::tint(color, 0.25)),
        corner_radius: 3.0,
        ..UiStyle::default()
    })
}
