//! InlineEditableField specimen — click-to-edit text field with display/edit toggle.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 16.0, ..UiStyle::default()
    });

    // ── Display mode ──
    section_label(tree, root, "Display Mode (click to edit)", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0, align: Align::Center,
            width: Sizing::Fixed(300.0),
            padding: Edges { top: 6.0, right: 8.0, bottom: 6.0, left: 8.0 },
            corner_radii: [4.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(root, row);

        let value = tree.create(Widget::Label { text: "Acme Project".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(row, value);

        let edit_icon = tree.create(Widget::Label { text: "✎".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(row, edit_icon);
    }

    // ── Edit mode ──
    section_label(tree, root, "Edit Mode (active)", text_secondary);
    {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0, align: Align::Center,
            width: Sizing::Fixed(300.0),
            ..UiStyle::default()
        });
        tree.add_child(root, row);

        let input = tree.create(Widget::Panel, UiStyle {
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_surface), border_color: Some(accent), border_width: 2.0,
            corner_radii: [6.0; 4], align: Align::Center,
            width: Sizing::Grow(1.0),
            ..UiStyle::default()
        });
        tree.add_child(row, input);

        let txt = tree.create(Widget::Label { text: "Acme Project".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(input, txt);

        let save = tree.create(Widget::Button {
            label: "✓".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(28.0), height: Sizing::Fixed(28.0),
            corner_radii: [6.0; 4], background: Some(accent),
            text_color: Some(glam::Vec4::ONE), text_size: Some(14.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(row, save);

        let cancel = tree.create(Widget::Button {
            label: "✕".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            width: Sizing::Fixed(28.0), height: Sizing::Fixed(28.0),
            corner_radii: [6.0; 4], border_color: Some(border), border_width: 1.0,
            text_color: Some(text_secondary), text_size: Some(14.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(row, cancel);
    }

    // ── In context ──
    section_label(tree, root, "In Context (label + value)", text_secondary);
    {
        let ctx = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, ctx);

        for &(label, value) in &[("Project Name", "Acme Project"), ("Description", "A brief project overview"), ("Status", "Active")] {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 12.0, align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(ctx, row);

            let l = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(11.0),
                width: Sizing::Fixed(100.0), ..UiStyle::default()
            });
            tree.add_child(row, l);

            let v = tree.create(Widget::Label { text: value.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
            });
            tree.add_child(row, v);

            let edit = tree.create(Widget::Label { text: "✎".to_string() }, UiStyle {
                text_color: Some(theme_bridge::tint(text_secondary, 0.5)), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(row, edit);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
