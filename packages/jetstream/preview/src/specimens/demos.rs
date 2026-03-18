//! Demo screen renderer — builds full-page demo screens using composed specimens.
//!
//! Each screen mirrors the GPUI demo_view.rs, adapted for the Jetstream retained-mode
//! UiTree. Raw Panel/Label nodes are used directly since Jetstream doesn't have the
//! high-level Pug*Component wrappers that GPUI uses.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_layout::{CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment};

use crate::app_state::DemoScreen;
use crate::theme_bridge;

/// Render a single demo screen by variant.
pub fn render_single_screen(
    screen: DemoScreen,
    tree: &mut UiTree,
    theme: &dyn ThemeProvider,
) -> UiNodeId {
    match screen {
        DemoScreen::OverviewShell => render_overview_shell(tree, theme),
        DemoScreen::FormAndValidation => render_form_screen(tree, theme),
        DemoScreen::BrowseAndTable => render_browse_screen(tree, theme),
        DemoScreen::DetailAndRelatedData => render_detail_screen(tree, theme),
        DemoScreen::PickerAndMedia => render_picker_screen(tree, theme),
        DemoScreen::CommandAndWorkspace => render_workspace_screen(tree, theme),
    }
}

// ── Overview Shell ──

fn render_overview_shell(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let success = theme_bridge::resolve_vec4(theme, "semantic.color.status.success");
    let warning = theme_bridge::resolve_vec4(theme, "semantic.color.status.warning");
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");

    let root = tree.create_node(Widget::Panel,
        pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)
            .with_gap(12.0)),
        NodeStyle { ..NodeStyle::default() });

    // Info banner
    {
        let banner = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_gap(8.0)
                .with_padding(LayoutEdges::uniform(10.0))
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(theme_bridge::tint(accent, 0.08)),
                border_color: Some(theme_bridge::tint(accent, 0.3)),
                border_width: 1.0,
                corner_radii: [6.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, banner);

        let icon = tree.create_node(Widget::Label { text: "ℹ".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(accent), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(banner, icon);

        let col = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_gap(2.0)),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(banner, col);

        let title = tree.create_node(Widget::Label { text: "Welcome".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(col, title);

        let msg = tree.create_node(Widget::Label {
            text: "Your workspace is ready. 3 items need attention.".to_string(),
        },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(col, msg);
    }

    // State tiles row
    {
        let row = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(8.0)),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(root, row);

        for &(label, value, color) in &[
            ("Active projects", "12", success),
            ("Pending reviews", "3", warning),
            ("Open issues", "27", danger),
        ] {
            let tile = tree.create_node(Widget::Panel,
                pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_direction(LayoutDirection::Column)
                    .with_width(LayoutSizing::Grow)
                    .with_gap(4.0)
                    .with_padding(LayoutEdges::uniform(12.0))),
                NodeStyle {
                    background: Some(bg_elevated),
                    border_color: Some(border),
                    border_width: 1.0,
                    corner_radii: [8.0; 4],
                    ..NodeStyle::default()
                });
            tree.add_child(row, tile);

            let lbl = tree.create_node(Widget::Label { text: label.to_string() },
                pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(tile, lbl);

            let val = tree.create_node(Widget::Label { text: value.to_string() },
                pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle { text_color: Some(color), text_size: Some(20.0), ..NodeStyle::default() });
            tree.add_child(tile, val);
        }
    }

    // Progress bar
    {
        let progress_col = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_gap(4.0)),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(root, progress_col);

        let lbl = tree.create_node(Widget::Label { text: "Sprint progress".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(progress_col, lbl);

        let bar = tree.create_node(Widget::ProgressBar { value: 0.75 },
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(6.0))),
            NodeStyle { corner_radii: [3.0; 4], ..NodeStyle::default() });
        tree.add_child(progress_col, bar);
    }

    root
}

// ── Form & Validation ──

fn render_form_screen(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let border = theme_bridge::border_subtle(theme);
    let danger = theme_bridge::resolve_vec4(theme, "semantic.color.status.danger");
    let bg_canvas = theme_bridge::canvas_background(theme);

    let root = tree.create_node(Widget::Panel,
        pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)
            .with_gap(12.0)),
        NodeStyle { ..NodeStyle::default() });

    // Form fields
    for &(label, placeholder, required) in &[
        ("Title", "Enter title...", true),
        ("Description", "Enter description...", false),
        ("Category", "Select category...", false),
    ] {
        let field = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_gap(4.0)),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(root, field);

        let label_text = if required {
            format!("{} *", label)
        } else {
            label.to_string()
        };
        let lbl = tree.create_node(Widget::Label { text: label_text },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(field, lbl);

        let input = tree.create_node(Widget::Label { text: placeholder.to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(32.0))
                .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(bg_canvas),
                border_color: Some(border),
                border_width: 1.0,
                corner_radii: [6.0; 4],
                text_color: Some(text_secondary),
                text_size: Some(12.0),
                ..NodeStyle::default()
            });
        tree.add_child(field, input);
    }

    // Validation banner
    {
        let banner = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_gap(8.0)
                .with_padding(LayoutEdges::uniform(10.0))
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(theme_bridge::tint(danger, 0.08)),
                border_color: Some(theme_bridge::tint(danger, 0.3)),
                border_width: 1.0,
                corner_radii: [6.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, banner);

        let icon = tree.create_node(Widget::Label { text: "⚠".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(danger), text_size: Some(14.0), ..NodeStyle::default() });
        tree.add_child(banner, icon);

        let msg = tree.create_node(Widget::Label {
            text: "Please correct the errors above".to_string(),
        },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(danger), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(banner, msg);
    }

    // Action buttons
    {
        let actions = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::End, CrossAxisAlignment::Start)),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(root, actions);

        let cancel = tree.create_node(Widget::Button {
            label: "Cancel".to_string(), pressed: false, hovered: false,
        },
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(32.0))
                .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle {
                corner_radii: [6.0; 4],
                border_color: Some(border),
                border_width: 1.0,
                text_color: Some(text_primary),
                text_size: Some(12.0),
                ..NodeStyle::default()
            });
        tree.add_child(actions, cancel);

        let submit = tree.create_node(Widget::Button {
            label: "Submit".to_string(), pressed: false, hovered: false,
        },
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(32.0))
                .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle {
                corner_radii: [6.0; 4],
                background: Some(accent),
                text_color: Some(text_inverse),
                text_size: Some(12.0),
                ..NodeStyle::default()
            });
        tree.add_child(actions, submit);
    }

    root
}

