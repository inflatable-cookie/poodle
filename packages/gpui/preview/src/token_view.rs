//! Token inspector — displays all semantic tokens with swatches and resolved values.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_tokens::semantic;
use crate::style_bridge::color_to_hsla;

/// Render the token inspector view.
pub fn render_token_inspector(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child(div().text_xl().child("Token Inspector"))
                .child(div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(format!("Active theme: {}", theme.theme_name)))
        )
        .child(render_color_section("Background Colors", theme, &[
            ("canvas", "semantic.color.background.canvas", semantic::COLOR_BACKGROUND_CANVAS),
            ("surface", "semantic.color.background.surface", semantic::COLOR_BACKGROUND_SURFACE),
            ("panel", "semantic.color.background.panel", semantic::COLOR_BACKGROUND_PANEL),
            ("elevated", "semantic.color.background.elevated", semantic::COLOR_BACKGROUND_ELEVATED),
            ("overlay", "semantic.color.background.overlay", semantic::COLOR_BACKGROUND_OVERLAY),
        ]))
        .child(render_color_section("Text Colors", theme, &[
            ("primary", "semantic.color.text.primary", semantic::COLOR_TEXT_PRIMARY),
            ("secondary", "semantic.color.text.secondary", semantic::COLOR_TEXT_SECONDARY),
            ("inverse", "semantic.color.text.inverse", semantic::COLOR_TEXT_INVERSE),
        ]))
        .child(render_color_section("Border Colors", theme, &[
            ("subtle", "semantic.color.border.subtle", semantic::COLOR_BORDER_SUBTLE),
            ("default", "semantic.color.border.default", semantic::COLOR_BORDER_DEFAULT),
            ("strong", "semantic.color.border.strong", semantic::COLOR_BORDER_STRONG),
        ]))
        .child(render_color_section("Accent Colors", theme, &[
            ("base", "semantic.color.accent.base", semantic::COLOR_ACCENT_BASE),
            ("hover", "semantic.color.accent.hover", semantic::COLOR_ACCENT_HOVER),
            ("focus ring", "semantic.color.accent.focusRing", semantic::COLOR_ACCENT_FOCUS_RING),
        ]))
        .child(render_color_section("Status Colors", theme, &[
            ("success", "semantic.color.status.success", semantic::COLOR_STATUS_SUCCESS),
            ("warning", "semantic.color.status.warning", semantic::COLOR_STATUS_WARNING),
            ("danger", "semantic.color.status.danger", semantic::COLOR_STATUS_DANGER),
        ]))
        .child(render_color_section("Icon Colors", theme, &[
            ("primary", "semantic.color.icon.primary", semantic::COLOR_ICON_PRIMARY),
            ("muted", "semantic.color.icon.muted", semantic::COLOR_ICON_MUTED),
        ]))
        .child(render_space_section("Spacing", theme, &[
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
        ]))
        .child(render_size_section("Sizes", theme, &[
            ("control-height", semantic::SIZE_CONTROL_HEIGHT),
            ("control-min-width", semantic::SIZE_CONTROL_MIN_WIDTH),
            ("icon-sm", semantic::SIZE_ICON_SM),
            ("icon-md", semantic::SIZE_ICON_MD),
            ("icon-lg", semantic::SIZE_ICON_LG),
            ("panel-header", semantic::SIZE_PANEL_HEADER),
        ]))
        .child(render_misc_section("Border & Radius", theme, &[
            ("border-width-default", semantic::BORDER_WIDTH_DEFAULT),
            ("border-width-focus", semantic::BORDER_WIDTH_FOCUS),
            ("radius-control", semantic::RADIUS_CONTROL),
            ("radius-surface", semantic::RADIUS_SURFACE),
            ("radius-pill", semantic::RADIUS_PILL),
        ]))
        .child(render_misc_section("State & Opacity", theme, &[
            ("opacity-disabled", semantic::STATE_OPACITY_DISABLED),
            ("opacity-muted", semantic::STATE_OPACITY_MUTED),
        ]))
        .child(render_typography_section(theme))
}

