//! ConfirmAction specimen — confirmation dialog with destructive and safe variants.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 24.0,
        ..UiStyle::default()
    });

    // ── Destructive ──
    section_label(tree, root, "Destructive Confirmation", text_secondary);
    {
        let dialog = dialog_frame(tree, bg_elevated, border, 340.0);
        tree.add_child(root, dialog);

        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges { top: 20.0, right: 20.0, bottom: 16.0, left: 20.0 },
            gap: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(dialog, body);

        let icon = tree.create(Widget::Label { text: "⚠".to_string() }, UiStyle {
            text_color: Some(danger), text_size: Some(24.0), ..UiStyle::default()
        });
        tree.add_child(body, icon);

        let title = tree.create(Widget::Label { text: "Delete 3 items?".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(15.0), ..UiStyle::default()
        });
        tree.add_child(body, title);

        let desc = tree.create(Widget::Label { text: "This action cannot be undone. The selected items will be permanently removed.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(body, desc);

        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(dialog, sep);

        let footer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 12.0, right: 20.0, bottom: 16.0, left: 20.0 },
            gap: 8.0, justify: Justify::End,
            ..UiStyle::default()
        });
        tree.add_child(dialog, footer);

        let cancel = tree.create(Widget::Button {
            label: "Cancel".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 },
            corner_radius: 6.0, border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(footer, cancel);

        let confirm = tree.create(Widget::Button {
            label: "Delete".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 },
            corner_radius: 6.0, background: Some(danger),
            text_color: Some(text_inverse), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(footer, confirm);
    }

    // ── Warning ──
    section_label(tree, root, "Warning Confirmation", text_secondary);
    {
        let dialog = dialog_frame(tree, bg_elevated, border, 340.0);
        tree.add_child(root, dialog);

        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges { top: 20.0, right: 20.0, bottom: 16.0, left: 20.0 },
            gap: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(dialog, body);

        let icon = tree.create(Widget::Label { text: "⚠".to_string() }, UiStyle {
            text_color: Some(warning), text_size: Some(24.0), ..UiStyle::default()
        });
        tree.add_child(body, icon);

        let title = tree.create(Widget::Label { text: "Unsaved Changes".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(15.0), ..UiStyle::default()
        });
        tree.add_child(body, title);

        let desc = tree.create(Widget::Label { text: "You have unsaved changes. Do you want to save before leaving?".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(body, desc);

        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(dialog, sep);

        let footer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 12.0, right: 20.0, bottom: 16.0, left: 20.0 },
            gap: 8.0, justify: Justify::End,
            ..UiStyle::default()
        });
        tree.add_child(dialog, footer);

        let discard = tree.create(Widget::Button {
            label: "Discard".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 },
            corner_radius: 6.0, border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(footer, discard);

        let save = tree.create(Widget::Button {
            label: "Save".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 },
            corner_radius: 6.0, background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(footer, save);
    }

    // ── Safe / Informational ──
    section_label(tree, root, "Informational Confirmation", text_secondary);
    {
        let dialog = dialog_frame(tree, bg_elevated, border, 340.0);
        tree.add_child(root, dialog);

        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges { top: 20.0, right: 20.0, bottom: 16.0, left: 20.0 },
            gap: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(dialog, body);

        let title = tree.create(Widget::Label { text: "Publish Article?".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(15.0), ..UiStyle::default()
        });
        tree.add_child(body, title);

        let desc = tree.create(Widget::Label { text: "This will make the article visible to all users.".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(body, desc);

        let sep = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(dialog, sep);

        let footer = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 12.0, right: 20.0, bottom: 16.0, left: 20.0 },
            gap: 8.0, justify: Justify::End,
            ..UiStyle::default()
        });
        tree.add_child(dialog, footer);

        let cancel = tree.create(Widget::Button {
            label: "Cancel".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 },
            corner_radius: 6.0, border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(footer, cancel);

        let publish = tree.create(Widget::Button {
            label: "Publish".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 },
            corner_radius: 6.0, background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(12.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(footer, publish);
    }

    root
}

fn dialog_frame(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4, width: f32) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(width),
        background: Some(bg),
        border_color: Some(border), border_width: 1.0,
        corner_radius: 10.0,
        ..UiStyle::default()
    })
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