// ── Browse & Table ──

fn render_browse_screen(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel,
        pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)
            .with_gap(12.0)),
        NodeStyle { ..NodeStyle::default() });

    // Search bar
    {
        let search = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(32.0))
                .with_gap(8.0)
                .with_padding(LayoutEdges { top: 0.0, right: 10.0, bottom: 0.0, left: 10.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(bg_canvas),
                border_color: Some(border),
                border_width: 1.0,
                corner_radii: [6.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, search);

        let icon = tree.create_node(Widget::Label { text: "🔍".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(search, icon);

        let placeholder = tree.create_node(Widget::Label { text: "Search assets…".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(search, placeholder);
    }

    // Data table
    {
        let table = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)),
            NodeStyle {
                background: Some(bg_elevated),
                border_color: Some(border),
                border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, table);

        // Header row
        let header = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(32.0))
                .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                border_color: Some(border),
                border_width: 1.0,
                ..NodeStyle::default()
            });
        tree.add_child(table, header);

        for &col in &["Name", "Type", "Status", "Modified"] {
            let cell = tree.create_node(Widget::Label { text: col.to_string() },
                pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_width(LayoutSizing::Grow)),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
            tree.add_child(header, cell);
        }

        // Data rows
        let rows = [
            ["hero-banner.png", "Image", "Published", "2 hours ago"],
            ["main.css", "Stylesheet", "Draft", "Yesterday"],
            ["index.html", "Document", "Published", "3 days ago"],
            ["config.json", "Config", "Review", "1 week ago"],
        ];

        for row_data in &rows {
            let row = tree.create_node(Widget::Panel,
                pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_width(LayoutSizing::Grow)
                    .with_height(LayoutSizing::Fixed(36.0))
                    .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle {
                    border_color: Some(border),
                    border_width: 1.0,
                    ..NodeStyle::default()
                });
            tree.add_child(table, row);

            for (i, &val) in row_data.iter().enumerate() {
                let color = if i == 0 { text_primary } else { text_secondary };
                let cell = tree.create_node(Widget::Label { text: val.to_string() },
                    pug_jetstream::map_layout(&LayoutIntent::new()
                        .with_width(LayoutSizing::Grow)),
                    NodeStyle { text_color: Some(color), text_size: Some(11.0), ..NodeStyle::default() });
                tree.add_child(row, cell);
            }
        }
    }

    // Pagination
    {
        let pagination = tree.create_node(Widget::Label {
            text: "Showing 1–25 of 142".to_string(),
        },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(root, pagination);
    }

    root
}

