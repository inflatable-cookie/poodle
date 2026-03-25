use gpui::*;
use poodle_composites::PageLoadingSpec;
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{PageLoading, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Indeterminate ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Indeterminate"), theme))
                .child(
                    PageLoading::from_spec(
                        PageLoadingSpec::new()
                            .with_message("Loading components..."),
                        theme,
                    )
                )
        )
        // --- Determinate (60%) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Determinate (60%)"), theme))
                .child(
                    PageLoading::from_spec(
                        PageLoadingSpec::new()
                            .with_value(60.0)
                            .with_max(100.0)
                            .with_message("Importing data...")
                            .with_can_cancel(true),
                        theme,
                    )
                )
        )
}
