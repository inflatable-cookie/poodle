use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, Icon, NavCard};
use poodle_specs::{ControlDensity, EyebrowSpec, IconSpec, NavCardSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(720.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Navigation cards in a grid (2 columns)"),
                    theme,
                ))
                .child(
                    div()
                        .grid()
                        .grid_cols(2)
                        .gap(px(12.0))
                        .child(
                            NavCard::from_spec(
                                NavCardSpec::new()
                                    .with_title("Getting Started")
                                    .with_description("Learn the basics of the component library."),
                                theme,
                            )
                            .with_icon(Icon::from_spec(IconSpec::new("folder"), theme)),
                        )
                        .child(
                            NavCard::from_spec(
                                NavCardSpec::new()
                                    .with_title("Components")
                                    .with_description("Browse all available components.")
                                    .with_badge("New"),
                                theme,
                            )
                            .with_icon(Icon::from_spec(IconSpec::new("search"), theme)),
                        )
                        .child(
                            NavCard::from_spec(
                                NavCardSpec::new()
                                    .with_title("Tokens")
                                    .with_description("Design tokens and theming system."),
                                theme,
                            )
                            .with_icon(Icon::from_spec(IconSpec::new("settings"), theme)),
                        )
                        .child(
                            NavCard::from_spec(
                                NavCardSpec::new()
                                    .with_title("API Reference")
                                    .with_description("Complete component API documentation.")
                                    .with_disabled(true),
                                theme,
                            )
                            .with_icon(Icon::from_spec(IconSpec::new("file"), theme)),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Single card (as link)"),
                    theme,
                ))
                .child(NavCard::from_spec(
                    NavCardSpec::new()
                        .with_title("View Documentation")
                        .with_description("Open the full documentation site.")
                        .with_href("#"),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Numeric badge"),
                    theme,
                ))
                .child(
                    NavCard::from_spec(
                        NavCardSpec::new()
                            .with_title("Composites")
                            .with_description("Higher-order components built from primitives.")
                            .with_badge("12"),
                        theme,
                    )
                    .with_icon(Icon::from_spec(IconSpec::new("layers"), theme)),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Density variants (compact / default / comfortable)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(
                            NavCard::from_spec(
                                NavCardSpec::new()
                                    .with_title("Components")
                                    .with_description("Browse all available components.")
                                    .with_badge("New")
                                    .with_density(ControlDensity::Compact),
                                theme,
                            )
                            .with_icon(Icon::from_spec(IconSpec::new("layers"), theme)),
                        )
                        .child(
                            NavCard::from_spec(
                                NavCardSpec::new()
                                    .with_title("Components")
                                    .with_description("Browse all available components.")
                                    .with_badge("New")
                                    .with_density(ControlDensity::Default),
                                theme,
                            )
                            .with_icon(Icon::from_spec(IconSpec::new("layers"), theme)),
                        )
                        .child(
                            NavCard::from_spec(
                                NavCardSpec::new()
                                    .with_title("Components")
                                    .with_description("Browse all available components.")
                                    .with_badge("New")
                                    .with_density(ControlDensity::Comfortable),
                                theme,
                            )
                            .with_icon(Icon::from_spec(IconSpec::new("layers"), theme)),
                        ),
                ),
        )
}
