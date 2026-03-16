//! Tabs specimen — tabbed interface with underline and pill variants.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Underline tabs ──
    section_label(tree, root, "Underline Tabs", text_secondary);
    {
        let tabs = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 0.0,
            border_color: Some(border), border_width: 1.0,
            ..UiStyle::default()
        });
        tree.add_child(root, tabs);

        for (i, &label) in ["General", "Security", "Notifications", "Billing"].iter().enumerate() {
            let is_active = i == 0;
            let color = if is_active { accent } else { text_secondary };

            let tab = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column,
                padding: Edges { top: 8.0, right: 16.0, bottom: 8.0, left: 16.0 },
                align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(tabs, tab);

            let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
                text_color: Some(color), text_size: Some(12.0), ..UiStyle::default()
            });
            tree.add_child(tab, lbl);

            if is_active {
                let underline = tree.create(Widget::Panel, UiStyle {
                    width: Sizing::Grow(1.0), height: Sizing::Fixed(2.0),
                    background: Some(accent), corner_radius: 1.0,
                    ..UiStyle::default()
                });
                tree.add_child(tab, underline);
            }
        }
    }

    // ── Pill tabs ──
    section_label(tree, root, "Pill Tabs", text_secondary);
    {
        let tabs = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            gap: 4.0,
            padding: Edges::all(4.0),
            background: Some(bg_surface),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, tabs);

        for (i, &label) in ["All", "Active", "Archived"].iter().enumerate() {
            let is_active = i == 0;
            let bg = if is_active { Some(theme_bridge::elevated_background(theme)) } else { None };
            let color = if is_active { text_primary } else { text_secondary };

            let tab = tree.create(Widget::Button {
                label: label.to_string(), pressed: false, hovered: false,
            }, UiStyle {
                height: Sizing::Fixed(28.0),
                padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
                corner_radius: 6.0, background: bg,
                text_color: Some(color), text_size: Some(12.0),
                align: Align::Center, justify: Justify::Center,
                focusable: true, ..UiStyle::default()
            });
            tree.add_child(tabs, tab);
        }
    }

    // ── With count badges ──
    section_label(tree, root, "With Count Badges", text_secondary);
    {
        let tabs = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 0.0,
            border_color: Some(border), border_width: 1.0,
            ..UiStyle::default()
        });
        tree.add_child(root, tabs);

        for &(label, count, active) in &[("Open", "12", true), ("Closed", "48", false), ("All", "60", false)] {
            let color = if active { accent } else { text_secondary };

            let tab = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row, gap: 6.0,
                padding: Edges { top: 8.0, right: 14.0, bottom: 8.0, left: 14.0 },
                align: Align::Center, ..UiStyle::default()
            });
            tree.add_child(tabs, tab);

            let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
                text_color: Some(color), text_size: Some(12.0), ..UiStyle::default()
            });
            tree.add_child(tab, lbl);

            let badge = tree.create(Widget::Label { text: count.to_string() }, UiStyle {
                padding: Edges { top: 1.0, right: 6.0, bottom: 1.0, left: 6.0 },
                corner_radius: 8.0, background: Some(border),
                text_color: Some(text_secondary), text_size: Some(9.0),
                ..UiStyle::default()
            });
            tree.add_child(tab, badge);
        }
    }

    root
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
