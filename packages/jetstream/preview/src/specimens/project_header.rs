//! ProjectHeader specimen — project name, branch badge, and actions.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{LayoutIntent, LayoutDirection, LayoutSizing, LayoutEdges, CrossAxisAlignment, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");

    let root = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
        .with_direction(LayoutDirection::Column)
        .with_width(LayoutSizing::Grow)
        .with_gap(16.0)),
        NodeStyle::default());

    section_label(tree, root, "Standard Project Header", text_secondary);
    {
        let header = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Fixed(480.0))
            .with_height(LayoutSizing::Fixed(44.0))
            .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
            .with_gap(10.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, header);

        let name = tree.create_node(Widget::Label { text: "Acme Project".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(header, name);

        // Branch badge
        let branch = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_gap(4.0)
            .with_height(LayoutSizing::Fixed(22.0))
            .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(theme_bridge::tint(success, 0.12)),
                corner_radii: [4.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(header, branch);

        let branch_icon = tree.create_node(Widget::Label { text: "\u{2387}".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(success), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(branch, branch_icon);

        let branch_name = tree.create_node(Widget::Label { text: "main".to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(success), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(branch, branch_name);

        let spacer = tree.create_node(Widget::Panel, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_width(LayoutSizing::Grow)),
            NodeStyle::default());
        tree.add_child(header, spacer);

        let run = tree.create_node(Widget::Button {
            label: "\u{25b6} Run".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(26.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle {
                corner_radii: [6.0; 4], background: Some(accent),
                text_color: Some(text_inverse), text_size: Some(11.0),
                focusable: true, ..NodeStyle::default()
            });
        tree.add_child(header, run);

        let share = tree.create_node(Widget::Button {
            label: "Share".to_string(), pressed: false, hovered: false,
        }, pug_jetstream::map_layout(&LayoutIntent::new()
            .with_height(LayoutSizing::Fixed(26.0))
            .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle {
                corner_radii: [6.0; 4], border_color: Some(border), border_width: 1.0,
                text_color: Some(text_primary), text_size: Some(11.0),
                focusable: true, ..NodeStyle::default()
            });
        tree.add_child(header, share);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(Widget::Label { text: text.to_string() }, pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
    tree.add_child(parent, lbl);
}
