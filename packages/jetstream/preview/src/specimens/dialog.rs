//! Dialog specimen — modal overlay with header, body, and actions.
//!
//! Since Jetstream preview renders a static UiTree, we show the dialog
//! as a card-based mockup rather than a true overlay.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Standard dialog ──
    section_label(tree, root, "Standard Dialog", text_secondary);
    {
        let dialog = dialog_frame(tree, 320.0, bg_elevated, border);
        tree.add_child(root, dialog);

        // Header
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0),
            padding: Edges { top: 16.0, right: 20.0, bottom: 12.0, left: 20.0 },
            justify: Justify::SpaceBetween,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(dialog, header);

        let title = tree.create(Widget::Label { text: "Confirm Action".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(16.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        let close = tree.create(Widget::Label { text: "✕".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(header, close);

        // Separator
        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(dialog, sep);

        // Body
        let body = tree.create(Widget::Panel, UiStyle {
            padding: Edges { top: 12.0, right: 20.0, bottom: 12.0, left: 20.0 },
            ..UiStyle::default()
        });
        tree.add_child(dialog, body);

        let msg = tree.create(Widget::Label { text: "Are you sure you want to proceed? This action cannot be undone.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(body, msg);

        // Footer
        let footer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0),
            padding: Edges { top: 12.0, right: 20.0, bottom: 16.0, left: 20.0 },
            gap: 8.0,
            justify: Justify::End,
            ..UiStyle::default()
        });
        tree.add_child(dialog, footer);

        btn_secondary(tree, footer, "Cancel", bg_surface, border, text_primary);
        btn_primary(tree, footer, "Confirm", accent, text_inverse);
    }

    // ── Dialog with form ──
    section_label(tree, root, "Dialog with Form Content", text_secondary);
    {
        let dialog = dialog_frame(tree, 360.0, bg_elevated, border);
        tree.add_child(root, dialog);

        let header = tree.create(Widget::Panel, UiStyle {
            padding: Edges { top: 16.0, right: 20.0, bottom: 12.0, left: 20.0 },
            ..UiStyle::default()
        });
        tree.add_child(dialog, header);

        let title = tree.create(Widget::Label { text: "Create New Item".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(16.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(dialog, sep);

        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges { top: 12.0, right: 20.0, bottom: 12.0, left: 20.0 },
            gap: 12.0,
            ..UiStyle::default()
        });
        tree.add_child(dialog, body);

        let bg_canvas = theme_bridge::canvas_background(theme);
        let border_default = theme_bridge::border_default(theme);

        for &(lbl, placeholder) in &[("Name", "Enter name"), ("Description", "Optional description")] {
            let field = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column, gap: 4.0, ..UiStyle::default()
            });
            tree.add_child(body, field);

            let l = tree.create(Widget::Label { text: lbl.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
            });
            tree.add_child(field, l);

            let input = tree.create(Widget::Panel, UiStyle {
                width: Sizing::Grow(1.0), height: Sizing::Fixed(32.0),
                padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
                background: Some(bg_canvas), border_color: Some(border_default),
                border_width: 1.0, corner_radius: 6.0, align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(field, input);

            let ph = tree.create(Widget::Label { text: placeholder.to_string() }, UiStyle {
                text_color: Some(theme_bridge::tint(text_secondary, 0.6)),
                text_size: Some(12.0), ..UiStyle::default()
            });
            tree.add_child(input, ph);
        }

        let sep2 = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(dialog, sep2);

        let footer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0),
            padding: Edges { top: 12.0, right: 20.0, bottom: 16.0, left: 20.0 },
            gap: 8.0,
            justify: Justify::End,
            ..UiStyle::default()
        });
        tree.add_child(dialog, footer);

        btn_secondary(tree, footer, "Cancel", bg_surface, border, text_primary);
        btn_primary(tree, footer, "Create", accent, text_inverse);
    }

    // ── Compact / small ──
    section_label(tree, root, "Small Dialog", text_secondary);
    {
        let dialog = dialog_frame(tree, 260.0, bg_elevated, border);
        tree.add_child(root, dialog);

        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges::all(20.0),
            gap: 12.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(dialog, body);

        let msg = tree.create(Widget::Label { text: "Delete this item?".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(body, msg);

        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0, ..UiStyle::default()
        });
        tree.add_child(body, row);

        let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");
        btn_secondary(tree, row, "Cancel", bg_surface, border, text_primary);
        btn_primary(tree, row, "Delete", danger, text_inverse);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn dialog_frame(tree: &mut UiTree, width: f32, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(width),
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        corner_radius: 12.0,
        ..UiStyle::default()
    })
}

fn btn_primary(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, fg: glam::Vec4) {
    let btn = tree.create(Widget::Button {
        label: text.to_string(), pressed: false, hovered: false,
    }, UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
        corner_radius: 6.0, background: Some(bg), text_color: Some(fg),
        text_size: Some(12.0), align: Align::Center, justify: Justify::Center,
        focusable: true, ..UiStyle::default()
    });
    tree.add_child(parent, btn);
}

fn btn_secondary(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: glam::Vec4, fg: glam::Vec4) {
    let btn = tree.create(Widget::Button {
        label: text.to_string(), pressed: false, hovered: false,
    }, UiStyle {
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 },
        corner_radius: 6.0, background: Some(bg), border_color: Some(border),
        border_width: 1.0, text_color: Some(fg), text_size: Some(12.0),
        align: Align::Center, justify: Justify::Center,
        focusable: true, ..UiStyle::default()
    });
    tree.add_child(parent, btn);
}
