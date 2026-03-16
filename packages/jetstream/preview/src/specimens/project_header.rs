//! ProjectHeader specimen — project name, branch badge, and actions.

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
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 16.0, ..UiStyle::default()
    });

    section_label(tree, root, "Standard Project Header", text_secondary);
    {
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Fixed(480.0), height: Sizing::Fixed(44.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0, gap: 10.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, header);

        let name = tree.create(Widget::Label { text: "Acme Project".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(header, name);

        // Branch badge
        let branch = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 4.0,
            height: Sizing::Fixed(22.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
            background: Some(theme_bridge::tint(success, 0.12)),
            corner_radius: 4.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(header, branch);

        let branch_icon = tree.create(Widget::Label { text: "⎇".to_string() }, UiStyle {
            text_color: Some(success), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(branch, branch_icon);

        let branch_name = tree.create(Widget::Label { text: "main".to_string() }, UiStyle {
            text_color: Some(success), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(branch, branch_name);

        let spacer = tree.create(Widget::Panel, UiStyle { width: Sizing::Grow(1.0), ..UiStyle::default() });
        tree.add_child(header, spacer);

        let run = tree.create(Widget::Button {
            label: "▶ Run".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(26.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            corner_radius: 6.0, background: Some(accent),
            text_color: Some(text_inverse), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(header, run);

        let share = tree.create(Widget::Button {
            label: "Share".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(26.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            corner_radius: 6.0, border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(header, share);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
