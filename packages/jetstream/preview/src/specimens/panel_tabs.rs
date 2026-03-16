//! PanelTabs specimen — tabbed panel views for switching between panel content.

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
        direction: Direction::Column, width: Sizing::Grow(1.0), gap: 20.0, ..UiStyle::default()
    });

    // ── Panel with tab bar ──
    section_label(tree, root, "Panel Tabs", text_secondary);
    {
        let panel = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(340.0), height: Sizing::Fixed(240.0),
            background: Some(bg_surface), border_color: Some(border), border_width: 1.0,
            corner_radius: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(root, panel);

        // Tab bar
        let tab_bar = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            height: Sizing::Fixed(34.0),
            padding: Edges { top: 0.0, right: 4.0, bottom: 0.0, left: 4.0 },
            background: Some(bg_elevated), gap: 0.0, align: Align::End,
            ..UiStyle::default()
        });
        tree.add_child(panel, tab_bar);

        for (i, &label) in ["Properties", "Styles", "Events"].iter().enumerate() {
            let is_active = i == 0;
            let color = if is_active { accent } else { text_secondary };

            let tab = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column,
                padding: Edges { top: 8.0, right: 14.0, bottom: 6.0, left: 14.0 },
                align: Align::Center, gap: 4.0,
                ..UiStyle::default()
            });
            tree.add_child(tab_bar, tab);

            let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
                text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
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

        // Divider
        let div = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
            background: Some(border), ..UiStyle::default()
        });
        tree.add_child(panel, div);

        // Content area (Properties tab active)
        let content = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Grow(1.0), height: Sizing::Grow(1.0),
            padding: Edges::all(12.0), gap: 8.0,
            ..UiStyle::default()
        });
        tree.add_child(panel, content);

        for &(key, val) in &[("id", "btn-submit"), ("label", "Save Changes"), ("variant", "primary"), ("size", "md")] {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row,
                height: Sizing::Fixed(22.0), gap: 8.0, align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(content, row);

            let k = tree.create(Widget::Label { text: key.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0),
                width: Sizing::Fixed(60.0), ..UiStyle::default()
            });
            tree.add_child(row, k);

            let v_bg = tree.create(Widget::Panel, UiStyle {
                height: Sizing::Fixed(22.0),
                padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
                background: Some(bg_elevated), border_color: Some(border), border_width: 1.0,
                corner_radius: 4.0, align: Align::Center,
                ..UiStyle::default()
            });
            tree.add_child(row, v_bg);

            let v = tree.create(Widget::Label { text: val.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(v_bg, v);
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
