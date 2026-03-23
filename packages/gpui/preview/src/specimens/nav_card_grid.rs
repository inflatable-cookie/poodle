use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{NavCardGridSpec, NavCardSpec, IconSpec};
use pug_gpui_components::{NavCardGrid, NavCard, Icon};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Navigation card grid (2 columns) ---
        .child(section_label("NAVIGATION CARD GRID (2 COLUMNS)", text_secondary))
        .child(
            NavCardGrid::from_spec(NavCardGridSpec::new(), theme)
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Getting Started")
                            .with_description("Learn the basics of the component library."),
                        theme,
                    )
                    .with_icon(Icon::from_spec(IconSpec::new("home"), theme))
                )
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Components")
                            .with_description("Browse all available components.")
                            .with_badge("New"),
                        theme,
                    )
                    .with_icon(Icon::from_spec(IconSpec::new("layers"), theme))
                )
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Tokens")
                            .with_description("Design tokens and theming system."),
                        theme,
                    )
                    .with_icon(Icon::from_spec(IconSpec::new("sliders-horizontal"), theme))
                )
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("API Reference")
                            .with_description("Complete component API documentation.")
                            .with_disabled(true),
                        theme,
                    )
                    .with_icon(Icon::from_spec(IconSpec::new("file-text"), theme))
                )
        )

        // --- 3 columns ---
        .child(section_label("3 COLUMNS", text_secondary))
        .child(
            NavCardGrid::from_spec(NavCardGridSpec::new().with_columns(3), theme)
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Overview").with_description("System overview"),
                        theme,
                    )
                )
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Installation").with_description("Setup guide"),
                        theme,
                    )
                )
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Configuration").with_description("Config options"),
                        theme,
                    )
                )
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Themes").with_description("Theme customization"),
                        theme,
                    )
                )
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Plugins").with_description("Extend functionality"),
                        theme,
                    )
                )
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("FAQ").with_description("Common questions"),
                        theme,
                    )
                )
        )

        // --- Single card (as link) ---
        .child(section_label("SINGLE CARD (AS LINK)", text_secondary))
        .child(
            NavCard::from_spec(
                NavCardSpec::new().with_title("View Documentation")
                    .with_description("Open the full documentation site.")
                    .with_href("#"),
                theme,
            )
        )

        // --- 4 columns ---
        .child(section_label("4 COLUMNS", text_secondary))
        .child(
            NavCardGrid::from_spec(NavCardGridSpec::new().with_columns(4), theme)
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Home").with_description("Return home"),
                        theme,
                    )
                    .with_icon(Icon::from_spec(IconSpec::new("home"), theme))
                )
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Search").with_description("Find content"),
                        theme,
                    )
                    .with_icon(Icon::from_spec(IconSpec::new("search"), theme))
                )
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Recent").with_description("Recent items"),
                        theme,
                    )
                    .with_icon(Icon::from_spec(IconSpec::new("clock"), theme))
                )
                .with_child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("Favorites").with_description("Saved items"),
                        theme,
                    )
                    .with_icon(Icon::from_spec(IconSpec::new("star"), theme))
                )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
