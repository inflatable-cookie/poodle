use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

#[derive(Clone, Copy)]
enum TokenKind {
    Color,
    Space,
    Radius,
    Opacity,
}

#[derive(Clone, Copy)]
struct TokenEntry {
    path: &'static str,
    kind: TokenKind,
}

const KEY_TOKENS: &[TokenEntry] = &[
    TokenEntry {
        path: "color.background.canvas",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.background.panel",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.background.elevated",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.text.primary",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.text.secondary",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.border.default",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.accent.base",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.status.success",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "size.control.height",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "space.control.x",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "space.control.y",
        kind: TokenKind::Space,
    },
];

const ALL_TOKENS: &[TokenEntry] = &[
    TokenEntry {
        path: "color.background.canvas",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.background.surface",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.background.panel",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.background.elevated",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.text.primary",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.text.secondary",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.text.inverse",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.border.subtle",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.border.default",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.border.strong",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.accent.base",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.accent.hover",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.accent.focusRing",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.status.success",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.status.warning",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "color.status.danger",
        kind: TokenKind::Color,
    },
    TokenEntry {
        path: "space.stack.sm",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "space.stack.md",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "space.stack.lg",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "space.inline.sm",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "space.inline.md",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "space.inline.lg",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "space.panel.x",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "space.panel.y",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "space.control.x",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "space.control.y",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "size.control.height",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "size.control.minWidth",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "size.icon.sm",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "size.icon.md",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "size.icon.lg",
        kind: TokenKind::Space,
    },
    TokenEntry {
        path: "radius.control",
        kind: TokenKind::Radius,
    },
    TokenEntry {
        path: "radius.surface",
        kind: TokenKind::Radius,
    },
    TokenEntry {
        path: "radius.pill",
        kind: TokenKind::Radius,
    },
    TokenEntry {
        path: "state.opacity.disabled",
        kind: TokenKind::Opacity,
    },
    TokenEntry {
        path: "state.opacity.muted",
        kind: TokenKind::Opacity,
    },
];

pub fn matching_token_count(query: &str) -> usize {
    filtered_tokens(query).len()
}

pub fn render_runtime_token_summary(theme: &GpuiThemeProvider) -> Div {
    let border = theme.resolve_color("color.border.subtle");
    let elevated_bg = theme.resolve_color("color.background.elevated");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");

    let mut grid = div().flex().flex_wrap().gap(px(12.0));

    for token in KEY_TOKENS {
        let value = resolve_token_value(theme, *token);
        let mut card = div()
            .w(px(220.0))
            .p(px(14.0))
            .rounded(px(8.0))
            .bg(color_to_hsla(elevated_bg))
            .border_1()
            .border_color(color_to_hsla(border))
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child(token.path),
            );

        if let Some(swatch) = resolve_swatch(theme, *token) {
            card = card.child(
                div()
                    .w(px(24.0))
                    .h(px(24.0))
                    .rounded(px(6.0))
                    .bg(swatch)
                    .border_1()
                    .border_color(color_to_hsla(border)),
            );
        }

        card = card.child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(color_to_hsla(text_primary))
                .child(value),
        );

        grid = grid.child(card);
    }

    grid
}

pub fn render_token_inspector(theme: &GpuiThemeProvider, query: &str) -> Div {
    let border = theme.resolve_color("color.border.subtle");
    let panel_bg = theme.resolve_color("color.background.panel");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");

    let mut rows = div().flex().flex_col().gap(px(8.0));

    for token in filtered_tokens(query) {
        let value = resolve_token_value(theme, token);
        let mut row = div()
            .p(px(12.0))
            .rounded(px(8.0))
            .bg(color_to_hsla(panel_bg))
            .border_1()
            .border_color(color_to_hsla(border))
            .flex()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(color_to_hsla(text_primary))
                            .child(token.path),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(color_to_hsla(text_secondary))
                            .child(value),
                    ),
            );

        if let Some(swatch) = resolve_swatch(theme, token) {
            row = row.child(
                div()
                    .w(px(20.0))
                    .h(px(20.0))
                    .rounded(px(5.0))
                    .bg(swatch)
                    .border_1()
                    .border_color(color_to_hsla(border)),
            );
        }

        rows = rows.child(row);
    }

    if filtered_tokens(query).is_empty() {
        rows = rows.child(
            div()
                .p(px(16.0))
                .rounded(px(8.0))
                .border_1()
                .border_color(color_to_hsla(border))
                .text_sm()
                .text_color(color_to_hsla(text_secondary))
                .child("No tokens match the current filter."),
        );
    }

    rows
}

fn filtered_tokens(query: &str) -> Vec<TokenEntry> {
    let q = query.trim().to_ascii_lowercase();
    ALL_TOKENS
        .iter()
        .copied()
        .filter(|token| q.is_empty() || token.path.to_ascii_lowercase().contains(&q))
        .collect()
}

fn resolve_swatch(theme: &GpuiThemeProvider, token: TokenEntry) -> Option<Hsla> {
    match token.kind {
        TokenKind::Color => Some(color_to_hsla(theme.resolve_color(token.path))),
        _ => None,
    }
}

fn resolve_token_value(theme: &GpuiThemeProvider, token: TokenEntry) -> String {
    match token.kind {
        TokenKind::Color => {
            let rgba: gpui::Rgba = color_to_hsla(theme.resolve_color(token.path)).into();
            format!(
                "rgba({:.0}, {:.0}, {:.0}, {:.2})",
                rgba.r * 255.0,
                rgba.g * 255.0,
                rgba.b * 255.0,
                rgba.a
            )
        }
        TokenKind::Space => format!("{:.1}px", theme.resolve_space(token.path)),
        TokenKind::Radius => format!("{:.1}px", theme.resolve_radius(token.path)),
        TokenKind::Opacity => format!("{:.2}", theme.resolve_opacity(token.path)),
    }
}
