//! RadioGroup specimen — single-selection group with selected and disabled states.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let border = theme_bridge::border_default(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    label(tree, root, "Radio Group (second selected)", text_secondary);

    let options = ["Option A", "Option B (selected)", "Option C", "Option D"];
    for (i, &opt) in options.iter().enumerate() {
        let selected = i == 1;
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(root, row);

        let radio = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(18.0),
            height: Sizing::Fixed(18.0),
            corner_radius: 9.0,
            border_color: Some(if selected { accent } else { border }),
            border_width: if selected { 5.0 } else { 1.5 },
            ..UiStyle::default()
        });
        tree.add_child(row, radio);

        let lbl = tree.create(Widget::Label { text: opt.to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(row, lbl);
    }

    // ── Disabled ──
    label(tree, root, "Disabled Group", text_secondary);

    for (i, &opt) in ["Enabled", "Disabled selected"].iter().enumerate() {
        let selected = i == 1;
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 8.0,
            align: Align::Center,
            opacity: 0.5,
            ..UiStyle::default()
        });
        tree.add_child(root, row);

        let radio = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(18.0),
            height: Sizing::Fixed(18.0),
            corner_radius: 9.0,
            border_color: Some(if selected { accent } else { border }),
            border_width: if selected { 5.0 } else { 1.5 },
            ..UiStyle::default()
        });
        tree.add_child(row, radio);

        let lbl = tree.create(Widget::Label { text: opt.to_string() }, UiStyle {
            text_color: Some(text_primary),
            text_size: Some(12.0),
            ..UiStyle::default()
        });
        tree.add_child(row, lbl);
    }

    root
}

fn label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color),
        text_size: Some(11.0),
        ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
