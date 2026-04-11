use gpui::*;
use poodle_specs::{PageLoadingPresentation, PageLoadingSpec};
use poodle_specs::EyebrowSpec;
use poodle_gpui_components::{PageLoading, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Indeterminate (spinner only) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Indeterminate (spinner only)"), theme))
                .child(
                    PageLoading::from_spec(
                        PageLoadingSpec::new()
                            .with_message("Loading data..."),
                        theme,
                    )
                )
        )
        // --- Determinate (with progress bar) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Determinate (with progress bar)"), theme))
                .child(
                    PageLoading::from_spec(
                        PageLoadingSpec::new()
                            .with_value(60.0)
                            .with_max(100.0)
                            .with_message("Uploading files... 60%"),
                        theme,
                    )
                )
        )
        // --- With cancel button ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With cancel button"), theme))
                .child(
                    PageLoading::from_spec(
                        PageLoadingSpec::new()
                            .with_message("Processing request...")
                            .with_can_cancel(true),
                        theme,
                    )
                )
        )
        // --- Determinate with cancel ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Determinate with cancel"), theme))
                .child(
                    PageLoading::from_spec(
                        PageLoadingSpec::new()
                            .with_value(35.0)
                            .with_max(100.0)
                            .with_message("Importing data... 35%")
                            .with_can_cancel(true),
                        theme,
                    )
                )
        )
        // --- Inline presentation (in-flow, no backdrop) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Inline presentation (no backdrop)"), theme))
                .child(
                    PageLoading::from_spec(
                        PageLoadingSpec::new()
                            .with_message("Loading this section\u{2026}")
                            .with_presentation(PageLoadingPresentation::Inline),
                        theme,
                    )
                )
        )
}
