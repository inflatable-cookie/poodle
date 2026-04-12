use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, Icon, NavCard};
use poodle_specs::{EyebrowSpec, IconSpec, NavCardSpec};

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
                    EyebrowSpec::new().with_content("Navigation cards"),
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
                                    .with_description("Design tokens and theming system.")
                                    .with_badge("Core"),
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
                    EyebrowSpec::new().with_content("Shell destination cards"),
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
                                    .with_title("Command Center")
                                    .with_description(
                                        "Open shared command search and recent actions.",
                                    )
                                    .with_badge("⌘K")
                                    .with_href("#commands"),
                                theme,
                            )
                            .with_icon(Icon::from_spec(IconSpec::new("terminal"), theme)),
                        )
                        .child(
                            NavCard::from_spec(
                                NavCardSpec::new()
                                    .with_title("Workspace Layout")
                                    .with_description("Inspect sidebars, docks, and shell regions.")
                                    .with_href("#workspace"),
                                theme,
                            )
                            .with_icon(Icon::from_spec(IconSpec::new("panel-left"), theme)),
                        ),
                ),
        )
}
