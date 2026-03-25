use gpui::*;
use poodle_primitives::{NavCardGridSpec, NavCardSpec, IconSpec, EyebrowSpec};
use poodle_gpui_components::{NavCardGrid, NavCard, Icon, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Navigation card grid (2 columns) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
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

        // --- Single card (as link) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
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
}
