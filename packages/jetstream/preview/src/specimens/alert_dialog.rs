//! AlertDialog specimen — focused confirmation modal with destructive/info variants.

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
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");

    let root = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_gap(20.0),
        ),
        NodeStyle::default(),
    );

    // ── Destructive alert ──
    section_label(tree, root, "Destructive Alert", text_secondary);
    {
        let card = alert_card(tree, 300.0, bg_elevated, border);
        tree.add_child(root, card);

        let body = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_padding(LayoutEdges { top: 20.0, right: 20.0, bottom: 12.0, left: 20.0 })
                    .with_gap(8.0),
            ),
            NodeStyle::default(),
        );
        tree.add_child(card, body);

        // Icon
        let icon = tree.create_node(
            Widget::Label { text: "⚠".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(danger), text_size: Some(20.0), ..NodeStyle::default() },
        );
        tree.add_child(body, icon);

        let title = tree.create_node(
            Widget::Label { text: "Delete Account?".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(16.0), ..NodeStyle::default() },
        );
        tree.add_child(body, title);

        let msg = tree.create_node(
            Widget::Label { text: "This will permanently delete your account and all data. This action cannot be undone.".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() },
        );
        tree.add_child(body, msg);

        let actions = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_width(LayoutSizing::Grow)
                    .with_padding(LayoutEdges { top: 8.0, right: 20.0, bottom: 16.0, left: 20.0 })
                    .with_gap(8.0)
                    .with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Stretch),
            ),
            NodeStyle::default(),
        );
        tree.add_child(card, actions);

        btn(tree, actions, "Cancel", bg_surface, Some(border), text_primary);
        btn(tree, actions, "Delete", danger, None, text_inverse);
    }

    // ── Warning alert ──
    section_label(tree, root, "Warning Alert", text_secondary);
    {
        let card = alert_card(tree, 300.0, bg_elevated, border);
        tree.add_child(root, card);

        let body = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_padding(LayoutEdges { top: 20.0, right: 20.0, bottom: 12.0, left: 20.0 })
                    .with_gap(8.0),
            ),
            NodeStyle::default(),
        );
        tree.add_child(card, body);

        let icon = tree.create_node(
            Widget::Label { text: "⚡".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(warning), text_size: Some(20.0), ..NodeStyle::default() },
        );
        tree.add_child(body, icon);

        let title = tree.create_node(
            Widget::Label { text: "Unsaved Changes".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(16.0), ..NodeStyle::default() },
        );
        tree.add_child(body, title);

        let msg = tree.create_node(
            Widget::Label { text: "You have unsaved changes. Do you want to save before leaving?".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() },
        );
        tree.add_child(body, msg);

        let actions = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_width(LayoutSizing::Grow)
                    .with_padding(LayoutEdges { top: 8.0, right: 20.0, bottom: 16.0, left: 20.0 })
                    .with_gap(8.0)
                    .with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Stretch),
            ),
            NodeStyle::default(),
        );
        tree.add_child(card, actions);

        btn(tree, actions, "Discard", bg_surface, Some(border), text_primary);
        btn(tree, actions, "Save", accent, None, text_inverse);
    }

    // ── Info alert ──
    section_label(tree, root, "Informational Alert", text_secondary);
    {
        let card = alert_card(tree, 280.0, bg_elevated, border);
        tree.add_child(root, card);

        let body = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_padding(LayoutEdges::uniform(20.0))
                    .with_gap(8.0)
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
            ),
            NodeStyle::default(),
        );
        tree.add_child(card, body);

        let icon = tree.create_node(
            Widget::Label { text: "ℹ".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(20.0), ..NodeStyle::default() },
        );
        tree.add_child(body, icon);

        let title = tree.create_node(
            Widget::Label { text: "Update Available".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(16.0), ..NodeStyle::default() },
        );
        tree.add_child(body, title);

        let msg = tree.create_node(
            Widget::Label { text: "A new version is ready to install.".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() },
        );
        tree.add_child(body, msg);

        btn(tree, body, "Update Now", accent, None, text_inverse);
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

fn alert_card(tree: &mut UiTree, width: f32, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Fixed(width)),
        ),
        NodeStyle {
            background: Some(bg),
            border_color: Some(border),
            border_width: 1.0,
            corner_radii: [12.0; 4],
            ..NodeStyle::default()
        },
    )
}

fn btn(tree: &mut UiTree, parent: UiNodeId, text: &str, bg: glam::Vec4, border: Option<glam::Vec4>, fg: glam::Vec4) {
    let btn = tree.create_node(
        Widget::Button {
            label: text.to_string(), pressed: false, hovered: false,
        },
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(32.0))
                .with_padding(LayoutEdges { top: 0.0, right: 16.0, bottom: 0.0, left: 16.0 })
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center),
        ),
        NodeStyle {
            corner_radii: [6.0; 4], background: Some(bg),
            border_color: border, border_width: if border.is_some() { 1.0 } else { 0.0 },
            text_color: Some(fg), text_size: Some(12.0),
            focusable: true, ..NodeStyle::default()
        },
    );
    tree.add_child(parent, btn);
}
