use gpui::*;
use pug_primitives::{NavCardGridSpec, NavCardSpec, IconSpec, EyebrowSpec};
use pug_gpui_components::{NavCardGrid, NavCard, Icon, Eyebrow};
use pug_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Navigation card grid (2 columns) ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Navigation card grid (2 columns)"), theme))
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
        )

        // --- 3 columns ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("3 columns"), theme))
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
        )

        // --- Single card (as link) ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Single card (as link)"), theme))
                .child(
                    NavCard::from_spec(
                        NavCardSpec::new().with_title("View Documentation")
                            .with_description("Open the full documentation site.")
                            .with_href("#"),
                        theme,
                    )
                )
        )

        // --- 4 columns ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("4 columns"), theme))
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
        )
}
