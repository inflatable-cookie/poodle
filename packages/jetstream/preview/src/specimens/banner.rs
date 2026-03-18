//! Banner specimen — demonstrates tone variants with title, message, and dismiss.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

/// Render the Banner specimen.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let border_subtle = theme_bridge::border_subtle(theme);

    let root = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_gap(12.0),
        ),
        NodeStyle::default(),
    );

    let label = tree.create_node(
        Widget::Label {
            text: "Status Tones".to_string(),
        },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle {
            text_color: Some(text_secondary),
            text_size: Some(11.0),
            ..NodeStyle::default()
        },
    );
    tree.add_child(root, label);

    // Status tone banners
    let tones: &[(&str, &str, &str, &str)] = &[
        ("Info", "Information", "This is an informational banner message.", "semantic.color.accent.base"),
        ("Success", "Operation Complete", "The action was completed successfully.", "semantic.color.status.success"),
        ("Warning", "Attention Needed", "Please review this before proceeding.", "semantic.color.status.warning"),
        ("Danger", "Error Detected", "Something went wrong. Please try again.", "semantic.color.status.danger"),
    ];

    for &(tone_name, title, message, color_token) in tones {
        let tone_color = theme_bridge::resolve_vec4(theme, color_token);

        let banner = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Grow)
                    .with_padding(LayoutEdges::uniform(12.0))
                    .with_gap(4.0),
            ),
            NodeStyle {
                background: Some(theme_bridge::tint(tone_color, 0.10)),
                border_color: Some(theme_bridge::tint(tone_color, 0.40)),
                border_width: 1.0,
                corner_radii: [6.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(root, banner);

        // Title row with tone label and dismiss
        let title_row = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_width(LayoutSizing::Grow)
                    .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center),
            ),
            NodeStyle::default(),
        );
        tree.add_child(banner, title_row);

        let title_label = tree.create_node(
            Widget::Label {
                text: title.to_string(),
            },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(text_primary),
                text_size: Some(12.0),
                ..NodeStyle::default()
            },
        );
        tree.add_child(title_row, title_label);

        let tone_badge = tree.create_node(
            Widget::Label {
                text: tone_name.to_string(),
            },
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_padding(LayoutEdges { top: 1.0, right: 6.0, bottom: 1.0, left: 6.0 }),
            ),
            NodeStyle {
                text_color: Some(tone_color),
                text_size: Some(10.0),
                background: Some(theme_bridge::tint(tone_color, 0.15)),
                corner_radii: [3.0; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(title_row, tone_badge);

        // Message
        let msg = tree.create_node(
            Widget::Label {
                text: message.to_string(),
            },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(text_secondary),
                text_size: Some(11.0),
                ..NodeStyle::default()
            },
        );
        tree.add_child(banner, msg);
    }

    // ── With dismiss button ──
    let dismiss_label = tree.create_node(
        Widget::Label {
            text: "With Dismiss Action".to_string(),
        },
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_padding(LayoutEdges { top: 8.0, right: 0.0, bottom: 0.0, left: 0.0 }),
        ),
        NodeStyle {
            text_color: Some(text_secondary),
            text_size: Some(11.0),
            ..NodeStyle::default()
        },
    );
    tree.add_child(root, dismiss_label);

    let accent = theme_bridge::accent_base(theme);
    let dismissible = tree.create_node(
        Widget::Panel,
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_padding(LayoutEdges::uniform(12.0))
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center),
        ),
        NodeStyle {
            background: Some(theme_bridge::tint(accent, 0.10)),
            border_color: Some(theme_bridge::tint(accent, 0.40)),
            border_width: 1.0,
            corner_radii: [6.0; 4],
            ..NodeStyle::default()
        },
    );
    tree.add_child(root, dismissible);

    let dismiss_text = tree.create_node(
        Widget::Label {
            text: "Dismissible banner with action.".to_string(),
        },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle {
            text_color: Some(text_primary),
            text_size: Some(11.0),
            ..NodeStyle::default()
        },
    );
    tree.add_child(dismissible, dismiss_text);

    let dismiss_btn = tree.create_node(
        Widget::Button {
            label: "Dismiss".to_string(),
            pressed: false,
            hovered: false,
        },
        pug_jetstream::map_layout(
            &LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(24.0))
                .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center),
        ),
        NodeStyle {
            corner_radii: [4.0; 4],
            background: Some(theme_bridge::tint(accent, 0.20)),
            text_color: Some(text_primary),
            text_size: Some(10.0),
            border_color: Some(border_subtle),
            border_width: 1.0,
            ..NodeStyle::default()
        },
    );
    tree.add_child(dismissible, dismiss_btn);

    root
}
