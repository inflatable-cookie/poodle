use gpui::*;
use poodle_composites::EmbedPreviewSpec;
use poodle_gpui_components::{EmbedPreview, Eyebrow};
use poodle_primitives::EyebrowSpec;
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // -- Default --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            EmbedPreview::from_spec(
                                EmbedPreviewSpec::new()
                                    .with_title("Rick Astley - Never Gonna Give You Up")
                                    .with_description("The official video for Rick Astley's classic 1987 hit.")
                                    .with_provider("YouTube"),
                                theme,
                            )
                        )
                        .child(
                            EmbedPreview::from_spec(
                                EmbedPreviewSpec::new()
                                    .with_loading(true),
                                theme,
                            )
                        )
                )
        )
}