fn render_color_section(
    title: &str,
    theme: &GpuiThemeProvider,
    tokens: &[(&str, &str, &str)],
) -> Div {
    let border = theme.resolve_color("semantic.color.border.default");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let elevated_bg = theme.resolve_color("semantic.color.background.elevated");

    let mut section = div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(div().text_base().child(title.to_string()));

    let mut grid = div().flex().flex_wrap().gap(px(8.0));

    for &(label, path, raw_value) in tokens {
        let resolved = theme.resolve_color(path);

        grid = grid.child(
            div()
                .w(px(160.0))
                .rounded(px(6.0))
                .bg(color_to_hsla(elevated_bg))
                .border_1()
                .border_color(color_to_hsla(border))
                .flex()
                .flex_col()
                .child(
                    div()
                        .h(px(32.0))
                        .rounded_tl(px(5.0))
                        .rounded_tr(px(5.0))
                        .bg(color_to_hsla(resolved)),
                )
                .child(
                    div()
                        .p(px(6.0))
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(div().text_xs().child(label.to_string()))
                        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child(raw_value.to_string())),
                ),
        );
    }

    section = section.child(grid);
    section
}

fn render_space_section(
    title: &str,
    theme: &GpuiThemeProvider,
    tokens: &[(&str, &str)],
) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let accent = theme.resolve_color("semantic.color.accent.base");

    let mut section = div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(div().text_base().child(title.to_string()));

    let mut rows = div().flex().flex_col().gap(px(4.0));
    for &(label, raw_value) in tokens {
        let resolved_px = theme.resolve_space(raw_value);

        rows = rows.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(div().w(px(100.0)).text_xs().text_color(color_to_hsla(text_secondary)).child(label.to_string()))
                .child(
                    div()
                        .w(px(resolved_px.min(200.0)))
                        .h(px(12.0))
                        .rounded(px(2.0))
                        .bg(color_to_hsla(accent).opacity(0.3))
                        .border_1()
                        .border_color(color_to_hsla(accent).opacity(0.5)),
                )
                .child(div().text_xs().child(format!("{} → {}px", raw_value, resolved_px))),
        );
    }

    section = section.child(rows);
    section
}

fn render_size_section(
    title: &str,
    theme: &GpuiThemeProvider,
    tokens: &[(&str, &str)],
) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let mut section = div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(div().text_base().child(title.to_string()));

    let mut rows = div().flex().flex_col().gap(px(4.0));
    for &(label, raw_value) in tokens {
        let resolved_px = theme.resolve_space(raw_value);

        rows = rows.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(div().w(px(120.0)).text_xs().text_color(color_to_hsla(text_secondary)).child(label.to_string()))
                .child(div().text_xs().child(format!("{} → {}px", raw_value, resolved_px))),
        );
    }

    section = section.child(rows);
    section
}

fn render_misc_section(
    title: &str,
    theme: &GpuiThemeProvider,
    tokens: &[(&str, &str)],
) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let mut section = div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(div().text_base().child(title.to_string()));

    let mut rows = div().flex().flex_col().gap(px(4.0));
    for &(label, raw_value) in tokens {
        rows = rows.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(div().w(px(140.0)).text_xs().text_color(color_to_hsla(text_secondary)).child(label.to_string()))
                .child(div().text_xs().child(raw_value.to_string())),
        );
    }

    section = section.child(rows);
    section
}

fn render_typography_section(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let families = [
        ("body", semantic::TYPOGRAPHY_BODY_FAMILY, semantic::TYPOGRAPHY_BODY_SIZE, semantic::TYPOGRAPHY_BODY_WEIGHT),
        ("label", semantic::TYPOGRAPHY_LABEL_FAMILY, semantic::TYPOGRAPHY_LABEL_SIZE, semantic::TYPOGRAPHY_LABEL_WEIGHT),
        ("heading", semantic::TYPOGRAPHY_HEADING_FAMILY, semantic::TYPOGRAPHY_HEADING_SIZE, semantic::TYPOGRAPHY_HEADING_WEIGHT),
        ("code", semantic::TYPOGRAPHY_CODE_FAMILY, semantic::TYPOGRAPHY_CODE_SIZE, semantic::TYPOGRAPHY_CODE_WEIGHT),
    ];

    let mut section = div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(div().text_base().child("Typography"));

    for (name, family, size, weight) in &families {
        section = section.child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(div().text_sm().child(format!("{}", name)))
                .child(div().text_xs().text_color(color_to_hsla(text_secondary))
                    .child(format!("size: {} • weight: {}", size, weight)))
                .child(div().text_xs().text_color(color_to_hsla(text_secondary))
                    .child(format!("family: {}", truncate_family(family)))),
        );
    }

    section
}

fn truncate_family(family: &str) -> String {
    if family.len() > 50 {
        format!("{}...", &family[..47])
    } else {
        family.to_string()
    }
}
