use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, PageLoading};
use poodle_specs::EyebrowSpec;
use poodle_specs::{PageLoadingPresentation, PageLoadingSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Inline"),
                    theme,
                ))
                .child(
                    div()
                        .min_h(px(288.0))
                        .border_1()
                        .border_color(color_to_hsla(theme.resolve_color("color.border.default")))
                        .rounded(px(8.0))
                        // GPUI has no dashed border. Approximate the Svelte inline shell.
                        .bg(color_to_hsla(
                            theme.resolve_color("color.background.surface"),
                        ))
                        .child(PageLoading::from_spec(
                            PageLoadingSpec::new()
                                .with_message("Loading section content...")
                                .with_presentation(PageLoadingPresentation::Inline),
                            theme,
                        )),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Indeterminate (spinner only)"),
                    theme,
                ))
                .child(PageLoading::from_spec(
                    PageLoadingSpec::new().with_message("Loading data..."),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Determinate (with progress bar)"),
                    theme,
                ))
                .child(PageLoading::from_spec(
                    PageLoadingSpec::new()
                        .with_value(64.0)
                        .with_max(100.0)
                        .with_message("Uploading files... 64%"),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With cancel button"),
                    theme,
                ))
                .child(PageLoading::from_spec(
                    PageLoadingSpec::new()
                        .with_message("Processing request...")
                        .with_can_cancel(true),
                    theme,
                )),
        )
}
