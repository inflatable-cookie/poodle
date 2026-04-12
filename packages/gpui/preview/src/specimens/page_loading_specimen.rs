use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, PageLoading};
use poodle_specs::EyebrowSpec;
use poodle_specs::{
    ControlDensity, ControlSize, PageLoadingPresentation, PageLoadingSpec, SemanticControlSizeRole,
};

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
                        .with_value(60.0)
                        .with_max(100.0)
                        .with_message("Uploading files... 60%"),
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
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Determinate with cancel"),
                    theme,
                ))
                .child(PageLoading::from_spec(
                    PageLoadingSpec::new()
                        .with_value(35.0)
                        .with_max(100.0)
                        .with_message("Importing data... 35%")
                        .with_can_cancel(true),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Inline presentation (no backdrop)"),
                    theme,
                ))
                .child(PageLoading::from_spec(
                    PageLoadingSpec::new()
                        .with_message("Loading this section\u{2026}")
                        .with_presentation(PageLoadingPresentation::Inline),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Semantic presentation"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(PageLoading::from_spec(
                            PageLoadingSpec::new()
                                .with_message("Preparing region…")
                                .with_presentation(PageLoadingPresentation::Inline)
                                .with_size(ControlSize::Sm)
                                .with_density(ControlDensity::Compact),
                            theme,
                        ))
                        .child(PageLoading::from_spec(
                            PageLoadingSpec::new()
                                .with_value(82.0)
                                .with_max(100.0)
                                .with_message("Publishing release… 82%")
                                .with_can_cancel(true)
                                .with_size_role(SemanticControlSizeRole::Prominent),
                            theme,
                        )),
                ),
        )
}
