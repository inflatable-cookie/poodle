//! ListShell specimen — list layout browse view with empty, loading, error, and populated states.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 24.0,
        ..UiStyle::default()
    });

    // ── Populated ──
    section_label(tree, root, "Populated List", text_secondary);
    {
        let shell = list_shell_frame(tree, bg_elevated, border);
        tree.add_child(root, shell);

        // Header
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            padding: Edges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 },
            background: Some(bg_surface),
            justify: Justify::SpaceBetween, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(shell, header);
        let title = tree.create(Widget::Label { text: "Documents".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(header, title);
        let count = tree.create(Widget::Label { text: "3 items".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(header, count);

        for &(name, meta) in &[
            ("Project Brief.pdf", "Updated 2 hours ago · 2.4 MB"),
            ("Q4 Report.xlsx", "Updated yesterday · 1.1 MB"),
            ("Design Spec.md", "Updated 3 days ago · 48 KB"),
        ] {
            let sep = tree.create(Widget::Panel, UiStyle {
                width: Sizing::Grow(1.0), height: Sizing::Fixed(1.0),
                background: Some(border), ..UiStyle::default()
            });
            tree.add_child(shell, sep);

            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Column,
                padding: Edges { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 },
                gap: 2.0,
                ..UiStyle::default()
            });
            tree.add_child(shell, row);

            let n = tree.create(Widget::Label { text: name.to_string() }, UiStyle {
                text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
            });
            tree.add_child(row, n);

            let m = tree.create(Widget::Label { text: meta.to_string() }, UiStyle {
                text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
            });
            tree.add_child(row, m);
        }
    }

    // ── Loading ──
    section_label(tree, root, "Loading State", text_secondary);
    {
        let shell = list_shell_frame(tree, bg_elevated, border);
        tree.add_child(root, shell);

        let loading = tree.create(Widget::Panel, UiStyle {
            padding: Edges::all(32.0),
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(shell, loading);

        let bar = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Fixed(120.0), height: Sizing::Fixed(4.0),
            background: Some(theme_bridge::tint(accent, 0.3)),
            corner_radius: 2.0, ..UiStyle::default()
        });
        tree.add_child(loading, bar);
    }

    // ── Empty ──
    section_label(tree, root, "Empty State", text_secondary);
    {
        let shell = list_shell_frame(tree, bg_elevated, border);
        tree.add_child(root, shell);

        let empty = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges::all(32.0), gap: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(shell, empty);

        let msg = tree.create(Widget::Label { text: "No documents yet".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(empty, msg);

        let hint = tree.create(Widget::Label { text: "Upload or create a new document to get started.".to_string() }, UiStyle {
            text_color: Some(theme_bridge::tint(text_secondary, 0.7)), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(empty, hint);
    }

    // ── Error ──
    section_label(tree, root, "Error State", text_secondary);
    {
        let shell = list_shell_frame(tree, bg_elevated, border);
        tree.add_child(root, shell);

        let err = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            padding: Edges::all(24.0), gap: 8.0,
            align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(shell, err);

        let icon = tree.create(Widget::Label { text: "⚠".to_string() }, UiStyle {
            text_color: Some(warning), text_size: Some(20.0), ..UiStyle::default()
        });
        tree.add_child(err, icon);

        let msg = tree.create(Widget::Label { text: "Failed to load documents".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(err, msg);

        let retry = tree.create(Widget::Button {
            label: "Retry".to_string(), pressed: false, hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            corner_radius: 6.0, border_color: Some(border), border_width: 1.0,
            text_color: Some(text_primary), text_size: Some(11.0),
            align: Align::Center, justify: Justify::Center,
            focusable: true, ..UiStyle::default()
        });
        tree.add_child(err, retry);
    }

    root
}

fn list_shell_frame(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(360.0),
        background: Some(bg),
        border_color: Some(border), border_width: 1.0,
        corner_radius: 8.0,
        ..UiStyle::default()
    })
}

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}
