//! Token inspector — displays semantic tokens with color swatches and resolved values.
//!
//! Mirrors the GPUI preview's token_view.rs, adapted for Jetstream's UiTree.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use pug_tokens::semantic;

use crate::theme_bridge;

/// Render the full token inspector view.
pub fn render_token_inspector(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0), gap: 24.0,
        ..UiStyle::default()
    });

    // Header
    {
        let header = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 4.0,
            ..UiStyle::default()
        });
        tree.add_child(root, header);

        let title = tree.create(Widget::Label { text: "Token Inspector".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(18.0), ..UiStyle::default()
        });
        tree.add_child(header, title);

        let subtitle = tree.create(Widget::Label { text: "Semantic design tokens resolved via current theme".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(header, subtitle);
    }

    // Color sections
    render_color_section(tree, root, "Background Colors", theme, &[
        ("canvas", "semantic.color.background.canvas", semantic::COLOR_BACKGROUND_CANVAS),
        ("surface", "semantic.color.background.surface", semantic::COLOR_BACKGROUND_SURFACE),
        ("panel", "semantic.color.background.panel", semantic::COLOR_BACKGROUND_PANEL),
        ("elevated", "semantic.color.background.elevated", semantic::COLOR_BACKGROUND_ELEVATED),
        ("overlay", "semantic.color.background.overlay", semantic::COLOR_BACKGROUND_OVERLAY),
    ]);

    render_color_section(tree, root, "Text Colors", theme, &[
        ("primary", "semantic.color.text.primary", semantic::COLOR_TEXT_PRIMARY),
        ("secondary", "semantic.color.text.secondary", semantic::COLOR_TEXT_SECONDARY),
        ("inverse", "semantic.color.text.inverse", semantic::COLOR_TEXT_INVERSE),
    ]);

    render_color_section(tree, root, "Border Colors", theme, &[
        ("subtle", "semantic.color.border.subtle", semantic::COLOR_BORDER_SUBTLE),
        ("default", "semantic.color.border.default", semantic::COLOR_BORDER_DEFAULT),
        ("strong", "semantic.color.border.strong", semantic::COLOR_BORDER_STRONG),
    ]);

    render_color_section(tree, root, "Accent Colors", theme, &[
        ("base", "semantic.color.accent.base", semantic::COLOR_ACCENT_BASE),
        ("hover", "semantic.color.accent.hover", semantic::COLOR_ACCENT_HOVER),
        ("focus ring", "semantic.color.accent.focusRing", semantic::COLOR_ACCENT_FOCUS_RING),
    ]);

    render_color_section(tree, root, "Status Colors", theme, &[
        ("success", "semantic.color.status.success", semantic::COLOR_STATUS_SUCCESS),
        ("warning", "semantic.color.status.warning", semantic::COLOR_STATUS_WARNING),
        ("danger", "semantic.color.status.danger", semantic::COLOR_STATUS_DANGER),
    ]);

    render_color_section(tree, root, "Icon Colors", theme, &[
        ("primary", "semantic.color.icon.primary", semantic::COLOR_ICON_PRIMARY),
        ("muted", "semantic.color.icon.muted", semantic::COLOR_ICON_MUTED),
    ]);

    // Value sections
    render_value_section(tree, root, "Spacing", theme, &[
        ("stack-sm", semantic::SPACE_STACK_SM),
        ("stack-md", semantic::SPACE_STACK_MD),
        ("stack-lg", semantic::SPACE_STACK_LG),
        ("inline-sm", semantic::SPACE_INLINE_SM),
        ("inline-md", semantic::SPACE_INLINE_MD),
        ("inline-lg", semantic::SPACE_INLINE_LG),
        ("panel-x", semantic::SPACE_PANEL_X),
        ("panel-y", semantic::SPACE_PANEL_Y),
        ("control-x", semantic::SPACE_CONTROL_X),
        ("control-y", semantic::SPACE_CONTROL_Y),
    ]);

    render_value_section(tree, root, "Sizes", theme, &[
        ("control-height", semantic::SIZE_CONTROL_HEIGHT),
        ("control-min-width", semantic::SIZE_CONTROL_MIN_WIDTH),
        ("icon-sm", semantic::SIZE_ICON_SM),
        ("icon-md", semantic::SIZE_ICON_MD),
        ("icon-lg", semantic::SIZE_ICON_LG),
        ("panel-header", semantic::SIZE_PANEL_HEADER),
    ]);

    render_value_section(tree, root, "Border & Radius", theme, &[
        ("border-width-default", semantic::BORDER_WIDTH_DEFAULT),
        ("border-width-focus", semantic::BORDER_WIDTH_FOCUS),
        ("radius-control", semantic::RADIUS_CONTROL),
        ("radius-surface", semantic::RADIUS_SURFACE),
        ("radius-pill", semantic::RADIUS_PILL),
    ]);

    render_value_section(tree, root, "State & Opacity", theme, &[
        ("opacity-disabled", semantic::STATE_OPACITY_DISABLED),
        ("opacity-muted", semantic::STATE_OPACITY_MUTED),
    ]);

    // Typography section
    render_typography_section(tree, root, theme);

    root
}

