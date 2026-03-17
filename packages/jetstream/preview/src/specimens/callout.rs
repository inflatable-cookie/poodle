//! CallOut specimen — demonstrates tone variants with title and content.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

/// Render the CallOut specimen.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 12.0,
        ..UiStyle::default()
    });

    let label = tree.create(Widget::Label {
        text: "Tone Variants".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(root, label);

    let tones: &[(&str, &str, &str, &str)] = &[
        ("Info", "Getting Started", "Review the documentation before making changes.", "semantic.color.accent.base"),
        ("Success", "Saved", "Your changes have been saved successfully.", "semantic.color.status.success"),
        ("Warning", "Deprecation Notice", "This API will be removed in the next release.", "semantic.color.status.warning"),
        ("Danger", "Breaking Change", "This update requires a database migration.", "semantic.color.status.danger"),
    ];

    for &(tone_name, title, content, color_token) in tones {
        let tone_color = theme_bridge::resolve_vec4(theme, color_token);

        let callout = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0),
            ..UiStyle::default()
        });
        tree.add_child(root, callout);

        // Left accent bar
        let bar = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(3.0),
            height: Sizing::Grow(1.0),
            background: Some(tone_color),
            corner_radii: [1.5; 4],
            ..UiStyle::default()
        });
        tree.add_child(callout, bar);

        // Content area
        let body = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Grow(1.0),
            padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
            gap: 4.0,
            background: Some(theme_bridge::tint(tone_color, 0.06)),
            ..UiStyle::default()
        });
        tree.add_child(callout, body);

        // Title row
        let title_row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(body, title_row);

        let title_label = tree.create(Widget::Label {
            text: title.to_string(),
        }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(title_row, title_label);

        let badge = tree.create(Widget::Label {
            text: tone_name.to_string(),
        }, UiStyle {
            text_color: Some(tone_color),
            text_size: Some(9.0),
            ..UiStyle::default()
        });
        tree.add_child(title_row, badge);

        // Content text
        let content_label = tree.create(Widget::Label {
            text: content.to_string(),
        }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(11.0),
            ..UiStyle::default()
        });
        tree.add_child(body, content_label);
    }

    root
}
