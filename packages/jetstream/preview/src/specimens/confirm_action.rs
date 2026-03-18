//! ConfirmAction specimen — confirmation dialog with destructive and safe variants.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

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

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_width(LayoutSizing::Grow).with_gap(24.0)), NodeStyle::default());

    // ── Destructive ──
    section_label(tree, root, "Destructive Confirmation", text_secondary);
    {
        let dialog = dialog_frame(tree, bg_elevated, border, 340.0);
        tree.add_child(root, dialog);

        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_padding(LayoutEdges { top: 20.0, right: 20.0, bottom: 16.0, left: 20.0 }).with_gap(8.0)), NodeStyle::default());
        tree.add_child(dialog, body);

        let icon = tree.create_node(Widget::Label { text: "⚠".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(danger), text_size: Some(24.0), ..NodeStyle::default()
        });
        tree.add_child(body, icon);

        let title = tree.create_node(Widget::Label { text: "Delete 3 items?".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_primary), text_size: Some(15.0), ..NodeStyle::default()
        });
        tree.add_child(body, title);

        let desc = tree.create_node(Widget::Label { text: "This action cannot be undone. The selected items will be permanently removed.".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(body, desc);

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(1.0))), NodeStyle {
            background: Some(border), ..NodeStyle::default()
        });
        tree.add_child(dialog, sep);

        let footer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_padding(LayoutEdges { top: 12.0, right: 20.0, bottom: 16.0, left: 20.0 }).with_gap(8.0).with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Stretch)), NodeStyle::default());
        tree.add_child(dialog, footer);

        let cancel = tree.create_node(Widget::Button {
            label: "Cancel".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new().with_height(LayoutSizing::Fixed(32.0)).with_padding(LayoutEdges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 }).with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(12.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(footer, cancel);

        let confirm = tree.create_node(Widget::Button {
            label: "Delete".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new().with_height(LayoutSizing::Fixed(32.0)).with_padding(LayoutEdges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 }).with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], background: Some(danger),
            text_color: Some(text_inverse), text_size: Some(12.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(footer, confirm);
    }

    // ── Warning ──
    section_label(tree, root, "Warning Confirmation", text_secondary);
    {
        let dialog = dialog_frame(tree, bg_elevated, border, 340.0);
        tree.add_child(root, dialog);

        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_padding(LayoutEdges { top: 20.0, right: 20.0, bottom: 16.0, left: 20.0 }).with_gap(8.0)), NodeStyle::default());
        tree.add_child(dialog, body);

        let icon = tree.create_node(Widget::Label { text: "⚠".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(warning), text_size: Some(24.0), ..NodeStyle::default()
        });
        tree.add_child(body, icon);

        let title = tree.create_node(Widget::Label { text: "Unsaved Changes".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_primary), text_size: Some(15.0), ..NodeStyle::default()
        });
        tree.add_child(body, title);

        let desc = tree.create_node(Widget::Label { text: "You have unsaved changes. Do you want to save before leaving?".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(body, desc);

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(1.0))), NodeStyle {
            background: Some(border), ..NodeStyle::default()
        });
        tree.add_child(dialog, sep);

        let footer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_padding(LayoutEdges { top: 12.0, right: 20.0, bottom: 16.0, left: 20.0 }).with_gap(8.0).with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Stretch)), NodeStyle::default());
        tree.add_child(dialog, footer);

        let discard = tree.create_node(Widget::Button {
            label: "Discard".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new().with_height(LayoutSizing::Fixed(32.0)).with_padding(LayoutEdges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 }).with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(12.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(footer, discard);

        let save = tree.create_node(Widget::Button {
            label: "Save".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new().with_height(LayoutSizing::Fixed(32.0)).with_padding(LayoutEdges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 }).with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(12.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(footer, save);
    }

    // ── Safe / Informational ──
    section_label(tree, root, "Informational Confirmation", text_secondary);
    {
        let dialog = dialog_frame(tree, bg_elevated, border, 340.0);
        tree.add_child(root, dialog);

        let body = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_padding(LayoutEdges { top: 20.0, right: 20.0, bottom: 16.0, left: 20.0 }).with_gap(8.0)), NodeStyle::default());
        tree.add_child(dialog, body);

        let title = tree.create_node(Widget::Label { text: "Publish Article?".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_primary), text_size: Some(15.0), ..NodeStyle::default()
        });
        tree.add_child(body, title);

        let desc = tree.create_node(Widget::Label { text: "This will make the article visible to all users.".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
            text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default()
        });
        tree.add_child(body, desc);

        let sep = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_width(LayoutSizing::Grow).with_height(LayoutSizing::Fixed(1.0))), NodeStyle {
            background: Some(border), ..NodeStyle::default()
        });
        tree.add_child(dialog, sep);

        let footer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Row).with_padding(LayoutEdges { top: 12.0, right: 20.0, bottom: 16.0, left: 20.0 }).with_gap(8.0).with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Stretch)), NodeStyle::default());
        tree.add_child(dialog, footer);

        let cancel = tree.create_node(Widget::Button {
            label: "Cancel".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new().with_height(LayoutSizing::Fixed(32.0)).with_padding(LayoutEdges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 }).with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(12.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(footer, cancel);

        let publish = tree.create_node(Widget::Button {
            label: "Publish".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new().with_height(LayoutSizing::Fixed(32.0)).with_padding(LayoutEdges { top: 0.0, right: 14.0, bottom: 0.0, left: 14.0 }).with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)), NodeStyle {
            corner_radii: [6.0; 4], background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(12.0),
            focusable: true, ..NodeStyle::default()
        });
        tree.add_child(footer, publish);
    }

    root
}

fn dialog_frame(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4, width: f32) -> UiNodeId {
    tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new().with_direction(LayoutDirection::Column).with_width(LayoutSizing::Fixed(width))), NodeStyle {
        background: Some(bg),
        border_color: Some(border), border_width: 1.0,
        corner_radii: [10.0; 4],
        ..NodeStyle::default()
    })
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()), NodeStyle {
        text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default()
    });
    tree.add_child(parent, lbl);
}