// ── Detail & Related Data ──

fn render_detail_screen(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel,
        pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)
            .with_gap(12.0)),
        NodeStyle { ..NodeStyle::default() });

    // Breadcrumbs
    {
        let crumbs = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(4.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(root, crumbs);

        for (i, &item) in ["Home", "Assets", "hero-banner.png"].iter().enumerate() {
            if i > 0 {
                let sep = tree.create_node(Widget::Label { text: "/".to_string() },
                    pug_jetstream::map_layout(&LayoutIntent::new()),
                    NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
                tree.add_child(crumbs, sep);
            }
            let is_last = i == 2;
            let lbl = tree.create_node(Widget::Label { text: item.to_string() },
                pug_jetstream::map_layout(&LayoutIntent::new()),
                NodeStyle {
                    text_color: Some(if is_last { text_primary } else { accent }),
                    text_size: Some(11.0),
                    ..NodeStyle::default()
                });
            tree.add_child(crumbs, lbl);
        }
    }

    // Detail shell — title + card
    {
        let title = tree.create_node(Widget::Label { text: "hero-banner.png".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(18.0), ..NodeStyle::default() });
        tree.add_child(root, title);

        // Overview section
        let overview = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_gap(6.0)
                .with_padding(LayoutEdges::uniform(12.0))),
            NodeStyle {
                background: Some(bg_elevated),
                border_color: Some(border),
                border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, overview);

        let heading = tree.create_node(Widget::Label { text: "Overview".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(overview, heading);

        let desc = tree.create_node(Widget::Label {
            text: "Image file uploaded 2 hours ago. 1920\u{00d7}1080 pixels, 2.4 MB.".to_string(),
        },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(overview, desc);

        // Badges
        let badges = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_gap(6.0)),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(overview, badges);

        for &(badge_text, is_accent) in &[("Published", true), ("Image", false)] {
            let bg = if is_accent {
                theme_bridge::tint(accent, 0.15)
            } else {
                theme_bridge::tint(border, 0.5)
            };
            let badge = tree.create_node(Widget::Label { text: badge_text.to_string() },
                pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_height(LayoutSizing::Fixed(20.0))
                    .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
                    .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
                NodeStyle {
                    background: Some(bg),
                    corner_radii: [10.0; 4],
                    text_color: Some(if is_accent { accent } else { text_secondary }),
                    text_size: Some(10.0),
                    ..NodeStyle::default()
                });
            tree.add_child(badges, badge);
        }

        // Separator
        let sep = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(1.0))),
            NodeStyle { background: Some(border), ..NodeStyle::default() });
        tree.add_child(root, sep);

        // Media section
        let media = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_gap(6.0)
                .with_padding(LayoutEdges::uniform(12.0))),
            NodeStyle {
                background: Some(bg_elevated),
                border_color: Some(border),
                border_width: 1.0,
                corner_radii: [8.0; 4],
                ..NodeStyle::default()
            });
        tree.add_child(root, media);

        let mh = tree.create_node(Widget::Label { text: "Media".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(media, mh);

        let md = tree.create_node(Widget::Label {
            text: "Preview area for the selected asset.".to_string(),
        },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(media, md);
    }

    root
}

// ── Picker & Media ──

fn render_picker_screen(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);

    let root = tree.create_node(Widget::Panel,
        pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)
            .with_gap(8.0)),
        NodeStyle { ..NodeStyle::default() });

    // Checkbox items
    for &(item, checked) in &[
        ("hero-banner.png", true),
        ("logo.svg", true),
        ("background.jpg", false),
        ("icon-set.zip", false),
    ] {
        let indicator = if checked { "☑" } else { "☐" };
        let row = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_height(LayoutSizing::Fixed(28.0))
                .with_gap(8.0)
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(root, row);

        let check = tree.create_node(Widget::Label { text: indicator.to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle {
                text_color: Some(if checked { accent } else { text_secondary }),
                text_size: Some(14.0),
                ..NodeStyle::default()
            });
        tree.add_child(row, check);

        let lbl = tree.create_node(Widget::Label { text: item.to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(12.0), ..NodeStyle::default() });
        tree.add_child(row, lbl);
    }

    // Selection summary + confirm
    {
        let footer = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_padding(LayoutEdges { top: 8.0, right: 0.0, bottom: 0.0, left: 0.0 })
                .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(root, footer);

        let summary = tree.create_node(Widget::Label { text: "2 items selected".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(footer, summary);

        let confirm = tree.create_node(Widget::Button {
            label: "Confirm".to_string(), pressed: false, hovered: false,
        },
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_height(LayoutSizing::Fixed(32.0))
                .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle {
                corner_radii: [6.0; 4],
                background: Some(accent),
                text_color: Some(text_inverse),
                text_size: Some(12.0),
                ..NodeStyle::default()
            });
        tree.add_child(footer, confirm);
    }

    root
}

// ── Command & Workspace ──

fn render_workspace_screen(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create_node(Widget::Panel,
        pug_jetstream::map_layout(&LayoutIntent::new()
            .with_direction(LayoutDirection::Column)
            .with_width(LayoutSizing::Grow)),
        NodeStyle { ..NodeStyle::default() });

    // App header bar
    {
        let header = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(40.0))
                .with_gap(8.0)
                .with_padding(LayoutEdges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(bg_surface),
                border_color: Some(border),
                border_width: 1.0,
                ..NodeStyle::default()
            });
        tree.add_child(root, header);

        let app_title = tree.create_node(Widget::Label { text: "Demo Project".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Grow)),
            NodeStyle { text_color: Some(text_primary), text_size: Some(13.0), ..NodeStyle::default() });
        tree.add_child(header, app_title);

        let branch = tree.create_node(Widget::Label { text: "main branch".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(header, branch);
    }

    // Split view: left dock + main panel
    {
        let split = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(240.0))),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(root, split);

        // Left dock — Explorer
        let left_dock = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Fixed(180.0))
                .with_height(LayoutSizing::Grow)),
            NodeStyle {
                background: Some(bg_elevated),
                border_color: Some(border),
                border_width: 1.0,
                ..NodeStyle::default()
            });
        tree.add_child(split, left_dock);

        // Panel header
        let lh = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(32.0))
                .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                border_color: Some(border),
                border_width: 1.0,
                ..NodeStyle::default()
            });
        tree.add_child(left_dock, lh);

        let lt = tree.create_node(Widget::Label { text: "Explorer".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(lh, lt);

        // File tree
        let file_tree = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_gap(4.0)
                .with_padding(LayoutEdges::uniform(8.0))),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(left_dock, file_tree);

        let dir = tree.create_node(Widget::Label { text: "src/".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(file_tree, dir);

        for &file in &["main.rs", "lib.rs"] {
            let f = tree.create_node(Widget::Label { text: file.to_string() },
                pug_jetstream::map_layout(&LayoutIntent::new()
                    .with_padding(LayoutEdges { top: 0.0, right: 0.0, bottom: 0.0, left: 12.0 })),
                NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
            tree.add_child(file_tree, f);
        }

        // Main panel — Editor
        let main_panel = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Column)
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Grow)),
            NodeStyle {
                background: Some(bg_elevated),
                border_color: Some(border),
                border_width: 1.0,
                ..NodeStyle::default()
            });
        tree.add_child(split, main_panel);

        let mh = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(32.0))
                .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
                .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Center)),
            NodeStyle {
                border_color: Some(border),
                border_width: 1.0,
                ..NodeStyle::default()
            });
        tree.add_child(main_panel, mh);

        let mt = tree.create_node(Widget::Label { text: "Editor".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_primary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(mh, mt);

        let mc = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Grow)
                .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)),
            NodeStyle { ..NodeStyle::default() });
        tree.add_child(main_panel, mc);

        let mp = tree.create_node(Widget::Label { text: "Panel content area".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(11.0), ..NodeStyle::default() });
        tree.add_child(mc, mp);
    }

    // Status bar
    {
        let status = tree.create_node(Widget::Panel,
            pug_jetstream::map_layout(&LayoutIntent::new()
                .with_direction(LayoutDirection::Row)
                .with_width(LayoutSizing::Grow)
                .with_height(LayoutSizing::Fixed(24.0))
                .with_padding(LayoutEdges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 })
                .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)),
            NodeStyle {
                background: Some(bg_surface),
                border_color: Some(border),
                border_width: 1.0,
                ..NodeStyle::default()
            });
        tree.add_child(root, status);

        let ready = tree.create_node(Widget::Label { text: "Ready".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(status, ready);

        let cursor = tree.create_node(Widget::Label { text: "Ln 42, Col 18".to_string() },
            pug_jetstream::map_layout(&LayoutIntent::new()),
            NodeStyle { text_color: Some(text_secondary), text_size: Some(10.0), ..NodeStyle::default() });
        tree.add_child(status, cursor);
    }

    root
}
