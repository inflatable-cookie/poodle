//! Banner specimen — demonstrates tone variants with title, message, and dismiss.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

/// Render the Banner specimen.
pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let border_subtle = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 12.0,
        ..UiStyle::default()
    });

    let label = tree.create(Widget::Label {
        text: "Status Tones".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
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

        let banner = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Grow(1.0),
            padding: Edges::all(12.0),
            gap: 4.0,
            background: Some(theme_bridge::tint(tone_color, 0.10)),
            border_color: Some(theme_bridge::tint(tone_color, 0.40)),
            border_width: 1.0,
            corner_radius: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(root, banner);

        // Title row with tone label and dismiss
        let title_row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0),
            align: Align::Center,
            justify: Justify::SpaceBetween,
            ..UiStyle::default()
        });
        tree.add_child(banner, title_row);

        let title_label = tree.create(Widget::Label {
            text: title.to_string(),
        }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(title_row, title_label);

        let tone_badge = tree.create(Widget::Label {
            text: tone_name.to_string(),
        }, UiStyle {
            text_color: Some(tone_color),
            text_size: Some(10.0),
            padding: Edges { top: 1.0, right: 6.0, bottom: 1.0, left: 6.0 },
            background: Some(theme_bridge::tint(tone_color, 0.15)),
            corner_radius: 3.0,
            ..UiStyle::default()
        });
        tree.add_child(title_row, tone_badge);

        // Message
        let msg = tree.create(Widget::Label {
            text: message.to_string(),
        }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(11.0),
            ..UiStyle::default()
        });
        tree.add_child(banner, msg);
    }

    // ── With dismiss button ──
    let dismiss_label = tree.create(Widget::Label {
        text: "With Dismiss Action".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(11.0),
        padding: Edges { top: 8.0, right: 0.0, bottom: 0.0, left: 0.0 },
        ..UiStyle::default()
    });
    tree.add_child(root, dismiss_label);

    let accent = theme_bridge::accent_base(theme);
    let dismissible = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        padding: Edges::all(12.0),
        gap: 8.0,
        align: Align::Center,
        justify: Justify::SpaceBetween,
        background: Some(theme_bridge::tint(accent, 0.10)),
        border_color: Some(theme_bridge::tint(accent, 0.40)),
        border_width: 1.0,
        corner_radius: 6.0,
        ..UiStyle::default()
    });
    tree.add_child(root, dismissible);

    let dismiss_text = tree.create(Widget::Label {
        text: "Dismissible banner with action.".to_string(),
    }, UiStyle {
        text_color: Some(text_primary),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(dismissible, dismiss_text);

    let dismiss_btn = tree.create(Widget::Button {
        label: "Dismiss".to_string(),
        pressed: false,
        hovered: false,
    }, UiStyle {
        height: Sizing::Fixed(24.0),
        padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
        corner_radius: 4.0,
        background: Some(theme_bridge::tint(accent, 0.20)),
        text_color: Some(text_primary),
        text_size: Some(10.0),
        border_color: Some(border_subtle),
        border_width: 1.0,
        align: Align::Center,
        justify: Justify::Center,
        ..UiStyle::default()
    });
    tree.add_child(dismissible, dismiss_btn);

    root
}
