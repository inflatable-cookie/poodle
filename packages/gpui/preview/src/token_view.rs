use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::Table;
use poodle_specs::{TableColumn, TableRow, TableSpec};

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
                    .font_family("SF Mono")
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
                .font_family("SF Mono")
                .child(value),
        );

        grid = grid.child(card);
    }

    grid
}

pub fn render_token_inspector(theme: &GpuiThemeProvider, query: &str) -> AnyElement {
    let columns = vec![
        TableColumn::new("path", "Path").with_row_header(true),
        TableColumn::new("value", "Value"),
    ];

    let rows = filtered_tokens(query)
        .into_iter()
        .enumerate()
        .map(|(index, token)| {
            TableRow::new(
                format!("token-{}", index),
                vec![
                    ("path".to_string(), token.path.to_string()),
                    ("value".to_string(), resolve_token_value(theme, token)),
                ],
            )
        })
        .collect();

    Table::from_spec(
        TableSpec::new()
            .with_columns(columns)
            .with_rows(rows)
            .with_aria_label("Semantic token inspector")
            .with_empty_message("No tokens match the current filter."),
        theme,
    )
    .into_any_element()
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
