//! Badge specimen — status indicators with text, tones, and dot variants.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_gap(16.0),
        ),
        NodeStyle::default(),
    );

    // ── Solid badges ──
    section_label(tree, root, "Solid Badges", text_secondary);
    {
        let row = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_gap(8.0)
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
            ),
            NodeStyle::default(),
        );
        tree.add_child(root, row);

        solid_badge(tree, row, "Default", accent, text_inverse);
        solid_badge(tree, row, "Success", success, text_inverse);
        solid_badge(tree, row, "Warning", warning, text_primary);
        solid_badge(tree, row, "Danger", danger, text_inverse);
    }

    // ── Outlined badges ──
    section_label(tree, root, "Outlined Badges", text_secondary);
    {
        let row = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_gap(8.0)
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
            ),
            NodeStyle::default(),
        );
        tree.add_child(root, row);

        outlined_badge(tree, row, "Default", accent);
        outlined_badge(tree, row, "Success", success);
        outlined_badge(tree, row, "Warning", warning);
        outlined_badge(tree, row, "Danger", danger);
    }

    // ── Dot badges ──
    section_label(tree, root, "Dot Badges (notification count)", text_secondary);
    {
        let row = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_gap(16.0)
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
            ),
            NodeStyle::default(),
        );
        tree.add_child(root, row);

        dot_badge(tree, row, "3", danger, text_inverse);
        dot_badge(tree, row, "12", danger, text_inverse);
        dot_badge(tree, row, "99+", danger, text_inverse);
    }

    // ── With icon placeholder ──
    section_label(tree, root, "With Icon", text_secondary);
    {
        let row = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_gap(8.0)
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
            ),
            NodeStyle::default(),
        );
        tree.add_child(root, row);

        let badge = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_gap(4.0)
                    .with_padding(LayoutEdges { top: 3.0, right: 8.0, bottom: 3.0, left: 6.0 })
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
            ),
            NodeStyle {
                corner_radii: [10.0; 4],
                background: Some(success),
                ..NodeStyle::default()
            },
        );
        tree.add_child(row, badge);

        let icon = tree.create_node(
            Widget::Label { text: "✓".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_inverse), text_size: Some(9.0), ..NodeStyle::default() },
        );
        tree.add_child(badge, icon);

        let lbl = tree.create_node(
            Widget::Label { text: "Verified".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_inverse), text_size: Some(10.0), ..NodeStyle::default() },
        );
        tree.add_child(badge, lbl);

        // Subtle border variant
        let badge2 = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_gap(4.0)
                    .with_padding(LayoutEdges { top: 3.0, right: 8.0, bottom: 3.0, left: 6.0 })
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
            ),
            NodeStyle {
                corner_radii: [10.0; 4],
                border_color: Some(border),
                border_width: 1.0,
                ..NodeStyle::default()
            },
        );
        tree.add_child(row, badge2);

        let icon2 = tree.create_node(
            Widget::Label { text: "⚡".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(9.0), ..NodeStyle::default() },
        );
        tree.add_child(badge2, icon2);

        let lbl2 = tree.create_node(
            Widget::Label { text: "Active".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(10.0), ..NodeStyle::default() },
        );
        tree.add_child(badge2, lbl2);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create_node(
        Widget::Label { text: text.to_string() },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() },
    );
    tree.add_child(parent, lbl);
}

fn solid_badge(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, fg: glam::Vec4) {
    let badge = tree.create_node(
        Widget::Label { text: text.to_string() },
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_padding(LayoutEdges { top: 3.0, right: 8.0, bottom: 3.0, left: 8.0 }),
        ),
        NodeStyle {
            corner_radii: [10.0; 4],
            background: Some(bg),
            text_color: Some(fg),
            text_size: Some(10.0),
            ..NodeStyle::default()
        },
    );
    tree.add_child(parent, badge);
}

fn outlined_badge(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let badge = tree.create_node(
        Widget::Label { text: text.to_string() },
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_padding(LayoutEdges { top: 3.0, right: 8.0, bottom: 3.0, left: 8.0 }),
        ),
        NodeStyle {
            corner_radii: [10.0; 4],
            border_color: Some(color),
            border_width: 1.0,
            text_color: Some(color),
            text_size: Some(10.0),
            ..NodeStyle::default()
        },
    );
    tree.add_child(parent, badge);
}

fn dot_badge(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, fg: glam::Vec4) {
    let badge = tree.create_node(
        Widget::Label { text: text.to_string() },
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_padding(LayoutEdges { top: 2.0, right: 6.0, bottom: 2.0, left: 6.0 })
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center),
        ),
        NodeStyle {
            corner_radii: [12.0; 4],
            background: Some(bg),
            text_color: Some(fg),
            text_size: Some(9.0),
            ..NodeStyle::default()
        },
    );
    tree.add_child(parent, badge);
}