/// Render a color section with swatch cards.
fn render_color_section(
    tree: &mut UiTree,
    parent: UiNodeId,
    title: &str,
    theme: &dyn ThemeProvider,
    tokens: &[(&str, &str, &str)],
) {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let section = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, gap: 8.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, section);

    let heading = tree.create(Widget::Label { text: title.to_string() }, UiStyle {
        text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
    });
    tree.add_child(section, heading);

    let grid = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 8.0,
        // Wrap not directly supported; using Row which will lay out horizontally
        ..UiStyle::default()
    });
    tree.add_child(section, grid);

    for &(label, path, raw_value) in tokens {
        let resolved = theme_bridge::resolve_vec4(theme, path);

        let card = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column,
            width: Sizing::Fixed(140.0),
            background: Some(bg_elevated),
            border_color: Some(border), border_width: 1.0,
            corner_radius: 6.0,
            ..UiStyle::default()
        });
        tree.add_child(grid, card);

        // Color swatch
        let swatch = tree.create(Widget::Panel, UiStyle {
            width: Sizing::Grow(1.0), height: Sizing::Fixed(32.0),
            background: Some(resolved),
            corner_radius: 0.0, // top radius is clipped by parent
            ..UiStyle::default()
        });
        tree.add_child(card, swatch);

        // Label area
        let info = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 2.0,
            padding: Edges::all(6.0),
            ..UiStyle::default()
        });
        tree.add_child(card, info);

        let name = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(info, name);

        let val = tree.create(Widget::Label { text: raw_value.to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(9.0), ..UiStyle::default()
        });
        tree.add_child(info, val);
    }
}

/// Render a value section with label/value rows.
fn render_value_section(
    tree: &mut UiTree,
    parent: UiNodeId,
    title: &str,
    theme: &dyn ThemeProvider,
    tokens: &[(&str, &str)],
) {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);

    let section = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, gap: 8.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, section);

    let heading = tree.create(Widget::Label { text: title.to_string() }, UiStyle {
        text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
    });
    tree.add_child(section, heading);

    let rows = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, gap: 4.0,
        ..UiStyle::default()
    });
    tree.add_child(section, rows);

    for &(label, raw_value) in tokens {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row, gap: 8.0, align: Align::Center,
            ..UiStyle::default()
        });
        tree.add_child(rows, row);

        let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0),
            width: Sizing::Fixed(120.0),
            ..UiStyle::default()
        });
        tree.add_child(row, lbl);

        let val = tree.create(Widget::Label { text: raw_value.to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(10.0),
            ..UiStyle::default()
        });
        tree.add_child(row, val);
    }
}

/// Render a typography section.
fn render_typography_section(tree: &mut UiTree, parent: UiNodeId, theme: &dyn ThemeProvider) {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);

    let section = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, gap: 8.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, section);

    let heading = tree.create(Widget::Label { text: "Typography".to_string() }, UiStyle {
        text_color: Some(text_primary), text_size: Some(14.0), ..UiStyle::default()
    });
    tree.add_child(section, heading);

    let families = [
        ("body", semantic::TYPOGRAPHY_BODY_FAMILY, semantic::TYPOGRAPHY_BODY_SIZE, semantic::TYPOGRAPHY_BODY_WEIGHT),
        ("label", semantic::TYPOGRAPHY_LABEL_FAMILY, semantic::TYPOGRAPHY_LABEL_SIZE, semantic::TYPOGRAPHY_LABEL_WEIGHT),
        ("heading", semantic::TYPOGRAPHY_HEADING_FAMILY, semantic::TYPOGRAPHY_HEADING_SIZE, semantic::TYPOGRAPHY_HEADING_WEIGHT),
        ("code", semantic::TYPOGRAPHY_CODE_FAMILY, semantic::TYPOGRAPHY_CODE_SIZE, semantic::TYPOGRAPHY_CODE_WEIGHT),
    ];

    for (name, family, size, weight) in &families {
        let row = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Column, gap: 2.0,
            ..UiStyle::default()
        });
        tree.add_child(section, row);

        let n = tree.create(Widget::Label { text: name.to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(12.0), ..UiStyle::default()
        });
        tree.add_child(row, n);

        let metrics = tree.create(Widget::Label {
            text: format!("size: {} • weight: {}", size, weight),
        }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(row, metrics);

        // Truncate family for display
        let fam_display = if family.len() > 50 {
            format!("{}...", &family[..47])
        } else {
            family.to_string()
        };
        let fam = tree.create(Widget::Label {
            text: format!("family: {}", fam_display),
        }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(10.0), ..UiStyle::default()
        });
        tree.add_child(row, fam);
    }
}
