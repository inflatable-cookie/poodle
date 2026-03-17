//! RelationPicker specimen — searchable related entity picker.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── With selected entities ──
    section_label(tree, root, "With Selected Relations", text_secondary);
    {
        let wrapper = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0,
            width: Sizing::Fixed(320.0),
            ..UiStyle::default()
        });
        tree.add_child(root, wrapper);

        // Label
        let label = tree.create(Widget::Label { text: "Related Projects".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(wrapper, label);

        // Selected tags
        let tags = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(wrapper, tags);

        for &name in &["Alpha Project", "Beta Launch"] {
            let tag = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 4.0,
                height: Sizing::Fixed(24.0),
                padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
                background: Some(theme_bridge::tint(accent, 0.12)),
                corner_radii: [4.0; 4], align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(tags, tag);

            let n = tree.create(Widget::Label { text: name.to_string() }, UiStyle {
                text_color: Some(accent), text_size: Some(11.0), ..UiStyle::default()
            });
            tree.add_child(tag, n);

            let x = tree.create(Widget::Label { text: "✕".to_string() }, UiStyle {
                text_color: Some(theme_bridge::tint(accent, 0.6)), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(tag, x);
        }

        // Search input
        let search = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0,
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4], align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(wrapper, search);

        let search_lbl = tree.create(Widget::Label { text: "🔍 Search projects…".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(search, search_lbl);

        // Dropdown results
        let dropdown = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4],
            ..UiStyle::default()
        });
        tree.add_child(wrapper, dropdown);

        for &(name, desc) in &[
            ("Gamma Initiative", "Marketing · 3 members"),
            ("Delta Core", "Engineering · 8 members"),
            ("Epsilon Research", "Research · 2 members"),
        ] {
            let item = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column,
                padding: Edges { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 },
                gap: 2.0,
                ..UiStyle::default()
            });
            tree.add_child(dropdown, item);

            let n = tree.create(Widget::Label { text: name.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
            });
            tree.add_child(item, n);

            let d = tree.create(Widget::Label { text: desc.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(item, d);
        }
    }

    // ── Empty ──
    section_label(tree, root, "Empty Picker", text_secondary);
    {
        let wrapper = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 8.0,
            width: Sizing::Fixed(320.0),
            ..UiStyle::default()
        });
        tree.add_child(root, wrapper);

        let label = tree.create(Widget::Label { text: "Assigned To".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(wrapper, label);

        let search = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 6.0,
            height: Sizing::Fixed(32.0),
            padding: Edges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 },
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radii: [6.0; 4], align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(wrapper, search);

        let search_lbl = tree.create(Widget::Label { text: "🔍 Search people…".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(search, search_lbl);
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
