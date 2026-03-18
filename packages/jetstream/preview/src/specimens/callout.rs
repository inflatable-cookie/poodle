//! CallOut specimen — demonstrates tone variants with title and content.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::theme_bridge;

/// Render the CallOut specimen.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);

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
            text: "Tone Variants".to_string(),
        },
        pug_jetstream::map_layout(&LayoutIntent::new()),
        NodeStyle {
            text_color: Some(text_secondary),
            text_size: Some(11.0),
            ..NodeStyle::default()
        },
    );
    tree.add_child(root, label);

    let tones: &[(&str, &str, &str, &str)] = &[
        ("Info", "Getting Started", "Review the documentation before making changes.", "semantic.color.accent.base"),
        ("Success", "Saved", "Your changes have been saved successfully.", "semantic.color.status.success"),
        ("Warning", "Deprecation Notice", "This API will be removed in the next release.", "semantic.color.status.warning"),
        ("Danger", "Breaking Change", "This update requires a database migration.", "semantic.color.status.danger"),
    ];

    for &(tone_name, title, content, color_token) in tones {
        let tone_color = theme_bridge::resolve_vec4(theme, color_token);

        let callout = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_width(LayoutSizing::Grow),
            ),
            NodeStyle::default(),
        );
        tree.add_child(root, callout);

        // Left accent bar
        let bar = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_width(LayoutSizing::Fixed(3.0))
                    .with_height(LayoutSizing::Grow),
            ),
            NodeStyle {
                background: Some(tone_color),
                corner_radii: [1.5; 4],
                ..NodeStyle::default()
            },
        );
        tree.add_child(callout, bar);

        // Content area
        let body = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Grow)
                    .with_padding(LayoutEdges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 })
                    .with_gap(4.0),
            ),
            NodeStyle {
                background: Some(theme_bridge::tint(tone_color, 0.06)),
                ..NodeStyle::default()
            },
        );
        tree.add_child(callout, body);

        // Title row
        let title_row = tree.create_node(
            Widget::Panel,
            pug_jetstream::map_layout(
                &LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_gap(8.0)
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center),
            ),
            NodeStyle::default(),
        );
        tree.add_child(body, title_row);

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

        let badge = tree.create_node(
            Widget::Label {
                text: tone_name.to_string(),
            },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(tone_color),
                text_size: Some(9.0),
                ..NodeStyle::default()
            },
        );
        tree.add_child(title_row, badge);

        // Content text
        let content_label = tree.create_node(
            Widget::Label {
                text: content.to_string(),
            },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(text_secondary),
                text_size: Some(11.0),
                ..NodeStyle::default()
            },
        );
        tree.add_child(body, content_label);
    }

    root
}
